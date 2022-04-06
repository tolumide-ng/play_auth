use std::env;
use auth::errors::app::ApiError;
use rocket::local::asynchronous::Client;
use redis::{Client as RedisClient};

use auth::routes::build;
use auth::settings::config::{get_configuration, Settings};
use crate::helpers::db::TestDb;
use sqlx::Pool;
use sqlx::Postgres;
use uuid::Uuid;

#[derive(Debug)]
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

    // pub async fn destrory_db(&self) {
    //     TestDb::drop_db(&self.config.db, &self.db).await.unwrap();
    // }

    // pub async fn clean_email_in_db(&self, email: String) {
    //     sqlx::query(r#"DELETE FROM play_user WHERE (email=$1)"#)
    //         .bind(email)
    //         .execute(&self.db).await.unwrap();
    // }

    pub async fn clean_redis(&self, key: String) -> Result<(), ApiError> {
        let mut conn = self.redis().get_async_connection().await?;
        // let ad = &self.config.db.database_name;
        redis::cmd("DEL").arg(&[&key]).query_async(&mut conn).await?;

        Ok(())
    }
}



fn get_test_config() -> Settings {
    let config = {
        env::set_var("APP_ENV", "test");

        let db_name = Uuid::new_v4().to_string();
        let mut app_config = get_configuration().expect("Failed to read configuration file");
        app_config.db.database_name = db_name;
        // env::remove_var("app__db__database_name");
        app_config
    };
    return config;
}


pub async fn get_client() -> TestClient {
    let config = get_test_config();
    // let db_pool = get_pool(&config.db);
    let db_pool = TestDb::create_db(&config.db).await;
    let redis_client = redis::Client::open(&*config.redis_uri).expect("Unable to establish connection to redis");

    let app = build(config.clone()).await;
    
    let test_client = TestClient::new(Client::tracked(app).await.expect("Could not create test client"), db_pool, redis_client, config);

    test_client
}
