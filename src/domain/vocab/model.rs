use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::authorization::OwnedResource;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Word {
    #[validate(length(min = 1, message = "Word cannot be empty"))]
    pub word: String,
    pub phonics: Option<String>,
    pub part_of_speech: Option<String>,
    #[validate(length(min = 1, message = "Vietnamese meaning cannot be empty"))]
    pub vietnamese_meaning: String,
    pub sample_sentence: Option<String>,
    pub vietnamese_translation: Option<String>,
    pub image: Option<String>,
    pub word_pronunciation: Option<String>,
    pub sentence_pronunciation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Vocab {
    pub id: Uuid,
    pub user_id: Uuid,
    pub words: Vec<Word>,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
}

impl OwnedResource for Vocab {
    fn owner_id(&self) -> Uuid {
        self.user_id
    }
}

impl Vocab {
    pub fn new(user_id: Uuid, words: Vec<Word>) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::now_v7(),
            user_id,
            words,
            created: now,
            updated: now,
        }
    }
}

#[derive(sqlx::FromRow)]
pub struct VocabRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub words: serde_json::Value,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
}

impl From<VocabRow> for Vocab {
    fn from(row: VocabRow) -> Self {
        let words: Vec<Word> = serde_json::from_value(row.words).unwrap_or_default();
        Self {
            id: row.id,
            user_id: row.user_id,
            words,
            created: row.created,
            updated: row.updated,
        }
    }
}

impl From<Vocab> for VocabRow {
    fn from(vocab: Vocab) -> Self {
        Self {
            id: vocab.id,
            user_id: vocab.user_id,
            words: serde_json::to_value(vocab.words).unwrap_or(serde_json::Value::Null),
            created: vocab.created,
            updated: vocab.updated,
        }
    }
}

// Custom implementation for JSON binding
impl Vocab {
    pub fn to_json_value(&self) -> Result<serde_json::Value, crate::domain::error::AppError> {
        serde_json::to_value(&self.words)
            .map_err(|e| crate::domain::error::AppError::Internal(format!("Failed to serialize words: {}", e)))
    }
}