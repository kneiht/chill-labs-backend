use crate::domain::error::AppError;
use crate::domain::response::Response;
use crate::domain::Transformer;
use crate::state::AppState;
use axum::extract::{Path, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

/// Macro to generate a CRUD repository similar to UserRepository.
///
/// Usage: crud_repository!(RepoName, ModelType, RowType, "table_name", insert_fields..., select_fields..., update_fields..., enum_fields...);
///
/// - RepoName: Name of the repository struct (e.g., UserRepository).
/// - ModelType: The model type (e.g., User).
/// - RowType: The row type for DB (e.g., UserRow).
/// - "table_name": String literal for the table name.
/// - insert_fields: List of field names for INSERT (e.g., id, display_name, ...).
/// - select_fields: List of field names for SELECT (same as insert).
/// - update_fields: List of field names for UPDATE (without id, created).
/// - enum_fields: List of enum field names (e.g., role, status).
///
/// This generates create, find_by_id, find_all, update, delete methods.
#[macro_export]
macro_rules! crud_repository {
    ($repo_name:ident, $model:ty, $row:ty, $table:expr, $( $insert_field:ident ),* ; $( $select_field:ident ),* ; $( $update_field:ident ),* ; $( $enum_field:ident ),* ) => {
        #[derive(Clone)]
        pub struct $repo_name {
            pool: PgPool,
        }

        impl $repo_name {
            pub fn new(pool: PgPool) -> Self {
                Self { pool }
            }

            pub async fn create<T: Transformer<$model>>(&self, to_model: T) -> Result<$model, AppError> {
                let model = to_model.transform()?;

                let placeholders: Vec<String> = (1..=vec![$( stringify!($insert_field) ),*].len()).map(|i| format!("${}", i)).collect();
                let placeholders_str = placeholders.join(", ");

                let query = format!(
                    "INSERT INTO {} ({}) VALUES ({}) RETURNING {}",
                    $table,
                    vec![$( stringify!($insert_field) ),*].join(", "),
                    placeholders_str,
                    vec![$( stringify!($select_field) ),*].join(", ")
                );

                let result = sqlx::query_as!( $row, &query $( , model.$insert_field )* )
                    .fetch_one(&self.pool)
                    .await
                    .map_err(AppError::from)?;

                Ok(result.into())
            }

            pub async fn find_by_id(&self, id: Uuid) -> Result<Option<$model>, AppError> {
                let query = format!("SELECT {} FROM {} WHERE id = $1", vec![$( stringify!($select_field) ),*].join(", "), $table);
                let user = sqlx::query_as!( $row, &query, id )
                    .fetch_optional(&self.pool)
                    .await
                    .map_err(AppError::from)?;
                Ok(user.map(|u| u.into()))
            }

            pub async fn find_all(&self) -> Result<Vec<$model>, AppError> {
                let query = format!("SELECT {} FROM {} ORDER BY created DESC", vec![$( stringify!($select_field) ),*].join(", "), $table);
                let rows = sqlx::query_as!( $row, &query )
                    .fetch_all(&self.pool)
                    .await
                    .map_err(AppError::from)?;
                Ok(rows.into_iter().map(|u| u.into()).collect())
            }

            pub async fn update<T: Transformer<$model>>(&self, to_model: T) -> Result<$model, AppError> {
                let model = to_model.transform()?;

                let set_parts: Vec<String> = vec![$( stringify!($update_field) ),*].iter().enumerate().map(|(i, f)| {
                    format!("{} = ${}", f, i + 2)
                }).collect();
                let set_str = set_parts.join(", ");

                let query = format!(
                    "UPDATE {} SET {} WHERE id = $1 RETURNING {}",
                    $table, set_str, vec![$( stringify!($select_field) ),*].join(", ")
                );

                let result = sqlx::query_as!( $row, &query, model.id $( , model.$update_field )* )
                    .fetch_one(&self.pool)
                    .await
                    .map_err(AppError::from)?;

                Ok(result.into())
            }

            pub async fn delete(&self, id: Uuid) -> Result<bool, AppError> {
                let query = format!("DELETE FROM {} WHERE id = $1", $table);
                let result = sqlx::query!(&query, id)
                    .execute(&self.pool)
                    .await
                    .map_err(AppError::from)?;
                Ok(result.rows_affected() > 0)
            }
        }
    };
}

/// Macro to generate a CRUD service similar to UserService.
///
/// Usage: crud_service!(ServiceName, ModelType, RepositoryType, CreateInputType, UpdateInputType, "model_name");
///
/// - ServiceName: Name of the service struct (e.g., UserService).
/// - ModelType: The model type (e.g., User).
/// - RepositoryType: The repository type (e.g., UserRepository).
/// - CreateInputType: The create DTO type (e.g., CreateUserInput).
/// - UpdateInputType: The update DTO type (e.g., UpdateUserInput).
/// - "model_name": String for error messages (e.g., "User").
///
/// Assumes:
/// - CreateInputType has fields: display_name, username, email, password_hash, role.
/// - UpdateInputType has fields: id, display_name, username, email, role, status.
/// - Model has new() method and fields like User.
/// - Repository has create, find_by_id, find_by_email, find_by_username, find_all, update, delete.
/// - Error types like username_already_exists, email_already_exists, user_not_found are available.
///
/// This generates create, get_by_id, get_by_email, get_by_username, get_all, update, delete methods.
#[macro_export]
macro_rules! crud_service {
    ($service_name:ident, $model:ty, $repo:ty, $create_input:ty, $update_input:ty, $model_name:expr) => {
        #[derive(Clone)]
        pub struct $service_name {
            repository: $repo,
        }

        impl $service_name {
            pub fn new(repository: $repo) -> Self {
                Self { repository }
            }

            pub async fn create<T: Transformer<$create_input>>(
                &self,
                to_create: T,
            ) -> Result<$model, AppError> {
                let create_input = to_create.transform()?;
                let model = <$model>::new(
                    create_input.user_id,
                    create_input.title,
                    create_input.content,
                );

                self.repository.create(model).await
            }

            pub async fn get_by_id(&self, id: Uuid) -> Result<$model, AppError> {
                self.repository
                    .find_by_id(id)
                    .await?
                    .ok_or_else(|| AppError::user_not_found(id))
            }

            pub async fn get_by_email(&self, email: &str) -> Result<$model, AppError> {
                self.repository.find_by_email(email).await?.ok_or_else(|| {
                    AppError::NotFound(format!("{} with email {} not found", $model_name, email))
                })
            }

            pub async fn get_by_username(&self, username: &str) -> Result<$model, AppError> {
                self.repository
                    .find_by_username(username)
                    .await?
                    .ok_or_else(|| {
                        AppError::NotFound(format!(
                            "{} with username {} not found",
                            $model_name, username
                        ))
                    })
            }

            pub async fn get_all(&self) -> Result<Vec<$model>, AppError> {
                self.repository.find_all().await
            }

            pub async fn update<T: Transformer<$update_input>>(
                &self,
                to_update: T,
            ) -> Result<$model, AppError> {
                let update_input = to_update.transform()?;
                let mut model = self.get_by_id(update_input.id).await?;

                if let Some(user_id) = update_input.user_id {
                    model.user_id = user_id;
                }

                if let Some(title) = update_input.title {
                    model.title = title;
                }

                if let Some(content) = update_input.content {
                    model.content = content;
                }

                model.updated = chrono::Utc::now();
                self.repository.update(model).await
            }

            pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
                self.get_by_id(id).await?;
                if !self.repository.delete(id).await? {
                    return Err(AppError::user_not_found(id));
                }
                Ok(())
            }
        }
    };
}

/// Macro to generate CRUD handlers similar to User handlers.
///
/// Usage: crud_handlers!(ModelType, ServiceType, CreateRequestType, UpdateRequestType, ResponseType, ServiceField, "model_name");
///
/// - ModelType: The model type (e.g., User).
/// - ServiceType: The service type (e.g., UserService).
/// - CreateRequestType: The create request DTO (e.g., CreateUserRequest).
/// - UpdateRequestType: The update request DTO (e.g., UpdateUserRequest).
/// - ResponseType: The response DTO (e.g., UserResponse).
/// - ServiceField: The field in AppState for the service (e.g., user_service).
/// - "model_name": String for success messages (e.g., "User").
///
/// Assumes:
/// - CreateRequestType has fields: display_name, username, email, password.
/// - UpdateRequestType has fields: display_name, username, email, role, status.
/// - ResponseType implements From<ModelType>.
/// - Service has create, get_by_id, get_all, update, delete methods.
/// - Password hashing function is available.
///
/// Generates create, get, get_all, update, delete handlers.
#[macro_export]
macro_rules! crud_handlers {
    ($model:ty, $service:ty, $create_req:ty, $update_req:ty, $response:ty, $service_field:ident, $model_name:expr) => {
        pub async fn create(
            State(state): State<AppState>,
            Json(req): Json<$create_req>,
        ) -> Response<$response> {
            let service = state.$service_field.clone();

            let create_input = <$create_input> {
                user_id: req.user_id,
                title: req.title,
                content: req.content,
            };
            service
                .create(create_input)
                .await
                .map(|m| m.into())
                .to_response_created(&format!("{} created successfully", $model_name))
        }

        pub async fn get(
            State(state): State<AppState>,
            Path(id): Path<Uuid>,
        ) -> Response<$response> {
            let service = state.$service_field.clone();
            service
                .get_by_id(id)
                .await
                .map(|m| m.into())
                .to_response(&format!("{} retrieved successfully", $model_name))
        }

        pub async fn get_all(State(state): State<AppState>) -> Response<Vec<$response>> {
            let service = state.$service_field.clone();
            service
                .get_all()
                .await
                .map(|models| models.into_iter().map(Into::into).collect())
                .to_response(&format!("{}s retrieved successfully", $model_name))
        }

        pub async fn update(
            State(state): State<AppState>,
            Path(id): Path<Uuid>,
            Json(req): Json<$update_req>,
        ) -> Response<$response> {
            let service = state.$service_field.clone();
            let update_input = super::service::UpdateInput {
                id,
                user_id: req.user_id,
                title: req.title,
                content: req.content,
            };
            service
                .update(update_input)
                .await
                .map(|m| m.into())
                .to_response(&format!("{} updated successfully", $model_name))
        }

        pub async fn delete(
            State(state): State<AppState>,
            Path(id): Path<Uuid>,
        ) -> Response<serde_json::Value> {
            let service = state.$service_field.clone();
            service
                .delete(id)
                .await
                .to_response_no_content(&format!("{} deleted successfully", $model_name))
        }
    };
}
