use rocket::http::{ContentType, Status};

const CREATE: &'static str = "/api/v1/create";


#[cfg(test)]
mod test {
    // use mockall::predicate::*;

    use auth::base_repository::user::DbUser;
    use auth::helpers::mails::email::Email;
    use auth::helpers::passwords::pwd::Password;
    use auth::helpers::test_helpers::get_appsettings;

    use super::*;
    use crate::helpers::app::{get_client};
    use crate::helpers::response::{parse_api_response, ResponseType};
    use crate::helpers::utils::{get_email, get_invalid_email, get_pwd};


    #[rocket::async_test]
    async fn test_invalid_signup_request() {
        let client = get_client().await;
        let response = client.app().post(CREATE).dispatch().await;
        assert_eq!(&response.status(), &Status::BadRequest);
        assert_eq!(&response.content_type().unwrap(), &ContentType::JSON);
    }

    #[rocket::async_test]
    async fn test_signup_with_no_password() {
        let client = get_client().await;
        // let response = client.app().post(CREATE).dispatch().await;
        let email: String = get_email();
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
        let password: &str = get_pwd();

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
        let email: String = get_email();
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
        let invalid_email: String = get_invalid_email();

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
        let email: String = get_email();
        let good_pwd: &str = get_pwd();
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
        let email = Email::parse(get_email()).expect("error creating user email");
        let pwd = Password::new(get_pwd().to_string(), &get_appsettings()).unwrap();

        let req_body = serde_json::json!({
            "email": email.to_string(),
            "password": pwd.to_string(),
        }).to_string();

        DbUser::create_user(&client.db(), &email, pwd).await.expect("Could not create user");        

        let response = client.app().post(CREATE)
            .header(ContentType::JSON)
            .body(req_body).dispatch().await;

        assert_eq!(&response.status(), &Status::Conflict);
        assert_eq!(&response.content_type().unwrap(), &ContentType::JSON);
        let res = parse_api_response(response, ResponseType::Error).await.unwrap_err().error;

        assert_eq!(res.status, 409);
        assert_eq!(res.message, "Conflict");
        assert_eq!(res.body, "Email already exists");

        client.clean_email_in_db(email.to_string()).await;
    }
}