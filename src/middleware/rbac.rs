use axum::extract::Request;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use crate::domain::response::Response as ApiResponse;
use crate::domain::user::model::{Role, User};

pub async fn require_role(
    allowed_roles: Vec<Role>,
) -> impl Fn(
    Request,
    Next,
)
    -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response, Response>> + Send>>
       + Clone {
    move |request: Request, next: Next| {
        let allowed_roles = allowed_roles.clone();
        Box::pin(async move {
            let user = request.extensions().get::<User>().ok_or_else(|| {
                ApiResponse::<()>::failure_unauthorized(
                    "Authentication required",
                    Some(
                        "User not found in request. Did you apply auth_middleware first?"
                            .to_string(),
                    ),
                )
                .into_response()
            })?;

            if !allowed_roles.contains(&user.role) {
                return Err(ApiResponse::<()>::failure_forbidden(
                    "Insufficient permissions",
                    Some(format!(
                        "This endpoint requires one of the following roles: {:?}",
                        allowed_roles
                    )),
                )
                .into_response());
            }

            Ok(next.run(request).await)
        })
    }
}

pub async fn require_admin(request: Request, next: Next) -> Result<Response, Response> {
    let user = request.extensions().get::<User>().ok_or_else(|| {
        ApiResponse::<()>::failure_unauthorized(
            "Authentication required",
            Some("User not found in request. Did you apply auth_middleware first?".to_string()),
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

pub async fn require_teacher_or_admin(request: Request, next: Next) -> Result<Response, Response> {
    let user = request.extensions().get::<User>().ok_or_else(|| {
        ApiResponse::<()>::failure_unauthorized(
            "Authentication required",
            Some("User not found in request. Did you apply auth_middleware first?".to_string()),
        )
        .into_response()
    })?;

    if user.role != Role::Teacher && user.role != Role::Admin {
        return Err(ApiResponse::<()>::failure_forbidden(
            "Insufficient permissions",
            Some("This endpoint requires Teacher or Admin role".to_string()),
        )
        .into_response());
    }

    Ok(next.run(request).await)
}
