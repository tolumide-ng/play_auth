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

const CREATE: &'static str = "/api/v1/create";

#[cfg(test)]
mod test {
    use super::*;
    use crate::helpers::app::{get_test_config, get_client};

    #[rocket::async_test]
    async fn test_invalid_signup_request() {
        let client = get_client().await;
        let response = client.post(CREATE).dispatch().await;
        assert_eq!(&response.status(), &Status::BadRequest);
        assert_ne!(&response.content_type().unwrap(), &ContentType::JSON);
    }

    #[rocket::async_test]
    async fn test_does_not_provide_password() {
        let client = get_client().await;
        let response = client.post(CREATE)
            .header(ContentType::JSON)
            .body(r#"{ "email": "sample@email.com", password: "APass9065#*" }"#).dispatch().await;

        println!("THE RESPONSE {:#?}", response.into_string().await.unwrap());
        assert_eq!(1, 2)
    }
}