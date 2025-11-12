use super::model::Vocab;
use super::repository::VocabRepository;
use crate::authorization::get_ownership_filter;
use crate::domain::user::model::User;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Validate)]
pub struct CreateVocabInput {
    pub user_id: Uuid,
    #[validate(length(min = 1, message = "Words list cannot be empty"))]
    pub words: Vec<super::model::Word>,
}

#[derive(Debug, Clone, Validate)]
pub struct UpdateVocabInput {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub words: Option<Vec<super::model::Word>>,
}

crate::crud_service!(
    VocabService,
    Vocab,
    VocabRepository,
    CreateVocabInput,
    UpdateVocabInput,
    "Vocab",
    create_logic: |input| {
        Vocab::new(input.user_id, input.words)
    },
    update_logic: |model, input| {
        if let Some(user_id) = input.user_id {
            model.user_id = user_id;
        }
        if let Some(words) = input.words {
            model.words = words;
        }
    }
);

impl VocabService {
    /// Get vocab lists filtered by user role:
    /// - Admins see all vocab lists
    /// - Regular users see only their own vocab lists
    pub async fn get_vocabs_by_user(
        &self,
        user: &User,
    ) -> Result<Vec<Vocab>, crate::domain::error::AppError> {
        match get_ownership_filter(user) {
            None => {
                // Admin - get all vocab lists
                self.repository
                    .find_all()
                    .await
                    .map_err(crate::domain::error::AppError::from)
            }
            Some(user_id) => {
                // Regular user - get only their vocab lists
                self.repository
                    .find_by_user_id(user_id)
                    .await
                    .map_err(crate::domain::error::AppError::from)
            }
        }
    }
}