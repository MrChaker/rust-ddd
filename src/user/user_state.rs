use crate::config::database::Database;
use async_trait::async_trait;
use axum::Error;

use super::{
    infastructure::persistance::repositories::{
        user_create_repository::UserCreateRepository, user_read_repository::UserReadRepository,
    },
    ports::{
        create_repository::UserCreateRepositoryTrait, read_repository::UserReadRepositoryTrait,
    },
    services::{create_user_services::UserCreateServices, read_user_services::UserReadServices},
};

#[derive(Clone)]
pub struct UserState {
    pub read_repository: UserReadRepository,
    //create_repository: UserCreateRepository,
}

impl UserState {
    pub fn new(db_conn: Database) -> Self {
        Self {
            read_repository: UserReadRepository::new(db_conn),
            //create_repository: UserCreateRepository::new(db_conn),
        }
    }
}
