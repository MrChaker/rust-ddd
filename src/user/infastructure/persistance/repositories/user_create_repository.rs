use crate::{
    config::database::Database, user::ports::create_repository::UserCreateRepositoryTrait,
};
use async_trait::async_trait;
use axum::Error;

#[derive(Clone)]
pub struct UserCreateRepository {
    pub db_conn: Database,
}

#[async_trait()]
impl UserCreateRepositoryTrait for UserCreateRepository {
    fn new(db_conn: Database) -> Self {
        UserCreateRepository { db_conn }
    }

    async fn create(&self, id: &str) -> Result<String, Error> {
        Ok("Created user".to_string())
    }
}
