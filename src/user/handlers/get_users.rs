use axum::{extract::State, Error, Json};

use crate::{
    error::api_error::ApiError,
    user::{
        domain::user::User, ports::read_repository::UserReadRepositoryTrait, user_state::UserState,
    },
};

#[axum::debug_handler]
pub async fn get_users(state: State<UserState>) -> Result<Json<Vec<User>>, ApiError> {
    let users = state.read_repository.find("id").await;

    Ok(Json(vec![]))
}
