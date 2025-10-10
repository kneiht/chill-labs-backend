use crate::entities::{BaseEntity, Entity, EntityError, HasId};
use bcrypt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// Role enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Role {
    USER,
    ADMIN,
}

// User DTOs
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateUserDto {
    #[validate(length(min = 3))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
    pub role: Option<Role>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct HydrateUserDto {
    pub id: Uuid,
    #[validate(length(min = 3))]
    pub name: Option<String>,
    #[validate(email)]
    pub email: String,
    pub hashed_password: String,
    pub role: Role,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateUserDto {
    pub id: Uuid,
    #[validate(length(min = 3))]
    pub name: Option<String>,
    pub role: Option<Role>,
}

impl HasId for UpdateUserDto {
    fn id(&self) -> Uuid {
        self.id
    }
}

// User entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(flatten)]
    pub base: BaseEntity,
    pub name: Option<String>,
    pub email: String,
    pub role: Role,
    hashed_password: String,
}

impl User {
    pub fn new(email: String, hashed_password: String, role: Role) -> Self {
        Self {
            base: BaseEntity::new(),
            name: None,
            email,
            role,
            hashed_password,
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub async fn create(dto: CreateUserDto) -> Result<Self, EntityError> {
        dto.validate()?;
        let hashed_password = bcrypt::hash(dto.password, 10)?;
        let role = dto.role.unwrap_or(Role::USER);
        Ok(Self::new(dto.email, hashed_password, role).with_name(dto.name))
    }

    pub async fn hydrate(dto: HydrateUserDto) -> Result<Self, EntityError> {
        dto.validate()?;
        Ok(Self {
            base: BaseEntity {
                id: dto.id,
                created_at: dto.created_at,
                updated_at: dto.updated_at,
            },
            name: dto.name,
            email: dto.email,
            role: dto.role,
            hashed_password: dto.hashed_password,
        })
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, EntityError> {
        Ok(bcrypt::verify(password, &self.hashed_password)?)
    }
}

#[async_trait::async_trait]
impl Entity for User {
    type CreateDto = CreateUserDto;
    type UpdateDto = UpdateUserDto;
    async fn create(dto: Self::CreateDto) -> Result<Self, EntityError> {
        User::create(dto).await
    }
    fn update(&mut self, dto: &Self::UpdateDto) {
        if let Some(name) = &dto.name {
            self.name = Some(name.clone());
        }
        if let Some(role) = &dto.role {
            self.role = role.clone();
        }
        self.base.updated_at = chrono::Utc::now();
    }
}

impl HasId for User {
    fn id(&self) -> uuid::Uuid {
        self.base.id
    }
}
