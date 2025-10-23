use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::authorization::OwnedResource;

// Note struct
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Note {
    pub id: Uuid,
    pub user_id: Uuid,
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    pub title: String,
    #[validate(length(min = 1, message = "Content cannot be empty"))]
    pub content: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
}

// Implementation of OwnedResource for Note
impl OwnedResource for Note {
    fn owner_id(&self) -> Uuid {
        self.user_id
    }
}

// Implementation of Note
impl Note {
    pub fn new(user_id: Uuid, title: String, content: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::now_v7(),
            user_id,
            title,
            content,
            created: now,
            updated: now,
        }
    }
}

// Internal struct for database queries
#[derive(sqlx::FromRow)]
pub struct NoteRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
}

// Implementation of From<NoteRow> for Note
impl From<NoteRow> for Note {
    fn from(row: NoteRow) -> Self {
        Self {
            id: row.id,
            user_id: row.user_id,
            title: row.title,
            content: row.content,
            created: row.created,
            updated: row.updated,
        }
    }
}
