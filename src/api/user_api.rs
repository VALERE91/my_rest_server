use rocket::{http::Status, serde::json::Json, State};
use crate::models::user_model::User;
use crate::repository::user_repo::UserRepo;

#[post("/user", data = "<new_user>")]
pub fn create_user(db: &State<UserRepo>, new_user: Json<User>)
    -> Result<Json<User>, Status>{
    let data = User {
        id: 42,
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };

    Ok(Json(data))
}

#[get("/user")]
pub fn get_user(db: &State<UserRepo>)-> Result<Json<User>, Status> {
    Ok(Json(db.getUser()))
}

pub fn delete_user(){

}