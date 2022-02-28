use sqlx::postgres::{PgConnectOptions, PgSslMode};

use super::variables::{EnvVars, AppEnv};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DbSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

impl DbSettings {
    pub fn new(vars: EnvVars) -> Self {
        let mut require_ssl = true;

        if [AppEnv::Local.to_string(), AppEnv::Test.to_string()].contains(&vars.app_env) {
            require_ssl = false;
        }

        let EnvVars { db_host, db_port, db_username, db_password, db_name, .. } = vars;

        Self {
            username: db_username,
            password: db_password,
            port: db_port,
            host: db_host,
            database_name: db_name,
            require_ssl,
        }
    }

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