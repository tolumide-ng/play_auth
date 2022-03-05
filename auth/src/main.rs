#[macro_use] extern crate rocket;

use auth::{routes::routes};


#[launch]
async fn rocket() -> _ {
    routes().await
}