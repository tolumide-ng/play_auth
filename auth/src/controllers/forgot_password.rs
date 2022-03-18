use redis::RedisError;
use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::{settings::config::Settings, helpers::{mail::Email, commons::{ApiResult, Str}}, errors::app::ApiError, base_repository::user::DbUser, response::ApiSuccess};


#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    email: String,
}

// #[post("/forgot_password", data = "<user>")]
#[post("/forgot", data = "<user>")]
pub async fn forgot(
    user: Json<User>,
    pool: &State<Pool<Postgres>>,
    state: &State<Settings>,
    redis: &State<redis::Client>,
) -> ApiResult<Json<ApiSuccess<Str>>> {
    let User { email } = user.0;
    let parsed_email = Email::parse(email);

    if parsed_email.is_err() {
        return Err(ApiError::BadRequest("Please check your email for the link to reset your password"))
    }
    
    let valid_email = parsed_email.unwrap();
    let user = DbUser::email_exists(pool, &valid_email).await?;
    
    if user.is_none() {
        // Avoid telling the user whether the email exists or not (Security)
        return Ok(ApiSuccess::reply_success(Some("Please check your email to reset your password")))
    }

    let mut redis_conn = redis.get_async_connection().await?;

    let key = format!("forgot_{}", valid_email);

    println!("THE KEY {:#?}", key);

    let avc: String = redis::cmd("SET").arg(&[key.clone(), "abcd".to_string()]).query_async(&mut redis_conn).await.unwrap();

    println!("value of avc {:#?}", avc);
    // remember to set an expiry for every key

    let forgot_pwd_exists: Result<String, RedisError> = redis::cmd("GET").arg(&[key]).query_async(&mut redis_conn).await;

    if forgot_pwd_exists.is_ok() {
        // this user has requested for a password changed in the last one
        return Ok(ApiSuccess::reply_success(Some("Please check your email for the link to reset your password")))
    }

    // At this point, we haven't sent the user a new password in the last 1 hour, and the user exists

    // let forgot_pwd_exists: String = redis::cmd("GET").arg(&[key]).query_async(&mut redis_conn).await.unwrap();
    // there's something wrong with this get request investigate it!!

    println!("TH VALE>>>>>>>>>>>>>>>>>>>>>>>>>>>>>U {:#?}", forgot_pwd_exists);

    // if !forgot_pwd_exists {}

    // if let Some(db_user) = user {
        
    // }

    return Ok(ApiSuccess::reply_success(Some("Please check your email to reset your password")))
}