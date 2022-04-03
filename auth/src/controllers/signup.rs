use redis::{AsyncCommands};
use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Pool};
use dotenv::dotenv;

use crate::base_repository::user::DbUser;
use crate::helpers::commons::{RedisKey, RedisPrefix, MINUTES_120, ApiResult};
use crate::helpers::jwt_tokens::jwt::{SignupJwt, Jwt};
use crate::helpers::{mails::email::{Email, MailType, MailInfo}, passwords::pwd::Password};
use crate::{response::{ApiSuccess}, errors::app::ApiError, settings::config::Settings};


#[derive(Deserialize, Serialize)]
pub struct User {
    email: String,
    password: String,
}


// todo!() only dispatch an event into the queue when a user has been verified on the verify endpoint (this should be on the verify endpoint)
#[post("/create", data = "<user>")]
pub async fn create(
    user: Json<User>, 
    pool: &State<Pool<Postgres>>, 
    state: &State<Settings>,
    redis: &State<redis::Client>,
) -> ApiResult<Json<ApiSuccess<&'static str>>> {
    dotenv().ok();
    let User {email, password} = user.0;
    let parsed_email = Email::parse(email)?;
    let parsed_pwd = Password::new(password.clone(), &state.app)?;

    let user_already_exists = DbUser::email_exists(pool, &parsed_email).await?;
    println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");

    // let mut redis_conn = redis.get_async_connection().await;
    match redis.get_async_connection().await {
        Ok(mut redis_conn) => {
            if user_already_exists.is_none() {
                println!("!~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~`");
                let user_id = DbUser::create_user(pool, &parsed_email, parsed_pwd).await?;
                let jwt = SignupJwt::new(user_id).encode(&state.app)?;
        
                let key = RedisKey::new(RedisPrefix::Signup, user_id).make_key();
                redis_conn.set(&key, &jwt).await?;
                redis_conn.expire(&key, MINUTES_120 as usize).await?;
                
                let mail_type = MailType::Signup(MailInfo::new(jwt, &state.app.frontend_url));
                Email::new(parsed_email, None, mail_type).send_email(&state.email);
        
                return Ok(ApiSuccess::reply_success(Some("Please check your email to verify your account")));
            }
        }

        Err(e) => {
            println!("THE ERROR!!!!!!!!!!!!!!!!!!!!!!!>>>>>>>>>>>>>>>>>>>>>>>>>>>> {:#?}", e);
        }
    }




    return Err(ApiError::Conflict("Email already exists"));
}
