use axum::extract::{Extension, Path, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::model::Note;
use super::service::{CreateNoteInput, UpdateNoteInput};
use crate::authorization::can_access_resource;
use crate::domain::error::ToResponse;
use crate::domain::response::Response;
use crate::domain::user::model::User;
use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct CreateNoteRequest {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateNoteRequest {
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct NoteResponse {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub content: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
}

impl From<Note> for NoteResponse {
    fn from(note: Note) -> Self {
        Self {
            id: note.id.to_string(),
            user_id: note.user_id.to_string(),
            title: note.title,
            content: note.content,
            created: note.created,
            updated: note.updated,
        }
    }
}

// GET /notes - Get all notes (admins see all, users see only their own)
pub async fn get_all_notes(
    State(state): State<AppState>,
    Extension(authenticated_user): Extension<User>,
) -> Response<Vec<NoteResponse>> {
    state
        .note_service
        .get_notes_by_user(&authenticated_user)
        .await
        .map(|notes| notes.into_iter().map(NoteResponse::from).collect())
        .to_response("Notes retrieved successfully")
}

// GET /notes/:id - Get a specific note
pub async fn get_note(
    State(state): State<AppState>,
    Extension(authenticated_user): Extension<User>,
    Path(id): Path<Uuid>,
) -> Response<NoteResponse> {
    // First, fetch the note
    let note = match state.note_service.get_by_id(id).await {
        Ok(note) => note,
        Err(e) => {
            return Response::failure_not_found(&e.to_string(), Some(e.to_string()));
        }
    };

    // Check authorization
    if !can_access_resource(&authenticated_user, &note) {
        return Response::failure_forbidden(
            "You don't have permission to access this note",
            Some("FORBIDDEN".to_string()),
        );
    }

    Response::success_ok(NoteResponse::from(note), "Note retrieved successfully")
}

// POST /notes - Create a new note
pub async fn create_note(
    State(state): State<AppState>,
    Extension(authenticated_user): Extension<User>,
    Json(req): Json<CreateNoteRequest>,
) -> Response<NoteResponse> {
    // User can only create notes for themselves
    let input = CreateNoteInput {
        user_id: authenticated_user.id,
        title: req.title,
        content: req.content,
    };

    state
        .note_service
        .create(input)
        .await
        .map(NoteResponse::from)
        .to_response_created("Note created successfully")
}

// PUT /notes/:id - Update a note
pub async fn update_note(
    State(state): State<AppState>,
    Extension(authenticated_user): Extension<User>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateNoteRequest>,
) -> Response<NoteResponse> {
    // First, fetch the note
    let note = match state.note_service.get_by_id(id).await {
        Ok(note) => note,
        Err(e) => {
            return Response::failure_not_found(&e.to_string(), Some(e.to_string()));
        }
    };

    // Check authorization
    if !can_access_resource(&authenticated_user, &note) {
        return Response::failure_forbidden(
            "You don't have permission to update this note",
            Some("FORBIDDEN".to_string()),
        );
    }

    // Perform the update
    let input = UpdateNoteInput {
        id,
        user_id: None, // Never allow changing the owner
        title: req.title,
        content: req.content,
    };

    state
        .note_service
        .update(input)
        .await
        .map(NoteResponse::from)
        .to_response("Note updated successfully")
}

// DELETE /notes/:id - Delete a note
pub async fn delete_note(
    State(state): State<AppState>,
    Extension(authenticated_user): Extension<User>,
    Path(id): Path<Uuid>,
) -> Response<serde_json::Value> {
    // First, fetch the note
    let note = match state.note_service.get_by_id(id).await {
        Ok(note) => note,
        Err(e) => {
            return Response::failure_not_found(&e.to_string(), Some(e.to_string()));
        }
    };

    // Check authorization
    if !can_access_resource(&authenticated_user, &note) {
        return Response::failure_forbidden(
            "You don't have permission to delete this note",
            Some("FORBIDDEN".to_string()),
        );
    }

    // Perform the deletion
    state
        .note_service
        .delete(id)
        .await
        .to_response_no_content("Note deleted successfully")
}
