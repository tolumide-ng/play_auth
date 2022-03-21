use std::env;
use auth::routes::get_pool;
use rocket::local::asynchronous::Client;
use redis::{Client as RedisClient};

use auth::routes::build;
use auth::settings::config::{get_configuration, Settings};
use sqlx::Pool;
use sqlx::Postgres;

pub struct TestClient {
    app: Client,
    db: Pool<Postgres>,
    redis: RedisClient,
    config: Settings,
}

impl TestClient {
    pub fn new(app: Client, db: Pool<Postgres>, redis: RedisClient, config: Settings) -> Self {
        Self {app, db, redis, config}
    }

    pub fn app(&self) -> &Client {
        &self.app
    }

    pub fn db(&self) -> &Pool<Postgres> {
        &self.db
    }

    pub fn config(&self) -> &Settings {
        &self.config
    }

    pub fn redis(&self) -> RedisClient {
        let client = self.redis.clone();
        client
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


fn get_test_config() -> Settings {
    let config = {
        env::set_var("APP_ENV", "test");
        get_configuration().expect("Failed to read configuration file")
    };
    return config;
}


pub async fn get_client() -> TestClient {
    let config = get_test_config();
    let db_pool = get_pool(&config.db);
    let redis_client = redis::Client::open(&*config.redis_uri).expect("Unable to establish connection to redis");

    let app = build(config.clone()).await;
    
    TestClient::new(Client::tracked(app).await.expect("Could not create test client"), db_pool, redis_client, config)
}
