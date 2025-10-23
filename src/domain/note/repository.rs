use super::model::{Note, NoteRow};
use crate::crud_repository;
use uuid::Uuid;

crud_repository!(
  NoteRepository,
  Note,
  NoteRow,
  "notes",
  id, user_id, title, content, created, updated;
  id, user_id, title, content, created, updated;
  user_id, title, content, updated;
);

// Additional methods for NoteRepository
impl NoteRepository {
    /// Find all notes for a specific user
    pub async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Note>, sqlx::Error> {
        let rows = sqlx::query_as::<_, NoteRow>(
            "SELECT id, user_id, title, content, created, updated 
             FROM notes 
             WHERE user_id = $1 
             ORDER BY updated DESC",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Note::from).collect())
    }
}
