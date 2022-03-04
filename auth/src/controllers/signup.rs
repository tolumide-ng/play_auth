use rocket::{serde::json::Json, State};
use serde::{Deserialize};
use sqlx::{Postgres, Pool};

use crate::{base_repository::user::DbUser, settings::variables::EnvVars, helpers::{mail::{Email, MailType}, auth::Password}};


#[derive(Deserialize)]
pub struct User {
    email: String,
    username: String,
    password: String,
}


#[post("/create", data = "<user>")]
pub async fn create(user: Json<User>, pool: &State<Pool<Postgres>>, env: &State<EnvVars>) -> &'static str {
    let user_exists = DbUser::email_exist(user.email.clone(), pool, env).await;

    if !user_exists {
        // verify the password at this point
        let pwd = Password::new(user.password.clone());
        match pwd {
            Some(hash) => {
                // persist on postgres at this point
                Email::new(user.email.clone(), user.password.clone(), MailType::Signup("")).send_email();
                return "Hello World from tolumide";
            },
            None => { return "Invalid password" }
        }
    }

    "User exists already"

}

