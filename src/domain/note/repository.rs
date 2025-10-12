use sqlx::PgPool;
use uuid::Uuid;

use super::model::{Note, NoteRow};
use crate::domain::error::AppError;
use crate::impl_crud_repository;

// Implement repository for Note using our macro
impl_crud_repository!(
    NoteRepository,
    Note,
    NoteRow,
    "notes",
    [title, content, user_id, created]
);

impl NoteRepository {
    pub async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Note>, AppError> {
        let notes = sqlx::query_as!(
            NoteRow,
            "SELECT * FROM notes WHERE user_id = $1 ORDER BY created DESC",
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(notes.into_iter().map(|n| n.into()).collect())
    }
}
