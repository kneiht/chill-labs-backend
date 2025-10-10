use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Role {
    Student,
    Teacher,
    Admin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub display_name: String,
    pub email: String,
    pub role: Role,
    pub status: UserStatus,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
}

impl User {
    pub fn new(display_name: String, email: String, role: Role) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::now_v7(),
            display_name,
            email,
            role,
            status: UserStatus::Pending,
            created: now,
            updated: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserStatus {
    Active,
    Pending,
    Suspended,
}
