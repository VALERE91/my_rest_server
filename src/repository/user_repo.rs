use crate::models::user_model::User;

pub struct UserRepo{
    user: User
}

impl UserRepo {
    pub fn init() -> Self {
        UserRepo{
            user : User{
                id: 0,
                name: "Toto".parse().unwrap(),
                title: "Machin".parse().unwrap(),
                location: "Truc".parse().unwrap()
            }
        }
    }

    pub fn getUser(&self) -> User {
        self.user.clone()
    }
}
