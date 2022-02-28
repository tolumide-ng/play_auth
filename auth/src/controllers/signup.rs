use rocket::{serde::json::Json};
use serde::{Deserialize};


#[derive(Deserialize)]
pub struct User {
    email: String,
    username: String,
    password: String,
}


#[post("/create", data = "<user>")]
pub async fn create(user: Json<User>) {
    // let user = User::email_exist();
}

