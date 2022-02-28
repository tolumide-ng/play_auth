use rocket::{serde::json::Json, State};
use serde::{Deserialize};
use sqlx::{Postgres, Pool};

use crate::base_repository::user::DbUser;


#[derive(Deserialize)]
pub struct User {
    email: String,
    username: String,
    password: String,
}


#[post("/create", data = "<user>")]
pub async fn create(user: Json<User>, pool: &State<Pool<Postgres>>) -> &'static str {
    let user = DbUser::email_exist(user.email.clone(), pool).await;
    
    "Hello World from tolumide"
}

