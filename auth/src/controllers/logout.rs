use auth_macro::jwt::JwtHelper;
use jsonwebtoken::TokenData;
use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::{settings::config::Settings, helpers::{commons::{ApiResult}, jwt::{LoginJwt, Jwt}}, response::ApiSuccess};


#[derive(Deserialize, Serialize)]
pub struct User {
    token: String,
}

#[post("/post", data = "<user>")]
pub async fn logout_token(
    user: Json<User>,
    pool: &State<Pool<Postgres>>,
    state: &State<Settings>,
    redis: &State<redis::Client>,
) -> ApiResult<Json<ApiSuccess<&'static str>>> {
    let User { token } = user.0;

    let token_data: TokenData<LoginJwt> = LoginJwt::decode(&token, &state.app)?;
    let mut redis_conn = redis.get_async_connection().await?;
    let user_id = token_data.claims.get_user();

    // let key = make_redis_key(prefix, id)

    return Ok(ApiSuccess::reply_success(None));

}