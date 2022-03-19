use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::{settings::config::Settings, helpers::{commons::ApiResult, mail::Email, pwd::Password}, response::ApiSuccess};

#[derive(Deserialize, Serialize)]
pub struct User {
    email: String,
    password: String,
    token: String,
}

#[put("/reset", data = "<user>")]
pub async fn reset(
    user: Json<User>,
    pool: &State<Pool<Postgres>>,
    state: &State<Settings>,
    redis: &State<redis::Client>,
) -> ApiResult<Json<ApiSuccess<&'static str>>> {
    let User {email, password, token} = user.0;

    let mut resis_conn = redis.get_async_connection().await?;
    

    let parsed_email = Email::parse(email)?;
    let parsed_password = Password::new(password, &state.app)?;

    Ok(ApiSuccess::reply_success(None))
}