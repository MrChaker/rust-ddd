use axum::{routing::get, Router};

use crate::config::database::Database;

pub fn users_router(db: Database) -> Router {
    let router = Router::new().route("/users", get(|| async { "Get users" }));

    router
}
