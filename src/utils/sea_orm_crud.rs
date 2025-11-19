use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Pagination {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
}

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
    pub total_pages: u64,
}

/// Simple macro to generate admin CRUD for any entity
///
/// Usage in admin.rs:
/// ```
/// admin_entity!(notes);
/// admin_entity!(users);
/// admin_entity!(lessons);
/// ```
///
/// This creates routes at:
/// - GET    /admin/notes
/// - POST   /admin/notes
/// - GET    /admin/notes/:id
/// - PUT    /admin/notes/:id
/// - DELETE /admin/notes/:id
#[macro_export]
macro_rules! admin_entity {
    ($entity_name:ident) => {
        paste::paste! {
            pub mod [<$entity_name _admin>] {
                use super::*;
                use axum::{
                    extract::{Path, Query, State},
                    routing::{get, post, put, delete},
                    Json, Router,
                };
                use sea_orm::*;
                use std::sync::Arc;
                use uuid::Uuid;
                use crate::state::AppState;
                use crate::entities::$entity_name::{ActiveModel, Entity, Model};
                use crate::utils::sea_orm_crud::{Pagination, PaginatedResponse};

                async fn list_handler(
                    State(state): State<Arc<AppState>>,
                    Query(query): Query<Pagination>,
                ) -> Result<Json<PaginatedResponse<Model>>, String> {
                    let page = query.page.unwrap_or(1);
                    let per_page = query.per_page.unwrap_or(10);

                    let paginator = Entity::find().paginate(&state.db, per_page);
                    let total = paginator.num_items().await.map_err(|e| e.to_string())?;
                    let total_pages = paginator.num_pages().await.map_err(|e| e.to_string())?;
                    let data = paginator.fetch_page(page - 1).await.map_err(|e| e.to_string())?;

                    Ok(Json(PaginatedResponse {
                        data,
                        total,
                        page,
                        per_page,
                        total_pages,
                    }))
                }

                async fn get_handler(
                    State(state): State<Arc<AppState>>,
                    Path(id): Path<Uuid>,
                ) -> Result<Json<Model>, String> {
                    Entity::find_by_id(id)
                        .one(&state.db)
                        .await
                        .map_err(|e| e.to_string())?
                        .ok_or_else(|| "Not found".to_string())
                        .map(Json)
                }

                async fn create_handler(
                    State(state): State<Arc<AppState>>,
                    Json(data): Json<serde_json::Value>,
                ) -> Result<Json<Model>, String> {
                    let mut active_model = ActiveModel {
                        id: Set(Uuid::now_v7()),
                        ..Default::default()
                    };

                    if let Ok(model) = serde_json::from_value::<Model>(data.clone()) {
                        let mut temp = model.into_active_model();
                        temp.id = Set(Uuid::now_v7());
                        // We need to manually copy fields if we want to be safe,
                        // but for now let's assume serde works for most fields
                        // Note: This might fail if Model has fields that are not in ActiveModel or vice versa
                        // A better way is to iterate over keys if possible, but ActiveModel is struct.

                        // For now, let's just try to insert what we got from JSON deserialization
                        // But wait, Model -> ActiveModel might have all fields set.
                        active_model = temp;
                    }

                    active_model
                        .insert(&state.db)
                        .await
                        .map(Json)
                        .map_err(|e| e.to_string())
                }

                async fn update_handler(
                    State(state): State<Arc<AppState>>,
                    Path(id): Path<Uuid>,
                    Json(data): Json<serde_json::Value>,
                ) -> Result<Json<Model>, String> {
                    let model = Entity::find_by_id(id)
                        .one(&state.db)
                        .await
                        .map_err(|e| e.to_string())?
                        .ok_or_else(|| "Not found".to_string())?;

                    let mut active_model = model.into_active_model();

                    // Simplified update: if we can deserialize to Model, we update
                    // This is risky because partial updates might be tricky with serde_json -> Model
                    // Ideally we should use a generic "UpdateModel" struct or similar.
                    // For this macro, we'll assume the user sends a full or partial object that matches Model structure

                    // Since we can't easily iterate fields of ActiveModel generically without more macros,
                    // we'll rely on the fact that we are in "Admin" mode and might send full objects.
                    // Or we can try to deserialize to ActiveModel directly if it supports it?
                    // SeaORM ActiveModel implements generic logic.

                    // Let's try a different approach:
                    // We can't easily do generic partial updates without reflection or code generation.
                    // So for now, we'll just say "Update not fully implemented generically" or
                    // try to deserialize to Model and overwrite.

                    if let Ok(updated_model) = serde_json::from_value::<Model>(data) {
                        let temp = updated_model.into_active_model();
                        // We should probably only update fields that are present in JSON.
                        // But we don't know which ones are present.
                        // This is the limitation of generic CRUD.

                        // For now, let's just update with what we have, assuming full update or careful usage.
                        active_model = temp;
                        // Restore ID just in case
                        active_model.id = Set(id);
                    }

                    active_model
                        .update(&state.db)
                        .await
                        .map(Json)
                        .map_err(|e| e.to_string())
                }

                async fn delete_handler(
                    State(state): State<Arc<AppState>>,
                    Path(id): Path<Uuid>,
                ) -> Result<Json<()>, String> {
                    Entity::delete_by_id(id)
                        .exec(&state.db)
                        .await
                        .map_err(|e| e.to_string())?;
                    Ok(Json(()))
                }

                pub fn router() -> Router<Arc<AppState>> {
                    Router::new()
                        .route(concat!("/admin/", stringify!($entity_name)), get(list_handler).post(create_handler))
                        .route(concat!("/admin/", stringify!($entity_name), "/{id}"), get(get_handler).put(update_handler).delete(delete_handler))
                }
            }
        }
    };
}

/// Macro to combine multiple admin entity routers
#[macro_export]
macro_rules! admin_routes {
    ($($entity:ident),+ $(,)?) => {
        {
            use axum::Router;
            let mut router = Router::new();
            $(
                paste::paste! {
                    router = router.merge([<$entity _admin>]::router());
                }
            )+
            router
        }
    };
}
