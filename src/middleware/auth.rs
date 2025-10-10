use crate::state::AppState;
use crate::utils::jwt::JwtUtil;
use axum::{
    extract::{FromRequestParts, Request},
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
    middleware::Next,
    response::Response,
};

pub struct AuthUser {
    pub user_id: uuid::Uuid,
    pub email: String,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    AppState: FromRequestParts<S>,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the authorization header first
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|header| header.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Missing authorization header"))?;

        // Check if it's a Bearer token
        let token = if auth_header.starts_with("Bearer ") {
            auth_header[7..].to_string()
        } else {
            return Err((
                StatusCode::UNAUTHORIZED,
                "Invalid authorization header format",
            ));
        };

        // Get JWT util from state
        let state = AppState::from_request_parts(parts, state)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get app state"))?;

        let jwt_util = JwtUtil::new(
            &state
                .settings
                .jwt
                .secret
                .as_ref()
                .unwrap_or(&"default_secret".to_string()),
            state.settings.jwt.expiration_hours.unwrap_or(24),
        );

        // Verify token
        let claims = jwt_util
            .verify_token(&token)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token"))?;

        let user_id = claims
            .sub
            .parse::<uuid::Uuid>()
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid user ID in token"))?;

        Ok(AuthUser {
            user_id,
            email: claims.email,
        })
    }
}

pub async fn auth_middleware(request: Request, next: Next) -> Response {
    // The AuthUser is extracted by FromRequestParts in the handler
    // For now, just pass through
    next.run(request).await
}
