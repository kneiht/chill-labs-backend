use crate::entities::{BaseEntity, Entity, EntityError, HasId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// Post DTOs
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreatePostDto {
    #[validate(length(min = 3))]
    pub title: String,
    pub content: String,
    pub image_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct HydratePostDto {
    pub id: Uuid,
    #[validate(length(min = 3))]
    pub title: Option<String>,
    pub content: Option<String>,
    pub image_id: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdatePostDto {
    pub id: Uuid,
    #[validate(length(min = 3))]
    pub title: Option<String>,
    pub content: Option<String>,
    pub image_id: Option<String>,
}

impl HasId for UpdatePostDto {
    fn id(&self) -> Uuid {
        self.id
    }
}

// Post entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    #[serde(flatten)]
    pub base: BaseEntity,
    pub title: Option<String>,
    pub content: Option<String>,
    pub image_id: Option<String>,
}

impl Post {
    pub fn new() -> Self {
        Self {
            base: BaseEntity::new(),
            title: None,
            content: None,
            image_id: None,
        }
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn with_content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    pub fn with_image_id(mut self, image_id: String) -> Self {
        self.image_id = Some(image_id);
        self
    }

    pub async fn create(dto: CreatePostDto) -> Result<Self, EntityError> {
        dto.validate()?;
        let mut post = Self::new().with_title(dto.title).with_content(dto.content);
        if let Some(image_id) = dto.image_id {
            post = post.with_image_id(image_id);
        }
        Ok(post)
    }

    pub async fn hydrate(dto: HydratePostDto) -> Result<Self, EntityError> {
        dto.validate()?;
        Ok(Self {
            base: BaseEntity {
                id: dto.id,
                created_at: dto.created_at,
                updated_at: dto.updated_at,
            },
            title: dto.title,
            content: dto.content,
            image_id: dto.image_id,
        })
    }
}

#[async_trait::async_trait]
impl Entity for Post {
    type CreateDto = CreatePostDto;
    type UpdateDto = UpdatePostDto;
    async fn create(dto: Self::CreateDto) -> Result<Self, EntityError> {
        Post::create(dto).await
    }
    fn update(&mut self, dto: &Self::UpdateDto) {
        if let Some(title) = &dto.title {
            self.title = Some(title.clone());
        }
        if let Some(content) = &dto.content {
            self.content = Some(content.clone());
        }
        if let Some(image_id) = &dto.image_id {
            self.image_id = Some(image_id.clone());
        }
        self.base.updated_at = chrono::Utc::now();
    }
}

impl HasId for Post {
    fn id(&self) -> uuid::Uuid {
        self.base.id
    }
}
