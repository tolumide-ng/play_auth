use std::env;
use rocket::local::asynchronous::Client;
use rocket::local::asynchronous::LocalResponse;
use serde_json::Value;

use auth::routes::build;
use auth::settings::config::{get_configuration, Settings};

pub fn get_test_config() -> Settings {
    let config = {
        env::set_var("APP_ENV", "test");
        get_configuration().expect("Failed to read configuration file")
    };
    return config;
}


pub async fn get_client() -> Client {
    let config = get_test_config();
    let app = build(config).await;
    Client::tracked(app).await.expect("Could not create test client")
}


#[derive(serde::Deserialize, Debug)]
pub struct ApiResponse {
    pub status: i32,
    pub body: String,
    pub message: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct ErrorApiResponse {
    pub error: ApiResponse,
}

pub enum ResponseType {
    Success,
    Error,
}

pub async fn parse_api_response(response: LocalResponse<'_>, response_type: ResponseType) -> Result<ApiResponse, ErrorApiResponse> {
    let res = response.into_bytes().await.unwrap();
    let b_res: Value = serde_json::from_slice(&res).unwrap();

    match response_type {
        ResponseType::Success => {
            let body: ApiResponse = serde_json::from_value(b_res).unwrap();
            return Ok(body);
        },
        ResponseType::Error => {
            let body: ErrorApiResponse = serde_json::from_value(b_res).unwrap();
            return Err(body);
        }
    }
}
