use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Pool};
use dotenv::dotenv;

use crate::{base_repository::user::DbUser, helpers::{mail::{Email, MailType}, auth::{Password}, commons::{Str, ApiResult}}, response::{ApiSuccess}, errors::app::ApiError, settings::config::Settings};


#[derive(Deserialize, Serialize)]
pub struct User {
    email: String,
    password: String,
}


// todo!() only dispatch an event into the queue when a user has been verified on the verify endpoint

#[post("/create", data = "<user>")]
pub async fn create(
    user: Json<User>, 
    pool: &State<Pool<Postgres>>, 
    envs: &State<Settings>
) -> ApiResult<Json<ApiSuccess<Str>>> {
    dotenv().ok();
    let User {email, password} = user.0;
    let user_exists = DbUser::email_exists(pool, email.clone()).await?;

    if user_exists.is_none() {
        let pwd = Password::new(password.clone(), &envs.app);
        match pwd {
            Some(hash) => {
                let user = DbUser::create_user(pool, email.clone(), hash.get_val()).await?;

                if user {
                    // generate jwt 
                    // SignupJwt::new(signup_id)
                    Email::new(email, None, MailType::Signup("")).send_email(&envs.email);
                    return Ok(ApiSuccess::reply_success(Some("Please check your email to verify your account")));
                }
            },
            None => { 
                return Err(ApiError::ValidationError("Password must be atleast 8 characters long 
                    containing atleast one special character, a capital letter, a small letter, and a digit"));
             }
        }
    }

    return Err(ApiError::Conflict("Email already exists"));
}
