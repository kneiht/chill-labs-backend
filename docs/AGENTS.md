# AGENTS.md - Development Guidelines

## Commands
- **Build**: `cargo build` or `cargo build --release`
- **Check**: `cargo check` (fast compilation check)
- **Test**: `cargo test` (all tests) or `cargo test <test_name>` (single test)
- **Run**: `cargo run` or `cargo run --bin scratch` for scratch binary

## Code Style Guidelines

### Imports & Structure
- Use `use crate::` for internal modules
- Group imports: std libs, external crates, internal modules
- Prefer `use super::*` for sibling module access

### Naming Conventions
- **Modules**: snake_case (e.g., `auth_handler.rs`)
- **Functions/Variables**: snake_case
- **Types/Structs**: PascalCase
- **Constants**: SCREAMING_SNAKE_CASE

### Error Handling
- Use custom `AppError` enum from `domain::error`
- Implement `ToResponse` trait for handler results
- Convert external errors using `From` traits
- Use `?` operator for error propagation

### Domain Architecture
- **Models**: Data structures with validation (`validator` derive)
- **Handlers**: HTTP request/response handlers
- **Services**: Business logic layer
- **Repositories**: Data access layer
- Use `Transformer` trait for input validation

### Developing a New Domain
Follow this 4-step pattern for each new domain (e.g., `note`, `post`, `comment`):

1. **Create domain structure** in `src/domain/{domain_name}/mod.rs`:
   ```rust
   pub mod handler;
   pub mod model;
   pub mod repository;
   pub mod service;
   
   use crate::state::AppState;
   use axum::routing::{get, post, put, delete};
   use axum::Router;
   
   pub fn {domain}_routes() -> Router<AppState> {
       Router::new()
           .route("/", post(create_{domain}).get(get_all_{domain}s))
           .route("/{id}", get(get_{domain}).put(update_{domain}).delete(delete_{domain}))
   }
   ```

2. **Define model** in `model.rs`:
   - Main struct with `id: Uuid`, `user_id: Uuid`, validation attributes
   - Implement `OwnedResource` trait for authorization
   - Add `new()` constructor with `Uuid::now_v7()` and timestamps
   - Create `RowType` struct with `sqlx::FromRow` derive
   - Implement `From<RowType>` conversion

3. **Generate repository** in `repository.rs` using macro:
   ```rust
   crud_repository!(
       {Domain}Repository,
       {Domain},
       {Domain}Row,
       "{domain}s",
       id, user_id, field1, field2, created, updated;  // INSERT
       id, user_id, field1, field2, created, updated;  // SELECT
       user_id, field1, field2, updated;               // UPDATE
   );
   ```

4. **Generate service** in `service.rs` using macro:
   ```rust
   crud_service!(
       {Domain}Service,
       {Domain},
       {Domain}Repository,
       Create{Domain}Input,
       Update{Domain}Input,
       "{Domain}",
       create_logic: |input| { {Domain}::new(input.user_id, input.field1, input.field2) },
       update_logic: |model, input| {
           if let Some(field) = input.field1 { model.field1 = field; }
       }
   );
   ```

5. **Implement handlers** in `handler.rs`:
   - Request/response DTOs with validation
   - Authorization checks using `can_access_resource()`
   - Use `ToResponse` trait for consistent responses
   - Follow REST patterns: GET/POST/PUT/DELETE

### Database
- Use SQLx with PostgreSQL
- UUID v7 for primary keys
- Async/await throughout
- Connection pooling via `AppState`

### Authentication/Authorization
- JWT tokens with rotating refresh strategy
- Role-based access control (Admin/User)
- Use `OwnedResource` trait for resource ownership
- Middleware for auth checks

### Response Format
- Use `Response<T>` wrapper from `domain::response`
- Consistent success/error structure
- HTTP status codes via `to_response()` methods

### Testing
- Unit tests in `#[cfg(test)]` modules
- Integration tests for handlers
- Use `cargo test <name>` for specific tests