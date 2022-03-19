use auth_macro::jwt::JwtHelper;
use jsonwebtoken::TokenData;
use rocket::{serde::json::Json, State};
use redis::{AsyncCommands};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::{settings::config::Settings, helpers::{commons::{ApiResult, make_redis_key}, mail::Email, pwd::Password, jwt::{ForgotPasswordJwt, Jwt}}, response::ApiSuccess, base_repository::user::DbUser, errors::app::ApiError};

#[derive(Deserialize, Serialize)]
pub struct User {
    email: String,
    password: String,
    token: String,
}

#[put("/reset", data = "<user>")]
pub async fn reset(
    user: Json<User>,
    pool: &State<Pool<Postgres>>,
    state: &State<Settings>,
    redis: &State<redis::Client>,
) -> ApiResult<Json<ApiSuccess<&'static str>>> {
    let User {email, password, token} = user.0;

    let token_data: TokenData<ForgotPasswordJwt> = ForgotPasswordJwt::decode(&token, &state.app)?;

    let mut redis_conn = redis.get_async_connection().await?;
    let user_id = token_data.claims.get_user();

    let key = make_redis_key("forgot", user_id);

    let key_exists: Option<String> = redis_conn.get(&key).await?;


    if let Some(data) = key_exists {
        if data == token {
            let parsed_email = Email::parse(email)?;
            let parsed_password = Password::new(password, &state.app)?;
    
            DbUser::update_pwd(pool, parsed_password, parsed_email).await?;
            // delete the forgot jwt token for this user
            redis::cmd("DEL").arg(&[&key]).query_async(&mut redis_conn).await?;
            // delete all current login_jwts for this user
            let login_key = format!("{}:*", make_redis_key("login", user_id));
            redis::cmd("DEL").arg(&[&login_key]).query_async(&mut redis_conn).await?;
    
            return Ok(ApiSuccess::reply_success(Some("password reset successful")));
        }
    }

    Err(ApiError::AuthorizationError("Token is either expired or invalid"))
}