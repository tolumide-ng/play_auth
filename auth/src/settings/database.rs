use serde::Deserialize;
use sqlx::{postgres::{PgConnectOptions, PgSslMode}};
use serde_aux::field_attributes::deserialize_number_from_string;
use serde_with_expand_env::with_expand_envs;
use std::env;

// use serde::{de::Error, Deserialize, Deserializer};
// use std::fmt::Display;
// use std::str::FromStr;



#[derive(Debug, Clone, Deserialize)]
pub struct DbSettings {
    pub host: String,
    // #[serde(deserialize_with = "deserialize_number_from_string")]
    #[serde(deserialize_with = "with_expand_envs")]
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_name: String,
    pub require_ssl: bool,
    // pub db_url: String,
}

impl DbSettings {
    // for tests?
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };


        PgConnectOptions::new().host(&self.host).username(&self.username).password(&self.password).port(self.port).ssl_mode(ssl_mode)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.database_name)
    }
}