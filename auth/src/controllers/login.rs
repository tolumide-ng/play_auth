use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Pool};

use crate::{helpers::{commons::{ApiResult, Str}, auth::{Password, LoginJwt}}, response::ApiSuccess, base_repository::user::DbUser, errors::app::ApiError};


#[derive(Deserialize, Serialize)]
pub struct User {
    email: String,
    password: String,
}

#[post("/login", data = "<user>")]
pub async fn user_login(
    user: Json<User>,
    pool: &State<Pool<Postgres>>,
) -> ApiResult<Json<ApiSuccess<Str>>> {
    let User { email, password } = user.0;

    let user = DbUser::email_exists(pool, email).await?;

    if let Some(db_user) = user {
        // let User { email, password }
        if Password::is_same(db_user.get_hash(), password) {
            if db_user.is_verified() {
                let info = db_user.get_user();
                let loginJwt = LoginJwt::new(info.0, info.1);
                // return Ok(ApiSuccess::reply_success(body))
                // let jwt = Login
            }
            return Err(ApiError::UnverifiedAccount)
        }

    }


    // return Ok(ApiSuccess::reply_success(Some("Please check your email to verify your account")));
    return Err(ApiError::AuthenticationError("Email or Password does not match"))
}