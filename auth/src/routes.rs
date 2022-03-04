use std::time::Duration;

use rocket::{Rocket, Build};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

use crate::controllers::{ create, health };
use crate::settings::database::DbSettings;
use crate::settings::variables::EnvVars;

pub async fn routes () -> Rocket<Build>{
    let db_pool = get_pool(DbSettings::new(EnvVars::new()));

    
    rocket::build()
        .attach(EnvVars::new())
        .manage(db_pool)
        .manage(EnvVars::new())
        .mount("/", routes![
            health, 
            create
        ])
}



  pub fn get_pool(config: DbSettings) -> PgPool {
        PgPoolOptions::new()
            .connect_timeout(Duration::from_secs(30))
            .connect_lazy_with(config.with_db())
    }