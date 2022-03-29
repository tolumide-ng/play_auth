#[macro_use] extern crate rocket;

use auth::{routes::build, settings::config};


#[launch]
pub async fn rocket() -> _ {
    println!("starting the rocket application");
    let config = config::get_configuration().unwrap();
    build(config).await
}
