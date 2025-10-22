use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// Role enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Role {
    Student,
    Teacher,
    Admin,
}

// UserStatus enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserStatus {
    Active,
    Pending,
    Suspended,
}

// User struct
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct User {
    pub id: Uuid,
    #[validate(length(min = 1, message = "Display name cannot be empty"))]
    pub display_name: Option<String>,
    #[validate(length(min = 1, message = "Username cannot be empty when provided"))]
    pub username: Option<String>,
    pub email: Option<String>,
    #[validate(length(min = 8, message = "Password hash must be at least 8 characters"))]
    pub password_hash: String,
    pub role: Role,
    pub status: UserStatus,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
}

// Validation error for User
#[derive(Debug, Clone)]
pub struct UserValidationError {
    pub message: String,
}

impl std::fmt::Display for UserValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for UserValidationError {}

// Implementation of User
impl User {
    pub fn new(
        display_name: Option<String>,
        username: Option<String>,
        email: Option<String>,
        password_hash: String,
        role: Role,
    ) -> Self {
        let now = chrono::Utc::now();

        // TODO: Validate that at least one of username or email is provided

        // Return a new instance of User
        Self {
            id: Uuid::now_v7(),
            display_name,
            username,
            email,
            password_hash,
            role,
            status: UserStatus::Pending,
            created: now,
            updated: now,
        }
    }
}

// Internal struct for database queries
#[derive(sqlx::FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub display_name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password_hash: String,
    pub role: String,
    pub status: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
}

// Implementation of From<UserRow> for User
impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        Self {
            id: row.id,
            display_name: row.display_name,
            username: row.username,
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
