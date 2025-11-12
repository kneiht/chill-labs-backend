use axum::extract::{Extension, Path, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::model::{Vocab, Word};
use super::service::{CreateVocabInput, UpdateVocabInput};
use crate::authorization::can_access_resource;
use crate::domain::error::ToResponse;
use crate::domain::response::Response;
use crate::domain::user::model::User;
use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct CreateVocabRequest {
    pub words: Vec<Word>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateVocabRequest {
    pub words: Option<Vec<Word>>,
}

#[derive(Debug, Serialize)]
pub struct VocabResponse {
    pub id: String,
    pub user_id: String,
    pub words: Vec<Word>,
    pub created: chrono::DateTime<chrono::Utc>,
    pub updated: chrono::DateTime<chrono::Utc>,
}

impl From<Vocab> for VocabResponse {
    fn from(vocab: Vocab) -> Self {
        Self {
            id: vocab.id.to_string(),
            user_id: vocab.user_id.to_string(),
            words: vocab.words,
            created: vocab.created,
            updated: vocab.updated,
        }
    }
}

// GET /vocabs - Get all vocab lists (admins see all, users see only their own)
pub async fn get_all_vocabs(
    State(state): State<AppState>,
    Extension(authenticated_user): Extension<User>,
) -> Response<Vec<VocabResponse>> {
    state
        .vocab_service
        .get_vocabs_by_user(&authenticated_user)
        .await
        .map(|vocabs| vocabs.into_iter().map(VocabResponse::from).collect())
        .to_response("Vocab lists retrieved successfully")
}

// GET /vocabs/:id - Get a specific vocab list
pub async fn get_vocab(
    State(state): State<AppState>,
    Extension(authenticated_user): Extension<User>,
    Path(id): Path<Uuid>,
) -> Response<VocabResponse> {
    // First, fetch the vocab
    let vocab = match state.vocab_service.get_by_id(id).await {
        Ok(vocab) => vocab,
        Err(e) => {
            return Response::failure_not_found(&e.to_string(), Some(e.to_string()));
        }
    };

    // Check authorization
    if !can_access_resource(&authenticated_user, &vocab) {
        return Response::failure_forbidden(
            "You don't have permission to access this vocab list",
            Some("FORBIDDEN".to_string()),
        );
    }

    Response::success_ok(VocabResponse::from(vocab), "Vocab list retrieved successfully")
}

// POST /vocabs - Create a new vocab list
pub async fn create_vocab(
    State(state): State<AppState>,
    Extension(authenticated_user): Extension<User>,
    Json(req): Json<CreateVocabRequest>,
) -> Response<VocabResponse> {
    // User can only create vocab lists for themselves
    let input = CreateVocabInput {
        user_id: authenticated_user.id,
        words: req.words,
    };

    state
        .vocab_service
        .create(input)
        .await
        .map(VocabResponse::from)
        .to_response_created("Vocab list created successfully")
}

// PUT /vocabs/:id - Update a vocab list
pub async fn update_vocab(
    State(state): State<AppState>,
    Extension(authenticated_user): Extension<User>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateVocabRequest>,
) -> Response<VocabResponse> {
    // First, fetch the vocab
    let vocab = match state.vocab_service.get_by_id(id).await {
        Ok(vocab) => vocab,
        Err(e) => {
            return Response::failure_not_found(&e.to_string(), Some(e.to_string()));
        }
    };

    // Check authorization
    if !can_access_resource(&authenticated_user, &vocab) {
        return Response::failure_forbidden(
            "You don't have permission to update this vocab list",
            Some("FORBIDDEN".to_string()),
        );
    }

    // Perform the update
    let input = UpdateVocabInput {
        id,
        user_id: None, // Never allow changing the owner
        words: req.words,
    };

    state
        .vocab_service
        .update(input)
        .await
        .map(VocabResponse::from)
        .to_response("Vocab list updated successfully")
}

// DELETE /vocabs/:id - Delete a vocab list
pub async fn delete_vocab(
    State(state): State<AppState>,
    Extension(authenticated_user): Extension<User>,
    Path(id): Path<Uuid>,
) -> Response<serde_json::Value> {
    // First, fetch the vocab
    let vocab = match state.vocab_service.get_by_id(id).await {
        Ok(vocab) => vocab,
        Err(e) => {
            return Response::failure_not_found(&e.to_string(), Some(e.to_string()));
        }
    };

    // Check authorization
    if !can_access_resource(&authenticated_user, &vocab) {
        return Response::failure_forbidden(
            "You don't have permission to delete this vocab list",
            Some("FORBIDDEN".to_string()),
        );
    }

    // Perform the deletion
    state
        .vocab_service
        .delete(id)
        .await
        .to_response_no_content("Vocab list deleted successfully")
}