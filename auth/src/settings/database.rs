use serde::Deserialize;
use sqlx::{postgres::{PgConnectOptions, PgSslMode}};

#[derive(Debug, Clone, Deserialize)]
pub struct DbSettings {
    pub host: String, // should be secret
    pub port: u16,
    pub username: String,
    pub password: String, // should be secret
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