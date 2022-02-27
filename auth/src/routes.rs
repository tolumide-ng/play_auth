use rocket::{Rocket, Build};

use crate::controllers::health_check::health;
use crate::settings::variables::EnvVars;

pub fn routes () -> Rocket<Build>{
    rocket::build()
        .attach(EnvVars::new())
        .mount("/", routes![health])
}