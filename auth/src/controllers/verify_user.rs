use auth_macro::jwt::JwtHelper;
use jsonwebtoken::TokenData;
use redis::{AsyncCommands};
use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::settings::config::Settings;
use crate::helpers::jwt_tokens::jwt::{SignupJwt, Jwt};
use crate::helpers::commons::{ApiResult, RedisKey, RedisPrefix};
use crate::response::ApiSuccess;
use crate::errors::app::ApiError;
use crate::base_repository::user::DbUser;


#[derive(Deserialize, Serialize)]
pub struct User {
    token: String,
}

#[put("/verify", data = "<user>" )]
pub async fn verify(
    user: Json<User>,
    state: &State<Settings>,
    pool: &State<Pool<Postgres>>,
    redis: &State<redis::Client>,
) -> ApiResult<Json<ApiSuccess<&'static str>>> {
    let User {token} = user.0;

    let token_data: TokenData<SignupJwt> = SignupJwt::decode(&token, &state.app)?;

    let mut redis_conn = redis.get_async_connection().await?;
    let user_id = token_data.claims.get_user();

    let key = RedisKey::new(RedisPrefix::Signup, user_id).make_key();
    // does this signup token exist?
    let key_exists: Option<String> = redis_conn.get(&key).await?;

    if let Some(data) = key_exists {
        if data == token {
            DbUser::verify_user(pool, user_id).await?;
    
            redis::cmd("DEL").arg(&[&key]).query_async(&mut redis_conn).await?;
            // delete all current login_jwts, user needs to sign in again
            let login_key = format!("{}:*", RedisKey::new(RedisPrefix::Login, user_id).make_key());
            redis::cmd("DEL").arg(&[&login_key]).query_async(&mut redis_conn).await?;
            return Ok(ApiSuccess::reply_success(Some("verified")));
        }
    }

    return Err(ApiError::AuthenticationError("Token is either expired or invalid"))
}