#[macro_use] extern crate rocket;

use auth::{routes::build, settings::config};


#[launch]
pub async fn rocket() -> _ {
    let app_config = config::get_configuration().unwrap();
    build(app_config).await
}
