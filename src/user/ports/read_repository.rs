use async_trait::async_trait;
use axum::Error;

use crate::config::database::Database;

#[async_trait]
pub trait UserReadRepositoryTrait {
    fn new(db_conn: Database) -> Self;
    async fn find(&self, id: &str) -> Result<String, Error>;
}
