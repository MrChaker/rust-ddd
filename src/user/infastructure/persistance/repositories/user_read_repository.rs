use crate::{config::database::Database, user::ports::read_repository::UserReadRepositoryTrait};
use async_trait::async_trait;
use axum::Error;

#[derive(Clone)]
pub struct UserReadRepository {
    pub db_conn: Database,
}

#[async_trait()]
impl UserReadRepositoryTrait for UserReadRepository {
    fn new(db_conn: Database) -> Self {
        UserReadRepository { db_conn }
    }

    async fn find(&self, id: &str) -> Result<String, Error> {
        Ok("Users".to_string())
    }
}
