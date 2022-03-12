use auth::routes::build;
use auth::settings::config;
// use rocket::local::Client;
use rocket::http::{ContentType, Status};
use rocket::launch;

// #[launch]
// pub async fn speak() -> _ {
//     let config = config::get_configuration().unwrap();
//     build(config).await
// }

#[launch]
pub async fn rocket() -> _ {
    let config = config::get_configuration().unwrap();
    build(config).await
}



#[cfg(test)]
mod test {
    use super::*;
    use crate::helpers::app::{get_test_config, get_client};

    #[rocket::async_test]
    async fn test_invalid_signup_request() {
        let client = get_client().await;
        let response = client.post("/api/v1/create").dispatch().await;
        assert_eq!(&response.status(), &Status::BadRequest);
        assert_ne!(&response.content_type().unwrap(), &ContentType::JSON);
    }
    // use 
    // use crate::
    // use auth::src::main;
}