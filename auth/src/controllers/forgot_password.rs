use redis::{RedisError, AsyncCommands};
use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::{settings::config::Settings, helpers::{mail::{Email, MailType, MailInfo}, commons::{ApiResult, MINUTES_60, RedisKey, RedisPrefix}, jwt::{ForgotPasswordJwt, Jwt}}, base_repository::user::DbUser, response::ApiSuccess};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    email: String,
}


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
        return Ok(ApiSuccess::reply_success(Some("Please check your email for the link to reset your password".to_string())))
    }

    let the_user = user.unwrap().get_user().1;
    let mut redis_conn = redis.get_async_connection().await?;
    
    let key = RedisKey::new(RedisPrefix::Forgot, the_user).make_key();

    let forgot_pwd_exists: Result<String, RedisError> = redis_conn.get(&key).await;
    if forgot_pwd_exists.is_ok() {
        // this user has requested for a password changed in the last one
        return Ok(ApiSuccess::reply_success(Some("Please check your email for the link to reset your password".to_string())))
    }
    
    // At this point, we haven't sent the user a new password in the last 1 hour, and the user exists
    let jwt = ForgotPasswordJwt::new(the_user).encode(&state.app)?;
    redis_conn.set(&key, &jwt).await?;
    redis_conn.expire(&key, MINUTES_60 as usize).await?;

    let mail_type = MailType::ForgotPassword(MailInfo::new(jwt, &state.app.frontend_url));
    Email::new(parsed_email, None, mail_type);

    Ok(ApiSuccess::reply_success(Some("Please check your email for the link to reset your password".to_string())))

}