use std::convert::TryInto;
use config::{ConfigBuilder, builder::DefaultState};
use rocket::{fairing::{Fairing, Info, self, Kind}, Rocket, Build};
use serde::{Deserialize};
use dotenv::dotenv;

use crate::settings::{database::DbSettings, email::EmailSettings, app::AppSettings};
use crate::settings::variables::AppEnv;


#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub db: DbSettings,
    pub app: AppSettings,
    pub email: EmailSettings,
}


#[rocket::async_trait]
impl Fairing for Settings {
    fn info(&self) -> Info {
        Info { name: "Verify required Env Variables", kind: Kind::Ignite }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        Ok(rocket)
    }

}


pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    dotenv().ok();
    let app_env: AppEnv = std::env::var("APP_ENV")
    .unwrap_or_else(|_| "local".into())
    .try_into().expect("Failed to parse APP_ENV");
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let mut config_dir = base_path.join("configuration");

    if app_env.to_string().to_lowercase() == "test" {
        config_dir = base_path.join("../configuration");
    }

    let settings = ConfigBuilder::<DefaultState>::default()
        .add_source(config::File::from(config_dir.join("base")).required(true))
        .add_source(config::File::from(config_dir.join(app_env.to_string())).required(true));

    settings.build()?.try_deserialize()
}
