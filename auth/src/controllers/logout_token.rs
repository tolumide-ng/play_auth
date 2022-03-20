use auth_macro::jwt::JwtHelper;
use jsonwebtoken::TokenData;
use redis::{AsyncCommands, AsyncIter, Value, RedisError};
use rocket::{serde::json::Json, State, futures::StreamExt};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::{settings::config::Settings, helpers::{commons::{ApiResult, RedisKey, RedisPrefix}, jwt::{LoginJwt, Jwt}}, response::ApiSuccess};


#[derive(Deserialize, Serialize)]
pub struct User {
    token: String,
}

#[post("/logout", data = "<user>")]
pub async fn logout(
    user: Json<User>,
    pool: &State<Pool<Postgres>>,
    state: &State<Settings>,
    redis: &State<redis::Client>,
) -> ApiResult<Json<ApiSuccess<&'static str>>> {
    let User { token } = user.0;

    let token_data: TokenData<LoginJwt> = LoginJwt::decode(&token, &state.app)?;
    let mut redis_conn = redis.get_async_connection().await?;
    let user_id = token_data.claims.get_user();

    let key = RedisKey::new(RedisPrefix::LoginWithoutTs, user_id).make_key();
    let login_key = format!("{}*", key);

    let mut values = redis_conn.scan_match::<&str, String>(&login_key).await?;
    let mut current_keys: Vec<String> = vec![];

    while let Some(r_key) = values.next().await {
        current_keys.push(r_key);
    }

    let mut key_found = false;
    let mut checked = 0;

    println!("{:#?}", current_keys);


    while checked != current_keys.len() && !key_found {
        let value: Result<Option<String>, RedisError> = redis_conn.get(&current_keys[checked]).await;

        match value {
            Ok(val)  => {
                if let Some(v) = val {
                    if v == token {
                        redis_conn.del(&current_keys[checked]).await?;
                        key_found = true;
                    }
                }
            },
            Err(_e) => {continue}
        }
    
        checked += 1;
    }

    return Ok(ApiSuccess::reply_success(None));

}