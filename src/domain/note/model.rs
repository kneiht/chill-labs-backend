use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub user_id: Uuid,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
}

impl Note {
    pub fn new(title: String, content: String, user_id: Uuid) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::now_v7(),
            title,
            content,
            user_id,
            created: now,
            updated: now,
        }
    }
}

#[derive(sqlx::FromRow)]
pub struct NoteRow {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub user_id: Uuid,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
}

impl From<NoteRow> for Note {
    fn from(row: NoteRow) -> Self {
        Self {
            id: row.id,
            title: row.title,
            content: row.content,
            user_id: row.user_id,
            created: row.created,
            updated: row.updated,
        }
    }
}