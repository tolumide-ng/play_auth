use auth_macro::jwt::JwtHelper;
use jsonwebtoken::TokenData;
use redis::{AsyncCommands};
use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::{settings::config::Settings, helpers::{commons::{ApiResult, RedisKey, RedisPrefix}, jwt::{Jwt, LoginJwt, SignupJwt}, mail::{Email, MailType, MailInfo}}, response::ApiSuccess, errors::app::ApiError, base_repository::user::DbUser};

#[derive(Deserialize, Serialize)]
pub struct User {
    // this should be on the request header instead
    token: String
}

#[post("/resend_verify", data = "<user>")]
pub async fn resend_verification_token(
    user: Json<User>,
    pool: &State<Pool<Postgres>>,
    state: &State<Settings>,
    redis: &State<redis::Client>,
) -> ApiResult<Json<ApiSuccess<&'static str>>> {
    let User { token } = user.0;

    // Check if the token is valid
    let token_data: TokenData<LoginJwt> = LoginJwt::decode(&token, &state.app)?;

    let mut redis_conn = redis.get_async_connection().await?;

    // Get the calims on the token
    let data = token_data.claims;
    let user_id = data.get_user();

    // Get Redis prefix for login
    let login_key = RedisKey::new(RedisPrefix::Login, user_id).make_key();

    // Check if the login jwt is still valid i.e. has not expired
    let key_exists: Option<String> = redis_conn.get(&login_key).await?;

    if key_exists.is_some() {
        let email = Email::parse(data.email())?;
        // Get the user info from db
        let the_user = DbUser::email_exists(&pool, &email).await?;
    
    
        if let Some(user) = the_user {
            // Remove any currently active signup token for this user
            let signup_key = format!("{}:*", RedisKey::new(RedisPrefix::Signup, user_id).make_key());
            redis::cmd("DEL").arg(&[&signup_key]).query_async(&mut redis_conn).await?;
    
            if !user.is_verified() {
                // let id = Uuid::parse_str(user.get_user().as_str());
                let signup_token = SignupJwt::new(user.get_user().1).encode(&state.app)?;
                let info = MailInfo::new(signup_token, &state.app.frontend_url);
                let mail_type = MailType::Signup(info);
                Email::new(email, None, mail_type).send_email(&state.email);
                
                return Ok(ApiSuccess::reply_success(Some("Please check your email to verify your account")));
            }
        }
    }

    return Err(ApiError::AuthorizationError("Invalid token"))
}