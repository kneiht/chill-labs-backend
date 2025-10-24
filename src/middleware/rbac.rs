use axum::extract::Request;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use crate::domain::response::Response as ApiResponse;
use crate::domain::user::model::{Role, User};

pub async fn require_admin(request: Request, next: Next) -> Result<Response, Response> {
    let user = request.extensions().get::<User>().ok_or_else(|| {
        ApiResponse::<()>::failure_unauthorized(
            "Authentication required",
            Some("User not found in request.".to_string()),
        )
        .into_response()
    })?;

    if user.role != Role::Admin {
        return Err(ApiResponse::<()>::failure_forbidden(
            "Admin access required",
            Some("Only administrators can access this endpoint".to_string()),
        )
        .into_response());
    }

    Ok(next.run(request).await)
}
