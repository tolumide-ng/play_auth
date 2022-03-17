use fake::Dummy;
use rand::Rng;
use rocket::http::{ContentType, Status};
use rand::seq::SliceRandom;

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
        let response = client.app().post(CREATE).dispatch().await;
        assert_eq!(&response.status(), &Status::BadRequest);
        assert_ne!(&response.content_type().unwrap(), &ContentType::JSON);
    }

    #[rocket::async_test]
    async fn test_signup_with_no_password() {
        let client = get_client().await;
        // let response = client.app().post(CREATE).dispatch().await;
        let email: String = fake::faker::internet::en::SafeEmail().fake();
        let req_body = serde_json::json!({
            "email": email,
        }).to_string();

        let response = client.app().post(CREATE)
            .header(ContentType::JSON)
            .body(req_body).dispatch().await;        
        
        assert_eq!(&response.status(), &Status::UnprocessableEntity);
        assert_eq!(&response.content_type().unwrap(), &ContentType::HTML);
        client.clean_email_in_db(email).await;
    }

    #[rocket::async_test]
    async fn test_signup_with_no_email() {
        let client = get_client().await;
        let password: &str = Pwd.fake();

        let req_body = serde_json::json!({
            "password": password,
        }).to_string();

        let response = client.app().post(CREATE)
            .header(ContentType::JSON)
            .body(req_body).dispatch().await;        
        
        assert_eq!(&response.status(), &Status::UnprocessableEntity);
        assert_eq!(&response.content_type().unwrap(), &ContentType::HTML);
    }

    #[rocket::async_test]
    async fn test_does_not_provide_valid_password() {
        let client = get_client().await;
        let email: String = fake::faker::internet::en::SafeEmail().fake();
        let password = "good_pwd";

        let req_body = serde_json::json!({
            "email": email,
            "password": password,
        }).to_string();

        let response = client.app().post(CREATE)
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
        
        client.clean_email_in_db(email).await;
    }


    #[rocket::async_test]
    async fn test_provides_invalid_email() {
        let client = get_client().await;
        let invalid_email: String = fake::faker::internet::en::Username().fake();

        let req_body = serde_json::json!({
            "email": invalid_email,
            "password": "good_pwd",
        }).to_string();

        let response = client.app().post(CREATE)
            .header(ContentType::JSON)
            .body(req_body).dispatch().await;

        assert_eq!(&response.status(), &Status::BadRequest);
        assert_eq!(&response.content_type().unwrap(), &ContentType::JSON);

        if let Err(res) = parse_api_response(response, ResponseType::Error).await {
            let body = res.error;
            
            assert_eq!(body.status, 400);
            assert!(body.body.contains("Please provide a valid email address"));
            assert_eq!(body.message, "Bad Request".to_string());
        } else {
            assert!(false);
        }
        
        client.clean_email_in_db(invalid_email).await;
    }

    #[rocket::async_test]
    async fn test_crates_valid_user() {
        let client = get_client().await;
        let email: String = fake::faker::internet::en::SafeEmail().fake();
        let good_pwd: &str = Pwd.fake();
        let req_body = serde_json::json!({
            "email": email,
            "password": good_pwd,
        }).to_string();

        let response = client.app().post(CREATE)
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

        client.clean_email_in_db(email).await;
    }

    #[rocket::async_test]
    async fn test_fails_when_email_already_exists() {
        let client = get_client().await;
        let email: String = fake::faker::internet::en::SafeEmail().fake();
        let pwd: &str = Pwd.fake();

        let req_body = serde_json::json!({
            "email": email,
            "password": pwd,
        }).to_string();

        sqlx::query!(r#"INSERT INTO play_user (email, hash) VALUES ($1, $2) RETURNING user_id"#, email.to_string(), pwd)
        .fetch_one(client.db()).await.unwrap();
        

        let response = client.app().post(CREATE)
            .header(ContentType::JSON)
            .body(req_body).dispatch().await;

        assert_eq!(&response.status(), &Status::Conflict);
        assert_eq!(&response.content_type().unwrap(), &ContentType::JSON);
        let res = parse_api_response(response, ResponseType::Error).await.unwrap_err().error;

        assert_eq!(res.status, 409);
        assert_eq!(res.message, "Conflict");
        assert_eq!(res.body, "Email already exists");

        client.clean_email_in_db(email).await;
    }
}