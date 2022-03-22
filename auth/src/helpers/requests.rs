// REALLY BAD IMPLEMENTATION, CAN WE MODIFY THIS AND MAKE IT CLEANER? --- HANDLE ONLY AuthHeader PASSWORD TO MAKE THIS CLEANER AND EASIER ON THE EYES

use auth_macro::jwt::JwtHelper;
use jsonwebtoken::TokenData;
use redis::aio::Connection;
use redis::{AsyncCommands};
use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};

use crate::errors::app::ApiError;
use crate::settings::config::Settings;
use crate::helpers::jwt_tokens::jwt::{LoginJwt, Jwt};
use crate::helpers::commons::{RedisKey, RedisPrefix};


#[derive(Debug)]
pub struct AuthHeader{
    id: String,
    email: String,
}

impl AuthHeader {
    pub fn new(email: String, id: String) -> Self {
        Self { email, id }
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }
    pub fn email(&self) -> String {
        self.email.to_string()
    }
}

#[derive(Debug)]
pub enum AuthHeaderError {
    Missing,
    Invalid,
    ServerError
}


async fn is_valid(token: &str, app_env: &Settings, conn: &mut Connection) -> Result<AuthHeader, ApiError> {
    let token_data: TokenData<LoginJwt> = LoginJwt::decode(&token, &app_env.app)?;
    let data = token_data.claims;
    let user_id = data.get_user();
    let redis_key = RedisKey::new(RedisPrefix::Login, user_id).make_key();
    let key_exists: Option<String> = conn.get(&redis_key).await?;

    if let Some(value) = key_exists {
        if value.len() > 0 && value == token {
            return Ok(AuthHeader::new(data.email(), data.get_user().to_string()));
        }
    }

    Err(ApiError::AuthenticationError("Authorization header is invalid"))
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthHeader {
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
            Some(token) if path.contains("resend_verify") => {
                let valid = is_valid(token, app_env, &mut conn).await;
                if let Ok(info) = valid {
                    return Outcome::Success(info)
                }

                Outcome::Failure((Status::Unauthorized, ApiError::AuthenticationError("")))
            },
            Some(_) => Outcome::Failure((Status::Unauthorized, ApiError::AuthenticationError(""))),
        }
    }
}