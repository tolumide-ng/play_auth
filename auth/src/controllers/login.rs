use std::collections::HashMap;

use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Pool};

use crate::{helpers::{commons::{ApiResult}, auth::{Password, LoginJwt, Jwt}, mail::Email}, response::ApiSuccess, base_repository::user::DbUser, errors::app::ApiError, settings::{config::Settings}};


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
) -> ApiResult<Json<ApiSuccess<HashMap<&'static str, String>>>> {
    let User { email, password } = user.0;

    let parsed_email = Email::parse(email);

    if parsed_email.is_err() {
        return Err(ApiError::BadRequest("Please provide a valid email address"))
    }

    let valid_email = parsed_email.unwrap();

    let user = DbUser::email_exists(pool, &valid_email).await?;

    if let Some(db_user) = user {
        if Password::is_same(db_user.get_hash(), password) {
            if db_user.is_verified() {
                let info = db_user.get_user();
                let login_jwt = LoginJwt::new(info.0, info.1).encode(&state.app)?;
                let mut body = HashMap::new();
                body.insert("jwt", login_jwt);
                return Ok(ApiSuccess::reply_success(Some(body)))
            }
            return Err(ApiError::UnverifiedAccount)
        }

    }
    return Err(ApiError::AuthenticationError("Email or Password does not match"))
}