use crate::user::domain::user::User;
use crate::user::ports::read_repository::UserReadRepositoryTrait;

// Service repo should implement the repo trait
pub struct UserReadServices<T: UserReadRepositoryTrait> {
    user_repo: T,
}

pub trait UserServicesTrait {
    fn get_users(&self) -> Vec<User>;
}

impl<T: UserReadRepositoryTrait> UserServicesTrait for UserReadServices<T> {
    fn get_users(&self) -> Vec<User> {
        //self.user_repo.find("id");
        self.user_repo.find("id");
        vec![]
    }
}
