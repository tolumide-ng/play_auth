use std::time::Duration;

use rocket::{Rocket, Build};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

use crate::controllers::{ create, health, user_login };
use crate::settings::database::DbSettings;
use crate::settings::variables::EnvVars;

pub async fn routes () -> Rocket<Build>{
    let db_pool = get_pool(DbSettings::new(EnvVars::new()));

    
    rocket::build()
        .attach(EnvVars::new_with_verify())
        .manage(db_pool)
        .manage(EnvVars::new())
        .mount("/", routes![
            health,  create, user_login
        ])
        // .register(catchers![not_found])
}



  pub fn get_pool(config: DbSettings) -> PgPool {
        PgPoolOptions::new()
            .connect_timeout(Duration::from_secs(30))
            .connect_lazy_with(config.with_db())
    }