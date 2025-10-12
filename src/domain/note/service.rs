use uuid::Uuid;

// Import note model
use super::model::Note;

// Import note repository
use super::repository::NoteRepository;

// Import error handling
use crate::domain::error::AppError;

// NoteService struct
#[derive(Clone)]
pub struct NoteService {
    repository: NoteRepository,
}

// Implementation of NoteService
impl NoteService {
    pub fn new(repository: NoteRepository) -> Self {
        Self { repository }
    }

    pub async fn create_note(
        &self,
        title: String,
        content: String,
        user_id: Uuid,
    ) -> Result<Note, AppError> {
        // Validate input
        if title.trim().is_empty() {
            return Err(AppError::missing_field("title"));
        }

        if content.trim().is_empty() {
            return Err(AppError::missing_field("content"));
        }

        let note = Note::new(title, content, user_id);
        self.repository.create(&note).await
    }

    pub async fn get_note_by_id(&self, id: Uuid) -> Result<Note, AppError> {
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Note with id {} not found", id)))
    }

    pub async fn get_notes_by_user_id(&self, user_id: Uuid) -> Result<Vec<Note>, AppError> {
        self.repository.find_by_user_id(user_id).await
    }

    pub async fn get_all_notes(&self) -> Result<Vec<Note>, AppError> {
        self.repository.find_all().await
    }

    pub async fn update_note(
        &self,
        id: Uuid,
        title: Option<String>,
        content: Option<String>,
    ) -> Result<Note, AppError> {
        let mut note = self.get_note_by_id(id).await?;

        if let Some(title) = title {
            if title.trim().is_empty() {
                return Err(AppError::missing_field("title"));
            }
            note.title = title;
        }

        if let Some(content) = content {
            if content.trim().is_empty() {
                return Err(AppError::missing_field("content"));
            }
            note.content = content;
        }

        note.updated = chrono::Utc::now();
        self.repository.update(&note).await
    }

    pub async fn delete_note(&self, id: Uuid) -> Result<bool, AppError> {
        // Check if note exists
        self.get_note_by_id(id).await?;
        
        // Delete note
        self.repository.delete(id).await
    }
}