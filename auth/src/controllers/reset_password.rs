use auth_macro::jwt::JwtHelper;
use jsonwebtoken::TokenData;
use rocket::{serde::json::Json, State};
use redis::{AsyncCommands};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::helpers::requests::Reset;
use crate::settings::config::Settings;
use crate::helpers::commons::{ApiResult, RedisKey, RedisPrefix};
use crate::helpers::{mails::email::Email, passwords::pwd::Password};
use crate::helpers::jwt_tokens::jwt::{ForgotPasswordJwt, Jwt};
use crate::response::ApiSuccess;
use crate::base_repository::user::DbUser;
use crate::errors::app::ApiError;

#[derive(Deserialize, Serialize)]
pub struct User {
    email: String,
    password: String,
}



#[put("/reset", data = "<user>")]
pub async fn reset(
    guard: Reset,
    user: Json<User>,
    pool: &State<Pool<Postgres>>,
    state: &State<Settings>,
    redis: &State<redis::Client>,
) -> ApiResult<Json<ApiSuccess<&'static str>>> {
    let User {email, password } = user.0;

    let user_id = guard.0;
    let mut redis_conn = redis.get_async_connection().await?;

    let key = RedisKey::new(RedisPrefix::Forgot, user_id).make_key();

    let parsed_email = Email::parse(email)?;
    let parsed_password = Password::new(password, &state.app)?;

    DbUser::update_pwd(pool, parsed_password, parsed_email).await?;
    // delete the forgot jwt token for this user
    redis::cmd("DEL").arg(&[&key]).query_async(&mut redis_conn).await?;
    // delete all current login_jwts for this user
    let login_key = format!("{}:*", RedisKey::new(RedisPrefix::Login, user_id).make_key());
    redis::cmd("DEL").arg(&[&login_key]).query_async(&mut redis_conn).await?;

    return Ok(ApiSuccess::reply_success(Some("password reset successful")));
}