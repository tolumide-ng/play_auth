#[macro_use] extern crate rocket;

use auth::controllers::health_check::health;


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![health])
}