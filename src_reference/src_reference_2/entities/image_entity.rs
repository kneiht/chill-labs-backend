use crate::entities::{BaseEntity, Entity, EntityError, HasId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// Image DTOs
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateImageDto {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct HydrateImageDto {
    pub id: Uuid,
    pub url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateImageDto {
    pub id: Uuid,
    pub url: Option<String>,
}

impl HasId for UpdateImageDto {
    fn id(&self) -> Uuid {
        self.id
    }
}

// Image entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    #[serde(flatten)]
    pub base: BaseEntity,
    pub url: String,
}

impl Image {
    pub fn new(url: String) -> Self {
        Self {
            base: BaseEntity::new(),
            url,
        }
    }

    pub async fn create(dto: CreateImageDto) -> Result<Self, EntityError> {
        dto.validate()?;
        Ok(Self::new(dto.url))
    }

    pub async fn hydrate(dto: HydrateImageDto) -> Result<Self, EntityError> {
        dto.validate()?;
        Ok(Self {
            base: BaseEntity {
                id: dto.id,
                created_at: dto.created_at,
                updated_at: dto.updated_at,
            },
            url: dto.url,
        })
    }
}

#[async_trait::async_trait]
impl Entity for Image {
    type CreateDto = CreateImageDto;
    type UpdateDto = UpdateImageDto;
    async fn create(dto: Self::CreateDto) -> Result<Self, EntityError> {
        Image::create(dto).await
    }
    fn update(&mut self, dto: &Self::UpdateDto) {
        if let Some(url) = &dto.url {
            self.url = url.clone();
        }
        self.base.updated_at = chrono::Utc::now();
    }
}

impl HasId for Image {
    fn id(&self) -> uuid::Uuid {
        self.base.id
    }
}
