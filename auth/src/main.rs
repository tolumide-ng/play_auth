#[macro_use] extern crate rocket;

use auth::{routes::routes};


#[launch]
fn rocket() -> _ {
    routes()
}