use std::time::Duration;

use rocket::{Rocket, Build, routes, catchers};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use crate::controllers::{ create, health, user_login, forgot,
    verify, reset, logout, resend_verify,
};
use crate::errors::catchers::{bad_request, internal_error, unauthenticated};
use crate::settings::config::Settings;
use crate::settings::database::DbSettings;

pub async fn build (config: Settings) -> Rocket<Build>{
    println!("CALLING BYULD!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! {:#?} \n\n\n\n", config);
    let db_pool = get_pool(&config.db);
    println!("ABOUT TO MIGRATE--ABOUT TO MIGRATE--ABOUT TO MIGRATE--ABOUT TO MIGRATE--ABOUT TO MIGRATE {:#?}", db_pool);
    sqlx::migrate!("./migrations").run(&db_pool).await.unwrap();
    println!("DONE WITH MIGRATIONS**DONE WITH MIGRATIONS**DONE WITH MIGRATIONS**DONE WITH MIGRATIONS**DONE WITH MIGRATIONS");
    let redis_client = redis::Client::open(&*config.redis_uri).expect("Unable to establish connection to redis");

    rocket::build()
        .attach(config.clone())
        .manage(db_pool)
        .manage(redis_client)
        .manage(config)
        .mount("/api/v1", routes![
            health,  create, user_login, forgot, verify, reset, logout,
            resend_verify,
        ])
        .register("/", catchers![bad_request, internal_error, unauthenticated])
        // .register(catchers![not_found])
}



  pub fn get_pool(config: &DbSettings) -> PgPool {
        PgPoolOptions::new()
            .connect_timeout(Duration::from_secs(30))
            .connect_lazy_with(config.with_db())
    }

