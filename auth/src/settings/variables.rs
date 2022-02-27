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


#[derive(Debug, Clone, Deserialize)]
pub struct EnvVars {
    pub db_host: String,
    pub db_port: String,
    pub db_username: String,
    pub db_password: String,
    pub db_name: String,
}


impl EnvVars {
    pub fn verify() -> Result<(), String> {
        dotenv().ok();
        let variables = vec!["APP_ENV", "DB_HOST", "DB_PORT", "DB_USERNAME", "DB_PASSWORD", "DB_NAME"];

        for var in variables {
            if env::var(var).is_err() {
                let err =  format!("Env variable: {:#?} is required", var);
                return Err(err)
            }
        }
        Ok(())
    }

    pub fn new() -> Self {
        dotenv().ok();
        
        Self {
            db_host: Self::get_var("DB_HOST"),
            db_port: Self::get_var("DB_PORT"),
            db_username: Self::get_var("DB_USERNAME"),
            db_password: Self::get_var("DB_PASSWORD"),
            db_name: Self::get_var("DB_NAME"),
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