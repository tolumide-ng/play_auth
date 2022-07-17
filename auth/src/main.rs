#[macro_use] extern crate rocket;

use auth::{routes::build, settings::config};


#[launch]
pub async fn rocket() -> _ {
    let app_config = config::get_configuration();
    println!("THE APP CONFIGURATION---------------------------------------------------------- {:#?}", app_config);
    let app_config = app_config.unwrap();
    build(app_config).await

    
    // if let Ok(app_config) = config::get_configuration {
    //     build(app_config).await
    // };
}
