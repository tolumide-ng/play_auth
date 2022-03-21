// REALLY BAD IMPLEMENTATION, CAN WE MODIFY THIS AND MAKE IT CLEANER? --- HANDLE ONLY RESET PASSWORD TO MAKE THIS CLEANER AND EASIER ON THE EYES

use auth_macro::jwt::JwtHelper;
use jsonwebtoken::TokenData;
use redis::{AsyncCommands};
use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};

use crate::errors::app::ApiError;
use crate::settings::config::Settings;
use crate::helpers::jwt_tokens::jwt::{ForgotPasswordJwt, Jwt};
use crate::helpers::commons::{RedisKey, RedisPrefix};

pub struct Reset<'r>(&'r str);

#[derive(Debug)]
pub enum ResetError {
    Missing,
    Invalid,
    ServerError
}

// pub enum {}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Reset<'r> {
    type Error = ResetError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let redis = req.rocket().state::<redis::Client>().unwrap();
        let app_env = req.rocket().state::<Settings>().unwrap();
        let redis_conn = redis.get_async_connection().await;

        if redis_conn.is_err() {
            return Outcome::Failure((Status::InternalServerError, ResetError::ServerError))
        }

        let mut redis_conn = redis.get_async_connection().await.unwrap();

        let path = req.uri().path().as_str().to_string();
                        // let key_exists: Option<String> = redis_conn.get("key").await.unwrap();
        // println!("the path {:#?}", path);
        // let path = origin;
        // let key = RedisKey::new(RedisPrefix::Forgot, );



        fn is_valid(key: &str) -> bool {
            key == "valid_api_key"
        }

        match req.headers().get_one("authorization") {
            None => Outcome::Failure((Status::BadRequest, ResetError::Missing)),
            Some(key) => {
                // if is_valid(key) => 
                if path.contains("reset") {
                    // we can make all of them here (errors) into an enum that returns Outcome
                    let token_data: Result<TokenData<ForgotPasswordJwt>, ApiError> = ForgotPasswordJwt::decode(&key, &app_env.app);

                    if let Ok(data) = token_data {
                        let user_id = data.claims.get_user();
                        let redis_key = RedisKey::new(RedisPrefix::Forgot, user_id).make_key();
                        let key_exists: Option<String> = redis_conn.get(&redis_key).await.unwrap();

                        if let Some(fetched_key) = key_exists {
                            if key == fetched_key {
                                return Outcome::Success(Reset(key))
                            }
                        }
                        
                    }
                    
                }
                return Outcome::Failure((Status::Unauthorized, ResetError::ServerError))
            },
            // Some(_) => Outcome::Failure((Status::BadRequest, ResetError::Invalid)),
        }
    }
}