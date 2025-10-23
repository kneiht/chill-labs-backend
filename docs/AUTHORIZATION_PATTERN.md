# Trait-Based Authorization Pattern

## Overview

This project uses a trait-based authorization system for resource-level access control. The pattern centralizes authorization logic while being flexible and type-safe.

## Core Components

### 1. The `OwnedResource` Trait (`src/authorization.rs`)

```rust
pub trait OwnedResource {
    fn owner_id(&self) -> Uuid;
}
```

Any model that has an owner (user_id) implements this trait.

### 2. Authorization Functions

```rust
// Check if user can access a resource (admin OR owner)
pub fn can_access_resource<T: OwnedResource>(
    authenticated_user: &User,
    resource: &T,
) -> bool

// Get filter for "get all" queries (None for admin, Some(user_id) for others)
pub fn get_ownership_filter(authenticated_user: &User) -> Option<Uuid>

// Check if user is admin
pub fn is_admin(user: &User) -> bool

// Require admin role
pub fn require_admin(user: &User) -> Result<(), String>
```

## Implementation Guide

### Step 1: Implement `OwnedResource` for Your Model

**For User (owner is self):**
```rust
impl OwnedResource for User {
    fn owner_id(&self) -> Uuid {
        self.id
    }
}
```

**For Note (owner is user_id field):**
```rust
impl OwnedResource for Note {
    fn owner_id(&self) -> Uuid {
        self.user_id
    }
}
```

### Step 2: Use Authorization in Handlers

#### Pattern for GET (single resource):
```rust
pub async fn get_note(
    State(state): State<AppState>,
    Extension(authenticated_user): Extension<User>,
    Path(id): Path<Uuid>,
) -> Response<NoteResponse> {
    let note_service = state.note_service.clone();

    // Fetch the resource first
    let note = match note_service.get_note_by_id(id).await {
        Ok(note) => note,
        Err(e) => {
            return Response::failure_not_found(
                "Note not found",
                Some(e.to_string()),
            )
        }
    };

    // Check authorization using the trait
    if !can_access_resource(&authenticated_user, &note) {
        return Response::failure_forbidden(
            "Access denied",
            Some("You can only access your own notes".to_string()),
        );
    }

    Response::success_ok(note.into(), "Note retrieved successfully")
}
```

#### Pattern for GET ALL (list resources):
```rust
pub async fn get_all_notes(
    State(state): State<AppState>,
    Extension(authenticated_user): Extension<User>,
) -> Response<Vec<NoteResponse>> {
    let note_service = state.note_service.clone();

    // Get ownership filter
    let filter = get_ownership_filter(&authenticated_user);

    // Query based on filter
    let notes = match filter {
        None => note_service.get_all_notes().await, // Admin sees all
        Some(user_id) => note_service.get_notes_by_user(user_id).await, // User sees own
    };

    notes
        .map(|notes| notes.into_iter().map(Into::into).collect())
        .to_response("Notes retrieved successfully")
}
```

#### Pattern for CREATE:
```rust
pub async fn create_note(
    State(state): State<AppState>,
    Extension(authenticated_user): Extension<User>,
    Json(req): Json<CreateNoteRequest>,
) -> Response<NoteResponse> {
    let note_service = state.note_service.clone();

    // Enforce ownership - user can only create notes for themselves
    // (unless admin, who can create for anyone)
    let user_id = if is_admin(&authenticated_user) {
        req.user_id.unwrap_or(authenticated_user.id)
    } else {
        authenticated_user.id // Force user's own ID
    };

    let create_input = CreateNoteInput {
        user_id,
        title: req.title,
        content: req.content,
    };

    note_service
        .create_note(create_input)
        .await
        .map(|note| note.into())
        .to_response_created("Note created successfully")
}
```

#### Pattern for UPDATE:
```rust
pub async fn update_note(
    State(state): State<AppState>,
    Extension(authenticated_user): Extension<User>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateNoteRequest>,
) -> Response<NoteResponse> {
    let note_service = state.note_service.clone();

    // Fetch the resource first
    let note = match note_service.get_note_by_id(id).await {
        Ok(note) => note,
        Err(e) => {
            return Response::failure_not_found(
                "Note not found",
                Some(e.to_string()),
            )
        }
    };

    // Check authorization
    if !can_access_resource(&authenticated_user, &note) {
        return Response::failure_forbidden(
            "Access denied",
            Some("You can only update your own notes".to_string()),
        );
    }

    // Prevent non-admin from changing ownership
    if !is_admin(&authenticated_user) && req.user_id.is_some() {
        return Response::failure_forbidden(
            "Access denied",
            Some("Only administrators can change note ownership".to_string()),
        );
    }

    let update_input = UpdateNoteInput {
        id,
        user_id: req.user_id,
        title: req.title,
        content: req.content,
    };

    note_service
        .update_note(update_input)
        .await
        .map(|note| note.into())
        .to_response("Note updated successfully")
}
```

#### Pattern for DELETE:
```rust
pub async fn delete_note(
    State(state): State<AppState>,
    Extension(authenticated_user): Extension<User>,
    Path(id): Path<Uuid>,
) -> Response<serde_json::Value> {
    let note_service = state.note_service.clone();

    // Fetch the resource first
    let note = match note_service.get_note_by_id(id).await {
        Ok(note) => note,
        Err(e) => {
            return Response::failure_not_found(
                "Note not found",
                Some(e.to_string()),
            )
        }
    };

    // Check authorization
    if !can_access_resource(&authenticated_user, &note) {
        return Response::failure_forbidden(
            "Access denied",
            Some("You can only delete your own notes".to_string()),
        );
    }

    note_service
        .delete_note(id)
        .await
        .to_response_no_content("Note deleted successfully")
}
```

## Service Layer Requirements

For owned resources, add user-filtered methods to your service:

```rust
impl NoteService {
    // Macro-generated (for admin)
    pub async fn get_all_notes(&self) -> Result<Vec<Note>, AppError>
    
    // Manual method (for regular users)
    pub async fn get_notes_by_user(&self, user_id: Uuid) -> Result<Vec<Note>, AppError> {
        self.repository.find_by_user_id(user_id).await
    }
}
```

## Repository Layer Requirements

Add user-filtered queries:

```rust
impl NoteRepository {
    // Macro-generated
    pub async fn find_all(&self) -> Result<Vec<Note>, AppError>
    
    // Manual method
    pub async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Note>, AppError> {
        sqlx::query_as!(
            NoteRow,
            "SELECT * FROM notes WHERE user_id = $1 ORDER BY created DESC",
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map(|rows| rows.into_iter().map(Note::from).collect())
        .map_err(AppError::from)
    }
}
```

## Architecture Summary

| Layer | Strategy |
|-------|----------|
| **Repository** | Macro-generated methods + manual user-filtered queries |
| **Service** | Macro-generated methods + manual user-filtered methods |
| **Handler** | **Manual handlers** with authorization checks for owned resources |
| **Model** | Implement `OwnedResource` trait |
| **Router** | All protected routes use `auth_middleware` |

## Benefits

✅ **DRY** - Authorization logic centralized in one place  
✅ **Type-safe** - Trait ensures all resources have consistent interface  
✅ **Flexible** - Easy to add complex authorization rules  
✅ **Testable** - Authorization logic has unit tests  
✅ **Maintainable** - Clear pattern to follow for new domains  
✅ **Professional** - Industry-standard approach used by major frameworks

## Examples in Codebase

- **User domain**: `src/domain/user/handler.rs` - Shows full implementation
- **Note domain**: Ready to implement (has `OwnedResource` trait)
- **Authorization module**: `src/authorization.rs` - Core logic with tests

## Next Steps for New Domains

1. Implement `OwnedResource` for your model
2. Add user-filtered repository methods
3. Add user-filtered service methods
4. Write manual handlers with authorization checks
5. Test your authorization logic