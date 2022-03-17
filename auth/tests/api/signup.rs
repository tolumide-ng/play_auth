use fake::Dummy;
use rand::Rng;
use rocket::http::{ContentType, Status};
use rand::seq::SliceRandom;

// #[launch]
// pub async fn rocket() -> _ {
//     let config = config::get_configuration().unwrap();
//     build(config).await
// }

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
    use mockall::predicate::*;

    use super::*;
    use crate::helpers::app::{get_client, parse_api_response};
    use crate::helpers::app::ResponseType;


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
        let email: String = fake::faker::internet::en::SafeEmail().fake();
        let req_body = serde_json::json!({
            "email": email,
            "password": "good_pwd",
        }).to_string();

        let response = client.post(CREATE)
            .header(ContentType::JSON)
            .body(req_body).dispatch().await;

        assert_eq!(&response.status(), &Status::BadRequest);
        assert_eq!(&response.content_type().unwrap(), &ContentType::JSON);

        if let Err(res) = parse_api_response(response, ResponseType::Error).await {
            let body = res.error;

            assert_eq!(body.status, 400);
            assert!(body.body.contains("Password must be atleast 8 characters long"));
            assert_eq!(body.message, "Bad Request".to_string());
        } else {
            assert!(false);
        }
    }

    #[rocket::async_test]
    async fn test_crates_valid_user() {
        // PLEASE STUB EMAIL CLIENT
        let client = get_client().await;
        let email: String = fake::faker::internet::en::SafeEmail().fake();
        let good_pwd: &str = Pwd.fake();
        let req_body = serde_json::json!({
            "email": email,
            "password": good_pwd,
        }).to_string();

        println!("THE BODY {:#?}", req_body);

        let response = client.post(CREATE)
            .header(ContentType::JSON)
            .body(req_body).dispatch().await;

        assert_eq!(&response.status(), &Status::Ok);
        assert_eq!(&response.content_type().unwrap(), &ContentType::JSON);
        if let Ok(body) = parse_api_response(response, ResponseType::Success).await {
            assert_eq!(body.status, 200);
            assert!(body.body.contains("check your email"));
            assert_eq!(body.message, "Success".to_string());
        } else {
            assert!(false)
        }
    }
}