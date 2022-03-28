#[macro_use] extern crate rocket;

use auth::{routes::build, settings::config};


#[launch]
pub async fn rocket() -> _ {
    let config = config::get_configuration();
    use std::env;
    for (key, val) in env::vars() {
        println!("=-0-----------------------------{:#?} {:#?}", key, val);
    }
    println!("THE ERRORS OR SUCCESSS???????? {:#?}", config);
    let config = config.unwrap();
    build(config).await
}
