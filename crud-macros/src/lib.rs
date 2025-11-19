use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, LitStr, Token, Type,
};

struct CrudInput {
    entity: Type,
    model: Type,
    active_model: Type,
    path: LitStr,
}

impl Parse for CrudInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut entity = None;
        let mut model = None;
        let mut active_model = None;
        let mut path = None;

        while !input.is_empty() {
            let key: syn::Ident = input.parse()?;
            input.parse::<Token![:]>()?;

            if key == "entity" {
                entity = Some(input.parse()?);
            } else if key == "model" {
                model = Some(input.parse()?);
            } else if key == "active_model" {
                active_model = Some(input.parse()?);
            } else if key == "path" {
                path = Some(input.parse()?);
            } else {
                return Err(syn::Error::new(key.span(), "Unknown key"));
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(CrudInput {
            entity: entity.ok_or_else(|| input.error("Missing 'entity'"))?,
            model: model.ok_or_else(|| input.error("Missing 'model'"))?,
            active_model: active_model.ok_or_else(|| input.error("Missing 'active_model'"))?,
            path: path.ok_or_else(|| input.error("Missing 'path'"))?,
        })
    }
}

#[proc_macro]
pub fn make_crud_routes(input: TokenStream) -> TokenStream {
    let CrudInput {
        entity,
        model,
        active_model,
        path,
    } = parse_macro_input!(input as CrudInput);

    // Generate unique names for the handlers to avoid collisions if multiple macros are used in the same file
    // Actually, we can just put them in a module or give them unique names based on the entity.
    // Let's use a module for cleanliness.

    // We need to convert the path to a string for the router
    let path_str = path.value();

    // Extract the last segment of the entity path to use as a base for naming
    // e.g., if entity is `crate::entities::users::Entity`, we want `users`
    // This is a bit complex to do robustly in a macro without more info,
    // but we can generate a module name based on the path or just use a fixed structure if we return a block.

    // Let's generate a block that returns the router.

    let expanded = quote! {
        {
            use axum::{
                extract::{Path, State, Query},
                routing::{get, post, put, delete},
                Json, Router,
            };
            use sea_orm::{
                ActiveModelTrait, EntityTrait, IntoActiveModel, Set, TryIntoModel, ActiveValue,
                QueryOrder, QuerySelect, PaginatorTrait, ModelTrait
            };
            use std::sync::Arc;
            use serde_json::Value;

            // Handlers

            async fn list_items(
                State(state): State<Arc<crate::AppState>>,
                Query(params): Query<std::collections::HashMap<String, String>>,
            ) -> impl axum::response::IntoResponse {
                // Simple pagination
                let page = params.get("page").and_then(|p| p.parse::<u64>().ok()).unwrap_or(1);
                let per_page = params.get("per_page").and_then(|p| p.parse::<u64>().ok()).unwrap_or(10);

                let paginator = <#entity>::find()
                    .paginate(&state.db, per_page);

                let items = paginator.fetch_page(page - 1).await;

                match items {
                    Ok(items) => {
                         let total = paginator.num_items().await.unwrap_or(0);
                         let total_pages = paginator.num_pages().await.unwrap_or(0);

                         axum::Json(serde_json::json!({
                             "data": items,
                             "meta": {
                                 "page": page,
                                 "per_page": per_page,
                                 "total": total,
                                 "total_pages": total_pages
                             }
                         }))
                    },
                    Err(e) => {
                        // TODO: Better error handling
                        axum::Json(serde_json::json!({ "error": e.to_string() }))
                    }
                }
            }

            async fn get_item(
                State(state): State<Arc<crate::AppState>>,
                Path(id): Path<uuid::Uuid>,
            ) -> impl axum::response::IntoResponse {
                let item = <#entity>::find_by_id(id)
                    .one(&state.db)
                    .await;

                match item {
                    Ok(Some(item)) => axum::Json(serde_json::json!(item)),
                    Ok(None) => axum::Json(serde_json::json!({ "error": "Not found" })),
                    Err(e) => axum::Json(serde_json::json!({ "error": e.to_string() })),
                }
            }

            async fn create_item(
                State(state): State<Arc<crate::AppState>>,
                Json(payload): Json<Value>,
            ) -> impl axum::response::IntoResponse {
                // Use from_json to create ActiveModel
                let mut active_model = <#active_model>::from_json(payload);

                match active_model {
                    Ok(mut am) => {
                        // Ensure ID is generated if not present (assuming UUID v7)
                        // If the model has an ID field that is a primary key and not auto-increment in DB (like UUID),
                        // we might need to set it.
                        // However, `from_json` might not set it if it's missing.
                        // For now, let's assume the payload might contain it or we rely on DB default?
                        // Our User entity has `id: Uuid`, not auto-generated by DB usually unless specified.
                        // Let's try to set ID if it's "NotSet".

                        // This part is tricky generic-wise.
                        // We can check if the entity has an `id` column and if it's UUID.
                        // But we don't know the field names easily here without inspecting the struct definition which we don't have.
                        // We only have the type names.

                        // Workaround: Try to save. If it fails due to missing ID, the user needs to send it or we need a smarter macro.
                        // Or we can assume a specific field name "id" for UUIDs.

                        // For now, let's just try to save.

                        let res = am.save(&state.db).await;
                        match res {
                            Ok(model) => {
                                // We need to convert ActiveModel back to Model to serialize it nicely,
                                // or just return the ID?
                                // `save` returns ActiveModel.
                                // We can try_into_model
                                match model.try_into_model() {
                                    Ok(m) => axum::Json(serde_json::json!(m)),
                                    Err(e) => axum::Json(serde_json::json!({ "error": "Failed to convert to model" })),
                                }
                            },
                            Err(e) => axum::Json(serde_json::json!({ "error": e.to_string() })),
                        }
                    },
                    Err(e) => axum::Json(serde_json::json!({ "error": e.to_string() })),
                }
            }

            async fn update_item(
                State(state): State<Arc<crate::AppState>>,
                Path(id): Path<uuid::Uuid>,
                Json(payload): Json<Value>,
            ) -> impl axum::response::IntoResponse {
                // First find the item
                let item = <#entity>::find_by_id(id)
                    .one(&state.db)
                    .await;

                match item {
                    Ok(Some(model)) => {
                        let mut active_model: #active_model = model.into();

                        // Update from JSON
                        // set_from_json takes &mut self and json
                        match active_model.set_from_json(payload) {
                            Ok(_) => {
                                let res = active_model.save(&state.db).await;
                                match res {
                                    Ok(updated_am) => {
                                        match updated_am.try_into_model() {
                                            Ok(m) => axum::Json(serde_json::json!(m)),
                                            Err(_) => axum::Json(serde_json::json!({ "error": "Failed to convert to model" })),
                                        }
                                    },
                                    Err(e) => axum::Json(serde_json::json!({ "error": e.to_string() })),
                                }
                            },
                            Err(e) => axum::Json(serde_json::json!({ "error": e.to_string() })),
                        }
                    },
                    Ok(None) => axum::Json(serde_json::json!({ "error": "Not found" })),
                    Err(e) => axum::Json(serde_json::json!({ "error": e.to_string() })),
                }
            }

            async fn delete_item(
                State(state): State<Arc<crate::AppState>>,
                Path(id): Path<uuid::Uuid>,
            ) -> impl axum::response::IntoResponse {
                let res = <#entity>::delete_by_id(id)
                    .exec(&state.db)
                    .await;

                match res {
                    Ok(res) => {
                        if res.rows_affected == 0 {
                             axum::Json(serde_json::json!({ "error": "Not found" }))
                        } else {
                             axum::Json(serde_json::json!({ "message": "Deleted successfully" }))
                        }
                    },
                    Err(e) => axum::Json(serde_json::json!({ "error": e.to_string() })),
                }
            }

            Router::new()
                .route(#path_str, get(list_items).post(create_item))
                .route(&format!("{}/{{id}}", #path_str), get(get_item).put(update_item).delete(delete_item))
        }
    };

    TokenStream::from(expanded)
}
