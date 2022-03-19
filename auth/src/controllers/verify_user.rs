use auth_macro::jwt::JwtHelper;
use jsonwebtoken::TokenData;
use redis::{RedisError, AsyncCommands};
use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::{settings::config::Settings, helpers::{jwt::{SignupJwt, Jwt}, commons::{ApiResult, make_redis_key}}, response::ApiSuccess, errors::app::ApiError, base_repository::user::DbUser};


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

    let user_token: Result<TokenData<SignupJwt>, _> = SignupJwt::decode(&token, &state.app);

    match user_token {
        Ok(token) => {
            let mut redis_conn = redis.get_async_connection().await?;
            let user_id = token.claims.get_user();

            let key = make_redis_key("signup", user_id);
            // does this signup token exist?
            let key_exists: Result<Option<String>, RedisError> = redis_conn.get(&key).await;

            if key_exists.is_ok() && key_exists.unwrap().is_some() {
                DbUser::verify_user(pool, user_id).await?;

                redis::cmd("DEL").arg(&[&key]).query_async(&mut redis_conn).await?;
                // delete all current login_jwts, user needs to sign in again
                let login_key = format!("{}:*", make_redis_key("login", user_id));
                redis::cmd("DEL").arg(&[&login_key]).query_async(&mut redis_conn).await?;
                return Ok(ApiSuccess::reply_success(Some("verified")));
            }

            return Err(ApiError::BadRequest("Token is either expired or does not exist"))
        },
        Err(e) => {
            println!("THE ACTUAL ERR {:#?}", e);
            Err(ApiError::BadRequest("Token is either expired or does not exist"))
        }
    }
}