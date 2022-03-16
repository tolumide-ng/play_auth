use auth::helpers::auth::Password;
use auth::routes::build;
use auth::settings::config;
use fake::Dummy;
use rand::Rng;
use rocket::http::{ContentType, Status};
use rand::seq::SliceRandom;
use rocket::tokio::io::AsyncReadExt;
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

struct Pwd;

impl Dummy<Pwd> for &'static str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Pwd, rng: &mut R) -> &'static str {
        const VALID_PWDS: &[&str] = &["Pwd|#89*jdssd", "Anot2143@!jjdsk"];
        VALID_PWDS.choose(rng).unwrap()
    }
}




#[cfg(test)]
mod test {
    use fake::Fake;
    // use mockall::*;
    use mockall::predicate::*;
    use serde::Deserialize;
    use serde_json::Value;

    use super::*;
    use crate::helpers::app::{get_client};


    #[derive(serde::Deserialize, Debug)]
    struct SuccessResponse {
        code: i32,
        body: String,
        message: String,
    }



    #[rocket::async_test]
    async fn test_invalid_signup_request() {
        let client = get_client().await;
        let response = client.post(CREATE).dispatch().await;
        assert_eq!(&response.status(), &Status::BadRequest);
        assert_ne!(&response.content_type().unwrap(), &ContentType::JSON);
    }

    // #[rocket::async_test]
    // async fn test_does_not_provide_password() {
    //     let client = get_client().await;
    //     let response = client.post(CREATE)
    //         .header(ContentType::JSON)
    //         .body(r#"{ "email": "sample@email.com", password: "APass9065#*" }"#).dispatch().await;

    //     println!("THE RESPONSE {:#?}", response.into_string().await.unwrap());
    //     assert_eq!(1, 2)
    // }

    #[rocket::async_test]
    async fn test_crates_valid_user() {
        // PLEASE STUB EMAIL CLIENT
        let client = get_client().await;
        let email: String = fake::faker::internet::en::SafeEmail().fake();
        let good_pwd: &str = Pwd.fake();
        // let req_body = format!(r#"{{"email": {}; "password": {}}}"#, email, good_pwd);
        let req_body = serde_json::json!({
            "email": email,
            "password": good_pwd,
        }).to_string();

        println!("THE BODY {:#?}", req_body);

        let response = client.post(CREATE)
            .header(ContentType::JSON)
            .body(req_body).dispatch().await;

        // println!(":::::::::::: RESPONSE.>>>>>>>>>>>>>>>>>>> {:#?}", response.into_string().await.unwrap());
        assert_eq!(&response.status(), &Status::Ok);
        assert_eq!(&response.content_type().unwrap(), &ContentType::JSON);
        // let res = response.into_json::<SuccessResponse>().await;
        let res = response.into_bytes().await.unwrap();
        let bres: Value = serde_json::from_slice(&res).unwrap();
        let body: SuccessResponse = serde_json::from_value(bres).unwrap();

        assert_eq!(body.code, 200);
        assert!(body.body.contains("check your email"));
        assert_eq!(body.message, "Success".to_string());
    }


}