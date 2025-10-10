use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use sqlx::Type;
use uuid::Uuid;

// Define an enum for user status
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Type)]
#[sqlx(type_name = "user_status", rename_all = "lowercase")]
pub enum UserStatus {
    Active,
    Pending,   // e.g., email verification needed
    Suspended, // e.g., banned by admin
}

// Define an enum for user roles
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Student,
    Teacher,
    Admin,
}

// Define an enum for user membership status
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Type)]
#[sqlx(type_name = "membership_type", rename_all = "lowercase")] // Ensure this matches your DB enum name
pub enum Membership {
    Free,
    Premium,
    Trial, // Add other relevant membership types
}

// Define an enum for user gender
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Type)]
#[sqlx(type_name = "gender_type", rename_all = "lowercase")] // Ensure this matches your DB enum name
pub enum Gender {
    Male,
    Female,
    Other,
}

// Define the User model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub display_name: String,
    pub email: String,
    pub email_verified: bool,
    pub password_hash: String,
    pub status: UserStatus,
    pub role: UserRole,
    pub avatar_url: Option<String>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub membership: Membership,
    pub gender: Gender,
    pub date_of_birth: Option<NaiveDate>,
    pub phone: Option<String>,
    pub bio: Option<String>,
}

// Define the EmailVerificationToken model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailVerificationToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub created: DateTime<Utc>,
}
