use crate::config::database::Database;
use async_trait::async_trait;
use axum::Error;

#[async_trait]
pub trait UserCreateRepositoryTrait {
    fn new(db_conn: Database) -> Self;

    async fn create(&self, id: &str) -> Result<String, Error>;
}
