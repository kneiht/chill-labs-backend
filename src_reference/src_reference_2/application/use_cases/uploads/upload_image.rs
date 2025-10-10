use crate::application::repositories::BaseRepository;
use crate::application::services::ImageUploadService;
use crate::application::use_cases::response::UseCaseResponse;
use crate::entities::{CreateImageDto, Image};
use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug)]
pub struct UploadImageInput {
    pub file: Vec<u8>,
}

#[async_trait]
pub trait UseCase<Input, Output> {
    async fn execute(&self, input: Input) -> Result<UseCaseResponse<Output>>;
}

pub struct UploadImageUseCase<
    I: ImageUploadService + Send + Sync,
    R: BaseRepository<Image> + Send + Sync,
> {
    pub image_upload_service: I,
    pub image_repository: R,
}

#[async_trait]
impl<I: ImageUploadService + Send + Sync, R: BaseRepository<Image> + Send + Sync>
    UseCase<UploadImageInput, Image> for UploadImageUseCase<I, R>
{
    async fn execute(&self, input: UploadImageInput) -> Result<UseCaseResponse<Image>> {
        // Upload image
        let response = self.image_upload_service.upload(input.file).await?;
        let url = response.url;

        // Create image entity
        let create_dto = CreateImageDto { url };
        let image = Image::create(create_dto).await?;

        // Save to repository
        let saved_image = self.image_repository.add(image).await?;

        Ok(UseCaseResponse::success_created(
            saved_image,
            "Image uploaded successfully",
        ))
    }
}
