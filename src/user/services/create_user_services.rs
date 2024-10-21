use crate::user::domain::user::User;
use crate::user::ports::create_repository::UserCreateRepositoryTrait;

// Service repo should implement the repo trait
pub struct UserCreateServices<T: UserCreateRepositoryTrait> {
    user_repo: T,
}

pub trait UserServicesTrait {
    fn create(&self) -> Vec<User>;
}

impl<T: UserCreateRepositoryTrait> UserServicesTrait for UserCreateServices<T> {
    fn create(&self) -> Vec<User> {
        //self.user_repo.find("id");
        self.user_repo.create("id");
        vec![]
    }
}
