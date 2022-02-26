use rocket::{Rocket, Build};

use crate::controllers::health_check::health;

pub fn routes () -> Rocket<Build>{
    rocket::build()
        .mount("/", routes![health])
}