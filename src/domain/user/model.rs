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
    pub password_hash: String,
    pub role: Role,
    pub status: UserStatus,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
}

impl User {
    pub fn new(display_name: String, email: String, password_hash: String, role: Role) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::now_v7(),
            display_name,
            email,
            password_hash,
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

// Internal struct for database queries
#[derive(sqlx::FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub display_name: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub status: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        Self {
            id: row.id,
            display_name: row.display_name,
            email: row.email,
            password_hash: row.password_hash,
            role: match row.role.as_str() {
                "Student" => Role::Student,
                "Teacher" => Role::Teacher,
                "Admin" => Role::Admin,
                _ => panic!("Invalid role: {}", row.role),
            },
            status: match row.status.as_str() {
                "Active" => UserStatus::Active,
                "Pending" => UserStatus::Pending,
                "Suspended" => UserStatus::Suspended,
                _ => panic!("Invalid status: {}", row.status),
            },
            created: row.created,
            updated: row.updated,
        }
    }
}
