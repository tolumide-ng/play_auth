use std::env;
use auth::routes::get_pool;
use rocket::local::asynchronous::Client;
use rocket::local::asynchronous::LocalResponse;
use serde_json::Value;

use auth::routes::build;
use auth::settings::config::{get_configuration, Settings};
use sqlx::Pool;
use sqlx::Postgres;

pub struct TestClient {
    app: Client,
    db: Pool<Postgres>,
}

impl TestClient {
    pub fn new(app: Client, db: Pool<Postgres>) -> Self {
        Self {app, db}
    }

    pub fn app(&self) -> &Client {
        &self.app
    }

    pub fn db(&self) -> &Pool<Postgres> {
        &self.db
    }

    pub async fn clean_db(&self) {
        sqlx::query(r#"DELETE FROM play_user"#).execute(&self.db).await.unwrap();
    }

    pub async fn clean_email_in_db(&self, email: String) {
        sqlx::query(r#"DELETE FROM play_user WHERE (email=$1)"#)
            .bind(email)
            .execute(&self.db).await.unwrap();
    }
}


pub fn get_test_config() -> Settings {
    let config = {
        env::set_var("APP_ENV", "test");
        get_configuration().expect("Failed to read configuration file")
    };
    return config;
}


pub async fn get_client() -> TestClient {
    let config = get_test_config();
    let db_pool = get_pool(&config.db);
    let app = build(config).await;
    
    TestClient::new(Client::tracked(app).await.expect("Could not create test client"), db_pool)
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
