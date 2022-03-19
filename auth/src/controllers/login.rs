use std::collections::HashMap;

use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Pool};

use crate::helpers::jwt::{LoginJwt, Jwt};
use crate::response::ApiSuccess;
use crate::base_repository::user::DbUser;
use crate::errors::app::ApiError;
use crate:: settings::{config::Settings};
use crate::helpers::{commons::{ApiResult}, pwd::Password, mail::Email};


#[derive(Deserialize, Serialize)]
pub struct User {
    email: String,
    password: String,
}


#[post("/login", data = "<user>")]
pub async fn user_login(
    user: Json<User>,
    pool: &State<Pool<Postgres>>,
    state: &State<Settings>,
    redis: &State<redis::Client>,
) -> ApiResult<Json<ApiSuccess<HashMap<&'static str, String>>>> {
    let User { email, password } = user.0;

    let parsed_email = Email::parse(email)?;

    let user = DbUser::email_exists(pool, &parsed_email).await?;


    if let Some(db_user) = user {
        if Password::is_same(db_user.get_hash(), password) {
            let info: (String, uuid::Uuid) = db_user.get_user();
            let login_jwt = LoginJwt::new(parsed_email, info.1, db_user.is_verified()).encode(&state.app)?;

            let mut redis_conn = redis.get_async_connection().await?;
            // redis_conn.set

            let mut body = HashMap::new();
            body.insert("jwt", login_jwt);
            body.insert("verified", db_user.is_verified().to_string());
            return Ok(ApiSuccess::reply_success(Some(body)))
        }
    }

    return Err(ApiError::AuthenticationError("Email or Password does not match"))
}