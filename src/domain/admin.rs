// Admin CRUD routes for all entities
// Add admin_entity!(entity_name) to get full CRUD at /admin/entity_name

use crate::AppState;
use axum::Router;
use std::sync::Arc;

use crate::{admin_entity, admin_routes};

// Generate admin CRUD for each entity
admin_entity!(notes);
admin_entity!(users);
admin_entity!(lessons);
admin_entity!(sentences);
admin_entity!(vocabs);
admin_entity!(words);
admin_entity!(word_sentences);

// Combine all admin routes
pub fn router() -> Router<Arc<AppState>> {
    admin_routes!(
        notes,
        users,
        lessons,
        sentences,
        vocabs,
        words,
        word_sentences
    )
}
