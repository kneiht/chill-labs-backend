use anyhow::Context;
use application::services::{JwtService, LocalImageUploadService};
use axum::{
    Json, Router,
    http::{Method, StatusCode},
    response::IntoResponse,
    routing::get,
};
use serde::Serialize;
use std::marker::PhantomData;
use std::sync::Arc;
use tokio::fs;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::cors::{Any, CorsLayer};

mod adapters;
mod application;
mod entities;
mod state;
use crate::application::use_cases::AddUseCase;
use crate::application::use_cases::AddUserUseCase;
use crate::application::use_cases::CheckAuthUseCase;
use crate::application::use_cases::DeleteByIdUseCase;
use crate::application::use_cases::GetAllUseCase;
use crate::application::use_cases::GetByIdUseCase;
use crate::application::use_cases::LoginUseCase;
use crate::application::use_cases::RegisterUseCase;
use crate::application::use_cases::UpdateUseCase;
use crate::application::use_cases::UpdateUserUseCase;
use adapters::api::routes;
use adapters::repositories::in_memory::{
    ImageInMemoryRepository, PostInMemoryRepository, UserInMemoryRepository,
};

pub struct AppError(anyhow::Error);

impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        Self(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}

#[derive(Serialize)]
struct Response {
    message: &'static str,
}

async fn hello_json() -> (StatusCode, Json<Response>) {
    let response = Response {
        message: "Hello, world!",
    };
    (StatusCode::OK, Json(response))
}

fn generate_message() -> anyhow::Result<&'static str> {
    if rand::random() {
        anyhow::bail!("no message for you");
    }
    Ok("Hello, world!")
}

async fn test_error() -> Result<(StatusCode, Json<Response>), AppError> {
    let response = Response {
        message: generate_message()?,
    };
    Ok((StatusCode::OK, Json(response)))
}

fn main() -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async move {
            println!("Starting server...");
            // Create uploads directory
            fs::create_dir_all("uploads").await?;

            // Initialize repositories
            let user_repo = Arc::new(UserInMemoryRepository::new().await);
            let post_repo = Arc::new(PostInMemoryRepository::new().await);
            let image_repo = Arc::new(ImageInMemoryRepository::new().await);

            // Initialize services
            let jwt_service = Arc::new(JwtService::new("your-secret-key".to_string()));
            let image_upload_service =
                Arc::new(LocalImageUploadService::new("uploads".to_string()));

            // Initialize use cases
            let add_user_use_case = Arc::new(AddUserUseCase {
                user_repository: user_repo.clone(),
            });
            let get_all_users_use_case = Arc::new(GetAllUseCase {
                repository: user_repo.clone(),
                _phantom: PhantomData,
            });
            let get_user_by_id_use_case = Arc::new(GetByIdUseCase {
                repository: user_repo.clone(),
                _phantom: PhantomData,
            });
            let update_user_use_case = Arc::new(UpdateUserUseCase {
                user_repository: user_repo.clone(),
            });
            let delete_user_by_id_use_case = Arc::new(DeleteByIdUseCase {
                repository: user_repo.clone(),
                _phantom: PhantomData,
            });
            let add_post_use_case = Arc::new(AddUseCase {
                repository: post_repo.clone(),
                _phantom: PhantomData,
            });
            let get_all_posts_use_case = Arc::new(GetAllUseCase {
                repository: post_repo.clone(),
                _phantom: PhantomData,
            });
            let get_post_by_id_use_case = Arc::new(GetByIdUseCase {
                repository: post_repo.clone(),
                _phantom: PhantomData,
            });
            let update_post_use_case = Arc::new(UpdateUseCase {
                repository: post_repo.clone(),
                _phantom: PhantomData,
            });
            let delete_post_by_id_use_case = Arc::new(DeleteByIdUseCase {
                repository: post_repo.clone(),
                _phantom: PhantomData,
            });
            let add_image_use_case = Arc::new(AddUseCase {
                repository: image_repo.clone(),
                _phantom: PhantomData,
            });
            let get_all_images_use_case = Arc::new(GetAllUseCase {
                repository: image_repo.clone(),
                _phantom: PhantomData,
            });
            let login_use_case = Arc::new(LoginUseCase {
                user_repository: user_repo.clone(),
                json_web_token: jwt_service.clone(),
            });
            let register_use_case = Arc::new(RegisterUseCase {
                json_web_token: jwt_service.clone(),
                add_user_use_case: add_user_use_case.clone(),
            });
            let check_auth_use_case = Arc::new(CheckAuthUseCase {
                json_web_token: jwt_service.clone(),
                user_repository: user_repo.clone(),
            });

            let repos = state::Repositories {
                user_repo,
                post_repo,
                image_repo,
            };

            let use_cases = state::UseCases {
                add_user_use_case,
                get_all_users_use_case,
                get_user_by_id_use_case,
                update_user_use_case,
                delete_user_by_id_use_case,
                add_post_use_case,
                get_all_posts_use_case,
                get_post_by_id_use_case,
                update_post_use_case,
                delete_post_by_id_use_case,
                add_image_use_case,
                get_all_images_use_case,
                login_use_case,
                register_use_case,
                check_auth_use_case,
            };

            let services = state::Services {
                jwt_service,
                image_upload_service,
            };

            let state = state::AppState {
                repos,
                use_cases,
                services,
            };

            let cors = CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::DELETE,
                    Method::OPTIONS,
                ])
                .allow_headers(Any);

            let app = Router::new()
                .route("/api/", get(hello_json))
                .route("/api/error", get(test_error))
                .nest("/api/auth", routes::auth_routes())
                .nest("/api/users", routes::user_routes(state.clone()))
                .nest("/api/posts", routes::post_routes())
                .nest("/api/images", routes::image_routes())
                .with_state(state)
                .layer(cors)
                .layer(CatchPanicLayer::new());

            let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
                .await
                .context("failed to bind TCP listener")?;
            axum::serve(listener, app)
                .await
                .context("axum::serve failed")?;

            Ok::<(), anyhow::Error>(())
        })?;
    Ok(())
}
