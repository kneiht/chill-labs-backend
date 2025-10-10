 use serde::{Deserialize, Serialize};
 use uuid::Uuid;

 #[derive(Debug, Deserialize)]
 pub struct LoginRequest {
     pub email: String,
     pub password: String,
 }

 #[derive(Debug, Deserialize)]
 pub struct RegisterRequest {
     pub display_name: String,
     pub email: String,
     pub password: String,
 }

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub display_name: String,
    pub email: String,
    pub role: String,
    pub status: String,
}
