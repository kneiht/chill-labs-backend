use std::sync::Arc;

use crate::adapters::repositories::in_memory::{
    ImageInMemoryRepository, PostInMemoryRepository, UserInMemoryRepository,
};

use crate::application::services::{JwtService, LocalImageUploadService};
use crate::application::use_cases::AddUseCase;
use crate::application::use_cases::DeleteByIdUseCase;
use crate::application::use_cases::GetAllUseCase;
use crate::application::use_cases::GetByIdUseCase;
use crate::application::use_cases::UpdateUseCase;
use crate::application::use_cases::AddUserUseCase;
use crate::application::use_cases::UpdateUserUseCase;
use crate::application::use_cases::CheckAuthUseCase;
use crate::application::use_cases::LoginUseCase;
use crate::application::use_cases::RegisterUseCase;
use crate::entities::{Image, Post, User};

#[derive(Clone)]
pub struct Repositories {
    pub user_repo: Arc<UserInMemoryRepository>,
    pub post_repo: Arc<PostInMemoryRepository>,
    pub image_repo: Arc<ImageInMemoryRepository>,
}

#[derive(Clone)]
pub struct UseCases {
    pub add_user_use_case: Arc<AddUserUseCase<UserInMemoryRepository>>,
    pub get_all_users_use_case: Arc<GetAllUseCase<UserInMemoryRepository, User>>,
    pub get_user_by_id_use_case: Arc<GetByIdUseCase<UserInMemoryRepository, User>>,
    pub update_user_use_case: Arc<UpdateUserUseCase<UserInMemoryRepository>>,
    pub delete_user_by_id_use_case: Arc<DeleteByIdUseCase<UserInMemoryRepository, User>>,
    pub add_post_use_case: Arc<AddUseCase<PostInMemoryRepository, Post>>,
    pub get_all_posts_use_case: Arc<GetAllUseCase<PostInMemoryRepository, Post>>,
    pub get_post_by_id_use_case: Arc<GetByIdUseCase<PostInMemoryRepository, Post>>,
    pub update_post_use_case: Arc<UpdateUseCase<PostInMemoryRepository, Post>>,
    pub delete_post_by_id_use_case: Arc<DeleteByIdUseCase<PostInMemoryRepository, Post>>,
    pub add_image_use_case: Arc<AddUseCase<ImageInMemoryRepository, Image>>,
    pub get_all_images_use_case: Arc<GetAllUseCase<ImageInMemoryRepository, Image>>,
    pub login_use_case: Arc<LoginUseCase<UserInMemoryRepository, JwtService>>,
    pub register_use_case: Arc<RegisterUseCase<JwtService, AddUserUseCase<UserInMemoryRepository>>>,
    pub check_auth_use_case: Arc<CheckAuthUseCase<UserInMemoryRepository, JwtService>>,
}

#[derive(Clone)]
pub struct Services {
    pub jwt_service: Arc<JwtService>,
    pub image_upload_service: Arc<LocalImageUploadService>,
}

#[derive(Clone)]
pub struct AppState {
    pub repos: Repositories,
    pub use_cases: UseCases,
    pub services: Services,
}
