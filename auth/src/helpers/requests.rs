// REALLY BAD IMPLEMENTATION, CAN WE MODIFY THIS AND MAKE IT CLEANER? --- HANDLE ONLY RESET PASSWORD TO MAKE THIS CLEANER AND EASIER ON THE EYES

use auth_macro::jwt::JwtHelper;
use jsonwebtoken::TokenData;
use redis::aio::Connection;
use redis::{AsyncCommands};
use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};
use derive_more;
use uuid::Uuid;

use crate::errors::app::ApiError;
use crate::settings::config::Settings;
use crate::helpers::jwt_tokens::jwt::{ForgotPasswordJwt, Jwt};
use crate::helpers::commons::{RedisKey, RedisPrefix};

#[derive(Debug, derive_more::Display)]
pub struct Reset(pub Uuid);

#[derive(Debug)]
pub enum ResetError {
    Missing,
    Invalid,
    ServerError
}


async fn is_valid(token: &str, app_env: &Settings, conn: &mut Connection) -> Result<Uuid, ApiError> {
    let token_data: TokenData<ForgotPasswordJwt> = ForgotPasswordJwt::decode(&token, &app_env.app)?;
    let user_id = token_data.claims.get_user();
    let redis_key = RedisKey::new(RedisPrefix::Forgot, user_id).make_key();
    let key_exists: Option<String> = conn.get(&redis_key).await?;

    if let Some(value) = key_exists {
        if value.len() > 0 && value == token {
            return Ok(user_id)
        }
    }

    Err(ApiError::AuthenticationError("Authorization key is either empty of invalid"))
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Reset {
    type Error = ApiError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let redis = req.rocket().state::<redis::Client>().unwrap();
        let app_env = req.rocket().state::<Settings>().unwrap();
        let redis_conn = redis.get_async_connection().await;

        if redis_conn.is_err() {
            return Outcome::Failure((Status::InternalServerError, ApiError::InternalServerError))
        }

        let mut conn = redis.get_async_connection().await.unwrap();

        let path = req.uri().path().as_str().to_string();



       

        match req.headers().get_one("authorization") {
            None => Outcome::Failure((Status::Unauthorized, ApiError::AuthenticationError(""))),
            Some(token) if path.contains("reset") => {
                let valid = is_valid(token, app_env, &mut conn).await;
                if let Ok(user_id) = valid {
                    return Outcome::Success(Reset(user_id))
                }

                Outcome::Failure((Status::Unauthorized, ApiError::AuthenticationError("")))
            },
            Some(_) => Outcome::Failure((Status::Unauthorized, ApiError::AuthenticationError(""))),
        }
    }
}