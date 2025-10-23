/// Macro to generate CRUD handlers.
///
/// Generates Axum handler functions for create, get, get_all, update, and delete operations.
///
/// # Usage
///
/// ```rust
/// crud_handlers!(
///     CreateRequestType,
///     UpdateRequestType,
///     ResponseType,
///     CreateInputType,
///     UpdateInputType,
///     service_field_name,
///     "ModelName",
///     request_to_create: |req| { CreateInputType { field: req.field } },
///     request_to_update: |id, req| { UpdateInputType { id, field: req.field } }
/// );
/// ```
///
/// # Parameters
///
/// - **CreateRequestType**: Request DTO for creation (e.g., `CreateNoteRequest`)
/// - **UpdateRequestType**: Request DTO for update (e.g., `UpdateNoteRequest`)
/// - **ResponseType**: Response DTO (e.g., `NoteResponse`)
/// - **CreateInputType**: Service input for creation (e.g., `CreateNoteInput`)
/// - **UpdateInputType**: Service input for update (e.g., `UpdateNoteInput`)
/// - **service_field_name**: Field name in `AppState` (e.g., `note_service`)
/// - **"ModelName"**: Display name for success messages (e.g., `"Note"`)
/// - **request_to_create**: Closure to map request to create input
/// - **request_to_update**: Closure to map id and request to update input
///
/// # Generated Handlers
///
/// - `create` - POST handler for creating new records
/// - `get` - GET handler for retrieving by ID
/// - `get_all` - GET handler for retrieving all records
/// - `update` - PUT/PATCH handler for updating records
/// - `delete` - DELETE handler for deleting records
///
/// # Example
///
/// ```rust
/// crud_handlers!(
///     CreateNoteRequest,
///     UpdateNoteRequest,
///     NoteResponse,
///     CreateNoteInput,
///     UpdateNoteInput,
///     note_service,
///     "Note",
///     request_to_create: |req| {
///         CreateNoteInput {
///             user_id: req.user_id,
///             title: req.title,
///             content: req.content,
///         }
///     },
///     request_to_update: |id, req| {
///         UpdateNoteInput {
///             id,
///             user_id: req.user_id,
///             title: req.title,
///             content: req.content,
///         }
///     }
/// );
/// ```
#[macro_export]
macro_rules! crud_handlers {
    (
        $create_req:ty,
        $update_req:ty,
        $response:ty,
        $create_input:ty,
        $update_input:ty,
        $service_field:ident,
        $model_name:expr,
        request_to_create: |$req_create:ident| $create_mapping:block,
        request_to_update: |$id_param:ident, $req_update:ident| $update_mapping:block
    ) => {
        pub async fn create(
            axum::extract::State(state): axum::extract::State<crate::state::AppState>,
            axum::Json($req_create): axum::Json<$create_req>,
        ) -> crate::domain::response::Response<$response> {
            use crate::domain::error::ToResponse;
            let service = state.$service_field.clone();
            let input = $create_mapping;

            service
                .create(input)
                .await
                .map(|m| m.into())
                .to_response_created(&format!("{} created successfully", $model_name))
        }

        pub async fn get(
            axum::extract::State(state): axum::extract::State<crate::state::AppState>,
            axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
        ) -> crate::domain::response::Response<$response> {
            use crate::domain::error::ToResponse;
            let service = state.$service_field.clone();

            service
                .get_by_id(id)
                .await
                .map(|m| m.into())
                .to_response(&format!("{} retrieved successfully", $model_name))
        }

        pub async fn get_all(
            axum::extract::State(state): axum::extract::State<crate::state::AppState>,
        ) -> crate::domain::response::Response<Vec<$response>> {
            use crate::domain::error::ToResponse;
            let service = state.$service_field.clone();

            service
                .get_all()
                .await
                .map(|models| models.into_iter().map(Into::into).collect())
                .to_response(&format!("{}s retrieved successfully", $model_name))
        }

        pub async fn update(
            axum::extract::State(state): axum::extract::State<crate::state::AppState>,
            axum::extract::Path($id_param): axum::extract::Path<uuid::Uuid>,
            axum::Json($req_update): axum::Json<$update_req>,
        ) -> crate::domain::response::Response<$response> {
            use crate::domain::error::ToResponse;
            let service = state.$service_field.clone();
            let input = $update_mapping;

            service
                .update(input)
                .await
                .map(|m| m.into())
                .to_response(&format!("{} updated successfully", $model_name))
        }

        pub async fn delete(
            axum::extract::State(state): axum::extract::State<crate::state::AppState>,
            axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
        ) -> crate::domain::response::Response<serde_json::Value> {
            use crate::domain::error::ToResponse;
            let service = state.$service_field.clone();

            service
                .delete(id)
                .await
                .to_response_no_content(&format!("{} deleted successfully", $model_name))
        }
    };
}
