use redis::{RedisError, AsyncCommands};
use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use uuid::Uuid;


use crate::settings::config::Settings;
use crate::helpers::commons::{ApiResult, RedisKey, RedisPrefix, MINUTES_60};
use crate::helpers::mails::email::{Email, MailInfo, MailType};
use crate::helpers::jwt_tokens::jwt::{ForgotPasswordJwt, Jwt};
use crate::response::ApiSuccess;
use crate::base_repository::user::DbUser;


#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    email: String,
}

const MESSAGE: &'static str = "Please check your email for the link to reset your password";


#[post("/forgot", data = "<user>")]
pub async fn forgot(
    user: Json<User>,
    pool: &State<Pool<Postgres>>,
    state: &State<Settings>,
    redis: &State<redis::Client>,
) -> ApiResult<Json<ApiSuccess<String>>> {
    let User { email } = user.0;
    let parsed_email = Email::parse(email)?;

    let user = DbUser::email_exists(pool, &parsed_email).await?;
    
    if user.is_none() {
        // Avoid telling the user whether the email exists or not (Security)
        return Ok(ApiSuccess::reply_success(Some(MESSAGE.to_string())))
    }

    let the_user = user.unwrap().get_user().user_id;

    let mut redis_conn = redis.get_async_connection().await?;

    let key = RedisKey::new(RedisPrefix::Forgot, the_user).make_key();
    let forgot_pwd_exists: Result<String, RedisError> = redis_conn.get(&key).await;

    if forgot_pwd_exists.is_ok() {
        // this user has requested for a password changed in the last one hour
        return Ok(ApiSuccess::reply_success(Some(MESSAGE.to_string())))
    }

    // At this point, we haven't sent the user a new password in the last 1 hour, and the user exists
    let jwt = ForgotPasswordJwt::new(the_user).encode(&state.app)?;
    redis_conn.set(&key, &jwt).await?;
    redis_conn.expire(&key, MINUTES_60 as usize).await?;

    let mail_type = MailType::ForgotPassword(MailInfo::new(jwt, &state.app.frontend_url));
    Email::new(parsed_email, None, mail_type);

    Ok(ApiSuccess::reply_success(Some(MESSAGE.to_string())))

}