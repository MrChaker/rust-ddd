use axum::{routing::get, Router};

use crate::config::database::Database;

use super::{handlers::get_users::get_users, user_state::UserState};

pub fn user_routes(db_connection: Database) -> Router<UserState> {
    let user_state = UserState::new(db_connection);

    let router: Router<UserState> = Router::new()
        .route("/users", get(get_users))
        .with_state(user_state);

    router
    //router.nest("/users", &router);
}
