use super::model::{Vocab, VocabRow};
use uuid::Uuid;

#[derive(Clone)]
pub struct VocabRepository {
    pool: sqlx::PgPool,
}

impl VocabRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn create<T: crate::domain::Transformer<Vocab>>(
        &self,
        to_model: T,
    ) -> Result<Vocab, crate::domain::error::AppError> {
        let model = to_model.transform()?;
        let words_json = model.to_json_value()?;

        let result = sqlx::query_as::<_, VocabRow>(
            "INSERT INTO vocabs (id, user_id, words, created, updated) VALUES ($1, $2, $3, $4, $5) RETURNING id, user_id, words, created, updated"
        )
        .bind(model.id)
        .bind(model.user_id)
        .bind(words_json)
        .bind(model.created)
        .bind(model.updated)
        .fetch_one(&self.pool)
        .await
        .map_err(crate::domain::error::AppError::from)?;

        Ok(result.into())
    }

    pub async fn find_by_id(
        &self,
        id: uuid::Uuid,
    ) -> Result<Option<Vocab>, crate::domain::error::AppError> {
        let vocab = sqlx::query_as::<_, VocabRow>(
            "SELECT id, user_id, words, created, updated FROM vocabs WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(crate::domain::error::AppError::from)?;
        Ok(vocab.map(|v| v.into()))
    }

    pub async fn find_all(&self) -> Result<Vec<Vocab>, crate::domain::error::AppError> {
        let rows = sqlx::query_as::<_, VocabRow>(
            "SELECT id, user_id, words, created, updated FROM vocabs ORDER BY created DESC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(crate::domain::error::AppError::from)?;
        Ok(rows.into_iter().map(|v| v.into()).collect())
    }

    pub async fn update<T: crate::domain::Transformer<Vocab>>(
        &self,
        to_model: T,
    ) -> Result<Vocab, crate::domain::error::AppError> {
        let model = to_model.transform()?;
        let words_json = model.to_json_value()?;

        let result = sqlx::query_as::<_, VocabRow>(
            "UPDATE vocabs SET user_id = $2, words = $3, updated = $4 WHERE id = $1 RETURNING id, user_id, words, created, updated"
        )
        .bind(model.id)
        .bind(model.user_id)
        .bind(words_json)
        .bind(chrono::Utc::now())
        .fetch_one(&self.pool)
        .await
        .map_err(crate::domain::error::AppError::from)?;

        Ok(result.into())
    }

    pub async fn delete(&self, id: uuid::Uuid) -> Result<bool, crate::domain::error::AppError> {
        let result = sqlx::query("DELETE FROM vocabs WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(crate::domain::error::AppError::from)?;
        Ok(result.rows_affected() > 0)
    }

    /// Find all vocab lists for a specific user
    pub async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Vocab>, sqlx::Error> {
        let rows = sqlx::query_as::<_, VocabRow>(
            "SELECT id, user_id, words, created, updated 
             FROM vocabs 
             WHERE user_id = $1 
             ORDER BY updated DESC",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Vocab::from).collect())
    }
}
