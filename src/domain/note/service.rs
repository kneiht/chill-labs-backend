use super::model::Note;
use super::repository::NoteRepository;
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
