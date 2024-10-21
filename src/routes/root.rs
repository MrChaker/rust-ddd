use crate::config::database::Database;
use axum::{routing::get, Router};

use super::users_router::users_router;

pub fn init(db: Database) -> Router {
    let app = Router::new()
        .route("/", get(|| async { "Hello" }))
        .merge(users_router(db));

    app
}
