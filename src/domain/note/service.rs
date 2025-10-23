use super::model::Note;
use super::repository::NoteRepository;
use crate::authorization::get_ownership_filter;
use crate::domain::user::model::User;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Validate)]
pub struct CreateNoteInput {
    pub user_id: Uuid,
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    pub title: String,
    #[validate(length(min = 1, message = "Content cannot be empty"))]
    pub content: String,
}

#[derive(Debug, Clone, Validate)]
pub struct UpdateNoteInput {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    pub title: Option<String>,
    #[validate(length(min = 1, message = "Content cannot be empty"))]
    pub content: Option<String>,
}

crate::crud_service!(
    NoteService,
    Note,
    NoteRepository,
    CreateNoteInput,
    UpdateNoteInput,
    "Note",
    create_logic: |input| {
        Note::new(input.user_id, input.title, input.content)
    },
    update_logic: |model, input| {
        if let Some(user_id) = input.user_id {
            model.user_id = user_id;
        }
        if let Some(title) = input.title {
            model.title = title;
        }
        if let Some(content) = input.content {
            model.content = content;
        }
    }
);

// Additional methods for NoteService
impl NoteService {
    /// Get notes filtered by user role:
    /// - Admins see all notes
    /// - Regular users see only their own notes
    pub async fn get_notes_by_user(
        &self,
        user: &User,
    ) -> Result<Vec<Note>, crate::domain::error::AppError> {
        match get_ownership_filter(user) {
            None => {
                // Admin - get all notes
                self.repository
                    .find_all()
                    .await
                    .map_err(crate::domain::error::AppError::from)
            }
            Some(user_id) => {
                // Regular user - get only their notes
                self.repository
                    .find_by_user_id(user_id)
                    .await
                    .map_err(crate::domain::error::AppError::from)
            }
        }
    }
}
