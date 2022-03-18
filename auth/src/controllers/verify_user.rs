use jsonwebtoken::TokenData;
use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::{settings::config::Settings, helpers::{jwt::{SignupJwt, Jwt}, commons::ApiResult}, response::ApiSuccess, errors::app::ApiError};


#[derive(Deserialize, Serialize)]
pub struct User {
    token: String,
}

#[put("/verify", data = "<user>" )]
pub async fn verify(
    user: Json<User>,
    state: &State<Settings>,
    pool: &State<Pool<Postgres>>,
    redis: &State<redis::Client>,
) -> ApiResult<Json<ApiSuccess<&'static str>>> {
    let User {token} = user.0;

    let user_token: Result<TokenData<SignupJwt>, _> = SignupJwt::decode(&token, &state.app);

    match user_token {
        Ok(token) => {
            // let mut claims = token.claims.user_id;
            // token.claims.user_id = uuid::Uuid::new_v4();
            
            Err(ApiError::BadRequest("Token is either expired or does not exist"))
        },
        Err(e) => {
            println!("THE ACTUAL ERR {:#?}", e);
            Err(ApiError::BadRequest("Token is either expired or does not exist"))
        }
    }
}