use redis::{AsyncCommands};
use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Pool};
use dotenv::dotenv;

use crate::base_repository::user::DbUser;
use crate::helpers::commons::make_redis_key;
use crate::helpers::jwt::{SignupJwt, Jwt};
use crate::helpers::{mail::{Email, MailType}, pwd::{Password}, commons::{Str, ApiResult}};
use crate::{response::{ApiSuccess}, errors::app::ApiError, settings::config::Settings};


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
    state: &State<Settings>,
    redis: &State<redis::Client>,
) -> ApiResult<Json<ApiSuccess<Str>>> {
    dotenv().ok();
    let User {email, password} = user.0;

    let parsed_email = Email::parse(email)?;
    let parsed_pwd = Password::new(password.clone(), &state.app)?;

    let user_already_exists = DbUser::email_exists(pool, &parsed_email).await?;

    let mut redis_conn = redis.get_async_connection().await?;

    if user_already_exists.is_none() {
        let user_id = DbUser::create_user(pool, &parsed_email, parsed_pwd.to_string()).await?;
        let jwt = SignupJwt::new(user_id).encode(&state.app)?;

        let key = make_redis_key("signup", user_id);
        redis_conn.set(&key, &jwt).await?;

        println!("the signup jwt {:#?}", jwt);

        Email::new(parsed_email, None, MailType::Signup(""), Some(user_id.to_string())).send_email(&state.email);
        return Ok(ApiSuccess::reply_success(Some("Please check your email to verify your account")));
    }

    return Err(ApiError::Conflict("Email already exists"));
}
