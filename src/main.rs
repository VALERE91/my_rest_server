#![feature(const_trait_impl)]

mod api;
mod models;
mod repository;

use api::user_api::{create_user, get_user};
use crate::repository::user_repo::UserRepo; //import the handler here

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    let repo = UserRepo::init();

    rocket::build()
        .manage(repo)
        .mount("/", routes![create_user])
        .mount("/", routes![get_user])
}