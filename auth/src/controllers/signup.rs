use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Pool};

use crate::{base_repository::user::DbUser, settings::variables::EnvVars, helpers::{mail::{Email, MailType}, auth::Password}, response::AppResponder};


#[derive(Deserialize, Serialize)]
pub struct User {
    email: String,
    username: String,
    password: String,
}


// todo!() only dispatch an event into the queue when a user has been verified on the verify endpoint

#[post("/create", data = "<user>")]
pub async fn create(user: Json<User>, pool: &State<Pool<Postgres>>, env: &State<EnvVars>) -> Json<AppResponder<&'static str>> {
    let User {email, username, password} = user.0;

    let user_exists = DbUser::email_exist(pool, email.clone(), username.clone()).await;

    if !user_exists {
        // verify the password at this point
        let pwd = Password::new(password.clone());
        match pwd {
            Some(hash) => {
                // persist on postgres at this point
                DbUser::create_user(pool, email.clone(), username.clone(), hash.get_val()).await;

                Email::new(email, username, MailType::Signup("")).send_email();
                return AppResponder::reply_success(Some("Please check your email to verify your account"));
            },
            None => { 
                return AppResponder::reply("Error", Some("Password must be atleast 8 characters long containing atleast
                one special character, a capital letter, a small letter, and a digit"), 400)
             }
        }
    }

    return AppResponder::reply("Error", Some("Email already exists"), 409)
}

