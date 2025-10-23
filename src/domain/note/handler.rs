use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::model::Note;
use super::service::{CreateNoteInput, UpdateNoteInput};

#[derive(Debug, Deserialize)]
pub struct CreateNoteRequest {
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateNoteRequest {
    pub user_id: Option<Uuid>,
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

crate::crud_handlers!(
    CreateNoteRequest,
    UpdateNoteRequest,
    NoteResponse,
    CreateNoteInput,
    UpdateNoteInput,
    note_service,
    "Note",
    request_to_create: |req| {
        CreateNoteInput {
            user_id: req.user_id,
            title: req.title,
            content: req.content,
        }
    },
    request_to_update: |id, req| {
        UpdateNoteInput {
            id,
            user_id: req.user_id,
            title: req.title,
            content: req.content,
        }
    }
);