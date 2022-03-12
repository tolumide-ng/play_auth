use std::env;
use rocket::local::asynchronous::Client;

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