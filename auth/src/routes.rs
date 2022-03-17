use std::time::Duration;

use rocket::{Rocket, Build};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use crate::controllers::{ create, health, user_login };
use crate::settings::config::Settings;
use crate::settings::database::DbSettings;

pub async fn build (config: Settings) -> Rocket<Build>{
    let db_pool = get_pool(&config.db);

    // println!("{{{{{{{{|||||||||||||||||| {:#?}", config);

    rocket::build()
        .attach(config.clone())
        .manage(db_pool)
        .manage(config)
        .mount("/api/v1", routes![
            health,  create, user_login
        ])
        // .register(catchers![not_found])
}



  pub fn get_pool(config: &DbSettings) -> PgPool {
        PgPoolOptions::new()
            .connect_timeout(Duration::from_secs(30))
            .connect_lazy_with(config.with_db())
    }

