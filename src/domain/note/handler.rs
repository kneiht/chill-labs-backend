use super::model::Note;
use crate::domain::error::ToResponse;
use crate::domain::response::Response;
use crate::state::AppState;
use axum::extract::{Path, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateNoteRequest {
    pub title: String,
    pub content: String,
    pub user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateNoteRequest {
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct NoteResponse {
    pub id: String,
    pub title: String,
    pub content: String,
    pub user_id: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
}

impl From<Note> for NoteResponse {
    fn from(note: Note) -> Self {
        Self {
            id: note.id.to_string(),
            title: note.title,
            content: note.content,
            user_id: note.user_id.to_string(),
            created: note.created,
            updated: note.updated,
        }
    }
}

pub async fn create_note(
    State(state): State<AppState>,
    Json(request): Json<CreateNoteRequest>,
) -> Response<NoteResponse> {
    let user_id = match Uuid::parse_str(&request.user_id) {
        Ok(id) => id,
        Err(_) => {
            return Response::failure_validation(
                "Invalid user ID format",
                Some("USER_ID_INVALID".to_string()),
            );
        }
    };

    state
        .note_service
        .create_note(request.title, request.content, user_id)
        .await
        .map(|note| note.into())
        .to_response_created("Note created successfully")
}

pub async fn get_note_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Response<NoteResponse> {
    let note_id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => {
            return Response::failure_validation(
                "Invalid note ID format",
                Some("NOTE_ID_INVALID".to_string()),
            );
        }
    };

    state
        .note_service
        .get_note_by_id(note_id)
        .await
        .map(|note| note.into())
        .to_response("Note retrieved successfully")
}

pub async fn get_notes_by_user_id(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Response<Vec<NoteResponse>> {
    let user_id = match Uuid::parse_str(&user_id) {
        Ok(id) => id,
        Err(_) => {
            return Response::failure_validation(
                "Invalid user ID format",
                Some("USER_ID_INVALID".to_string()),
            );
        }
    };

    state
        .note_service
        .get_notes_by_user_id(user_id)
        .await
        .map(|notes| notes.into_iter().map(Into::into).collect())
        .to_response("Notes retrieved successfully")
}

pub async fn get_all_notes(State(state): State<AppState>) -> Response<Vec<NoteResponse>> {
    state
        .note_service
        .get_all_notes()
        .await
        .map(|notes| notes.into_iter().map(Into::into).collect())
        .to_response("All notes retrieved successfully")
}

pub async fn update_note(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<UpdateNoteRequest>,
) -> Response<NoteResponse> {
    let note_id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => {
            return Response::failure_validation(
                "Invalid note ID format",
                Some("NOTE_ID_INVALID".to_string()),
            );
        }
    };

    state
        .note_service
        .update_note(note_id, request.title, request.content)
        .await
        .map(|note| note.into())
        .to_response("Note updated successfully")
}

pub async fn delete_note(State(state): State<AppState>, Path(id): Path<String>) -> Response<bool> {
    let note_id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => {
            return Response::failure_validation(
                "Invalid note ID format",
                Some("NOTE_ID_INVALID".to_string()),
            );
        }
    };

    state
        .note_service
        .delete_note(note_id)
        .await
        .to_response("Note deleted successfully")
}
