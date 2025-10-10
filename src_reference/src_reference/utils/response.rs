use crate::middleware::global::RequestId;
use axum::{
    http::{header, StatusCode},
    response::AppendHeaders,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use serde::Serialize;

// --- Core Response Structures ---

/// Represents the standardized API JSON response.
/// T: Type of the data payload for successful responses.
/// E: Type of the error payload for failed responses.
#[derive(Serialize, Debug)]
pub struct ApiResponse<T, E> {
    status: bool,
    code: u16,
    message: String,
    data: Option<T>,
    errors: Option<E>,
    meta: Meta,
}

/// Metadata associated with the API response.
#[derive(Serialize, Debug)]
pub struct Meta {
    request_id: RequestId,
    timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pagination: Option<PaginationMeta>,
}

/// Pagination details for paginated responses.
#[derive(Serialize, Debug)]
pub struct PaginationMeta {
    page: u32,
    per_page: u32,
    total: u64,
    last_page: u32,
    next_page: Option<u32>,
    prev_page: Option<u32>,
}

// --- Helper Functions for Creating Meta ---

fn create_meta(request_id: RequestId, pagination: Option<PaginationMeta>) -> Meta {
    Meta {
        request_id,
        timestamp: Utc::now().to_rfc3339(),
        pagination,
    }
}

// --- Public Constructors for ApiResponse ---

impl<T, E> ApiResponse<T, E>
where
    T: Serialize,
    E: Serialize,
{
    /// Creates a successful API response.
    pub fn success(
        status_code: StatusCode,
        message: impl Into<String>,
        data: T,
        request_id: RequestId,
        headers: Option<AppendHeaders<Vec<(header::HeaderName, String)>>>,
    ) -> impl IntoResponse {
        let response_body: ApiResponse<T, E> = ApiResponse {
            status: true,
            code: status_code.as_u16(),
            message: message.into(),
            data: Some(data),
            errors: None,
            meta: create_meta(request_id, None),
        };

        if let Some(headers) = headers {
            (status_code, headers, Json(response_body)).into_response()
        } else {
            (status_code, Json(response_body)).into_response()
        }
    }

    /// Creates a successful API response with pagination.
    pub fn paginated(
        status_code: StatusCode,
        message: impl Into<String>,
        data: T,
        pagination: PaginationMeta,
        request_id: RequestId,
    ) -> impl IntoResponse {
        let response_body: ApiResponse<T, E> = ApiResponse {
            status: true,
            code: status_code.as_u16(),
            message: message.into(),
            data: Some(data),
            errors: None,
            meta: create_meta(request_id, Some(pagination)),
        };
        (status_code, Json(response_body))
    }

    /// Creates a failed API response.
    pub fn error(
        status_code: StatusCode,
        message: impl Into<String>,
        errors: E,
        request_id: RequestId,
    ) -> impl IntoResponse {
        let response_body: ApiResponse<T, E> = ApiResponse {
            status: false,
            code: status_code.as_u16(),
            message: message.into(),
            data: None,
            errors: Some(errors),
            meta: create_meta(request_id, None),
        };
        (status_code, Json(response_body))
    }
}
