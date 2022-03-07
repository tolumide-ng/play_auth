use std::convert::TryInto;
use config::{Environment, ConfigBuilder, builder::DefaultState};
use serde::{Deserialize, Serialize};

use crate::settings::{database::DbSettings, email::EmailSettings, app::AppSettings};

use super::variables::AppEnv;



#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub db: DbSettings,
    pub app: AppSettings,
    pub email: EmailSettings,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // let mut settings = config::Config::default();
    // let mut settings = config::Config::builder();
    let app_env: AppEnv = std::env::var("APP_ENV")
    .unwrap_or_else(|_| "local".into())
    .try_into().expect("Failed to parse APP_ENV");
    
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let config_dir = base_path.join("configuration");


    let settings = ConfigBuilder::<DefaultState>::default()
        .add_source(config::File::from(config_dir.join("base")).required(true))
        .add_source(config::File::from(config_dir.join(app_env.to_string())).required(true));


    settings.build().unwrap().try_deserialize()
}