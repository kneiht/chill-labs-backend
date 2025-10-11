use super::model::{AuthResponse, LoginRequest, RegisterRequest, UserInfo};
use crate::domain::response::Response;
use crate::domain::user::model::Role;
use crate::state::AppState;
use crate::utils::jwt::JwtUtil;
use crate::utils::password::hash_password;
use axum::Json;

pub async fn login(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Response<AuthResponse> {
    let user_service = state.user_service.clone();

    match user_service
        .authenticate_user(&req.email, &req.password)
        .await
    {
        Ok(user) => {
            let jwt_util = JwtUtil::new(
                &state
                    .settings
                    .jwt
                    .secret
                    .as_ref()
                    .unwrap_or(&"default_secret".to_string()),
                state.settings.jwt.expiration_hours.unwrap_or(24),
            );

            match jwt_util.generate_token(user.id, &user.email) {
                Ok(token) => {
                    let user_info = UserInfo {
                        id: user.id,
                        display_name: user.display_name,
                        email: user.email,
                        role: format!("{:?}", user.role),
                        status: format!("{:?}", user.status),
                    };

                    Response::success_ok(
                        AuthResponse {
                            token,
                            user: user_info,
                        },
                        "Login successful",
                    )
                }
                Err(e) => {
                    Response::failure_internal("Failed to generate token", Some(e.to_string()))
                }
            }
        }
        Err(e) => Response::failure_unauthorized("Invalid credentials", Some(e.to_string())),
    }
}

pub async fn register(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Response<AuthResponse> {
    let user_service = state.user_service.clone();

    // Hash the password
    let password_hash = match hash_password(&req.password) {
        Ok(hash) => hash,
        Err(e) => {
            return Response::failure_internal("Failed to hash password", Some(e.to_string()))
        }
    };

    // All new users default to Student role
    let role = Role::Student;

    match user_service
        .create_user_with_password(req.display_name, req.email, password_hash, role)
        .await
    {
        Ok(user) => {
            let jwt_util = JwtUtil::new(
                &state
                    .settings
                    .jwt
                    .secret
                    .as_ref()
                    .unwrap_or(&"default_secret".to_string()),
                state.settings.jwt.expiration_hours.unwrap_or(24),
            );

            match jwt_util.generate_token(user.id, &user.email) {
                Ok(token) => {
                    let user_info = UserInfo {
                        id: user.id,
                        display_name: user.display_name,
                        email: user.email,
                        role: format!("{:?}", user.role),
                        status: format!("{:?}", user.status),
                    };

                    Response::success_created(
                        AuthResponse {
                            token,
                            user: user_info,
                        },
                        "User registered successfully",
                    )
                }
                Err(e) => {
                    Response::failure_internal("Failed to generate token", Some(e.to_string()))
                }
            }
        }
        Err(e) => Response::failure_conflict("Failed to register user", Some(e.to_string())),
    }
}
