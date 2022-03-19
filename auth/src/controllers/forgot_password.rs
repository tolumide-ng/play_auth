use redis::{RedisError, AsyncCommands};
use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::{settings::config::Settings, helpers::{mail::Email, commons::{ApiResult, make_redis_key}, jwt::{ForgotPasswordJwt, Jwt}}, errors::app::ApiError, base_repository::user::DbUser, response::ApiSuccess};

const EXPIRE_AT: usize = 3600;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    email: String,
}


#[post("/forgot", data = "<user>")]
pub async fn forgot(
    user: Json<User>,
    pool: &State<Pool<Postgres>>,
    state: &State<Settings>,
    redis: &State<redis::Client>,
) -> ApiResult<Json<ApiSuccess<String>>> {
    let User { email } = user.0;
    let parsed_email = Email::parse(email)?;
    
    // let valid_email = parsed_email.unwrap();
    let user = DbUser::email_exists(pool, &parsed_email).await?;
    
    if user.is_none() {
        // Avoid telling the user whether the email exists or not (Security)
        return Ok(ApiSuccess::reply_success(Some("Please check your email for the link to reset your password".to_string())))
    }

    let the_user = user.unwrap().get_user().1;
    let mut redis_conn = redis.get_async_connection().await?;
    
    let key = make_redis_key("forgot", the_user);

    let forgot_pwd_exists: Result<String, RedisError> = redis_conn.get(&key).await;
    if forgot_pwd_exists.is_ok() {
        // this user has requested for a password changed in the last one
        return Ok(ApiSuccess::reply_success(Some("Please check your email for the link to reset your password".to_string())))
    }

    // At this point, we haven't sent the user a new password in the last 1 hour, and the user exists
    let jwt = ForgotPasswordJwt::new(the_user).encode(&state.app)?;
    redis_conn.set(&key, &jwt).await?;
    redis_conn.expire(&key, EXPIRE_AT).await?;

    return Ok(ApiSuccess::reply_success(Some(jwt)))

}