use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

fn id_generator() -> Uuid {
    Uuid::now_v7()
}

// Base entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseEntity {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl BaseEntity {
    pub fn new() -> Self {
        Self {
            id: id_generator(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
