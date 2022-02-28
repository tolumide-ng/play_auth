use dotenv::dotenv;
use rocket::{fairing::{self, Fairing, Info, Kind}, Rocket, Build};
use serde::Deserialize;
use std::env;


#[derive(Debug, Clone, Deserialize, derive_more::Display)]
pub enum AppEnv {
    #[display(fmt = "local")]
    Local,
    #[display(fmt = "test")]
    Test,
    #[display(fmt = "staging")]
    Staging,
    #[display(fmt = "production")]
    Production
}


#[derive(Debug, Clone, Deserialize, Default)]
pub struct EnvVars {
    pub db_host: String,
    pub db_port: u16,
    pub db_username: String,
    pub db_password: String,
    pub db_name: String,
    pub app_env: String,
    pub db_url: String,
}


impl EnvVars {
    pub fn verify() -> Result<(), String> {
        dotenv().ok();
        let variables = vec!["APP_ENV", "DB_HOST", "DB_PORT", 
            "DB_USERNAME", "DB_PASSWORD", "DB_NAME", "DB_URL",
        ];

        for var in variables {
            if env::var(var).is_err() {
                let err =  format!("Env variable: {:#?} is required", var);
                return Err(err)
            }

            if var == "DB_PORT" && EnvVars::get_var("DB_PORT").parse::<u16>().is_err() {
                return Err("DB_PORT must be a valid port number".to_string())
            }
        }
        Ok(())
    }

    pub fn new() -> Self {
        dotenv().ok();
        if let Err(e) = EnvVars::verify() {
            panic!("{}", e)
        }
        
        Self {
            app_env: Self::get_var("APP_ENV"),
            db_host: Self::get_var("DB_HOST"),
            db_port: Self::get_var("DB_PORT").parse::<u16>().unwrap(),
            db_username: Self::get_var("DB_USERNAME"),
            db_password: Self::get_var("DB_PASSWORD"),
            db_name: Self::get_var("DB_NAME"),
            db_url: Self::get_var("DB_URL"),
        }
    }

    fn get_var(name: &str) -> String {
        env::var(name).unwrap()
    }
}

#[rocket::async_trait]
impl Fairing for EnvVars {
    fn info(&self) -> Info {
        Info { name: "Verify required Env Variables", kind: Kind::Ignite }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        match EnvVars::verify() {
            Ok(_) => Ok(rocket),
            Err(e) => panic!("{}", e)
        }
    }

}