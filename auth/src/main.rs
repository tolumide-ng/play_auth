#[macro_use] extern crate rocket;

use auth::controllers::signup::create;


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![create])
}