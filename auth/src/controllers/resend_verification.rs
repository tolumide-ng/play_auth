use rocket::{serde::json::Json, State};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::helpers::requests::AuthHeader;
use crate::settings::config::Settings;
use crate::helpers::commons::{ApiResult, RedisKey, RedisPrefix};
use crate::helpers::mails::email::{Email, MailInfo, MailType};
use crate::helpers::jwt_tokens::jwt::{SignupJwt, Jwt};
use crate::response::ApiSuccess;
use crate::base_repository::user::DbUser;
use crate::errors::app::ApiError;



#[post("/resend_verify")]
pub async fn resend_verification_token(
    pool: &State<Pool<Postgres>>,
    state: &State<Settings>,
    redis: &State<redis::Client>,
    guard: AuthHeader,
) -> ApiResult<Json<ApiSuccess<&'static str>>> {

    let email = Email::parse(guard.email())?;
    let user_id = Uuid::parse_str(&guard.id())?;


    let mut redis_conn = redis.get_async_connection().await?;
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

    return Err(ApiError::AuthorizationError("Invalid token"))
}