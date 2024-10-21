use super::user_entity::User;

pub struct UserServices {
    user_repo: UserRepository,
}

pub trait UserServicesTrait {
    fn get_users(&self) -> Vec<User>;
}

impl UserServicesTrait for UserServices {
    fn get_users(&self) -> Vec<User> {
        //self.user_repo.find("id");
        vec![]
    }
}
