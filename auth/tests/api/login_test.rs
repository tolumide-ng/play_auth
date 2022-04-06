#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use auth::{helpers::{test_helpers::get_appsettings, passwords::pwd::Password, mails::email::Email, commons::{RedisKey, RedisPrefix}}, base_repository::user::DbUser};
    use rocket::http::{ContentType, Status};
    use serde::Deserialize;
    use serde_json::Value;

    use crate::helpers::{app::get_client, utils::{get_email, get_pwd}, response::{parse_api_response, ResponseType}};
    
    const LOGIN: &'static str = "/api/v1/login";

    #[rocket::async_test]
    pub async fn test_user_login() {
        let client = get_client().await;
        let email = Email::parse(get_email()).unwrap();
        let password = get_pwd().to_string();
        let pwd = Password::new(password.clone(), &get_appsettings()).unwrap();
        let user_id = DbUser::create_user(&client.db(), &email, pwd).await.unwrap();
        let jwt_key = RedisKey::new(RedisPrefix::Login, user_id).make_key();

        let req_body = serde_json::json!({
            "email": email.to_string(),
            "password": password,
        }).to_string();

        let response = client.app().post(LOGIN)
            .header(ContentType::JSON)
            .body(req_body)
            .dispatch().await;

        assert_eq!(&response.status(), &Status::Ok);
        assert_eq!(&response.content_type().unwrap(), &ContentType::JSON);
        #[derive(Debug, Deserialize)]
        struct LoginResponse {
            status: i32,
            message: String,
            body: HashMap<String, String>
        }
        let res = response.into_bytes().await.unwrap();
        let b_res: Value = serde_json::from_slice(&res).unwrap();
        let body: LoginResponse = serde_json::from_value(b_res).unwrap();
        assert_eq!(body.status, 200);
        assert_eq!(body.message, "Success");
        assert!(body.body.get("jwt").is_some());
        assert!(body.body.get("jwt").is_some());
        // client.destrory_db().await;
        client.clean_redis(jwt_key).await.unwrap();
    }

    #[rocket::async_test]
    pub async fn test_user_does_not_exist() {
        let client = get_client().await;
        let email = Email::parse(get_email()).unwrap();

        let req_body = serde_json::json!({
            "email": email.to_string(),
            "password": get_pwd().to_string(),
        }).to_string();

        let response = client.app().post(LOGIN)
            .header(ContentType::JSON)
            .body(req_body)
            .dispatch().await;

        assert_eq!(&response.status(), &Status::Unauthorized);
        assert_eq!(&response.content_type().unwrap(), &ContentType::JSON);

        let res = parse_api_response(response, ResponseType::Error).await.unwrap_err();
        let error = res.error;
        assert_eq!(error.status, 401);
        assert_eq!(error.message, "Unauthorized");
        assert_eq!(error.body, "Email or Password does not match");
    }


     #[rocket::async_test]
    pub async fn test_incorrect_password() {
        let client = get_client().await;
        let email = Email::parse(get_email()).unwrap();
        let pwd = Password::new(get_pwd().to_string(), &get_appsettings()).unwrap();
        let user_id = DbUser::create_user(&client.db(), &email, pwd).await.unwrap();

        let req_body = serde_json::json!({
            "email": email.to_string(),
            "password": "aCOmpleteleyInvalidPassword12890*",
        }).to_string();

        let response = client.app().post(LOGIN)
            .header(ContentType::JSON)
            .body(req_body)
            .dispatch().await;

        assert_eq!(&response.status(), &Status::Unauthorized);
        assert_eq!(&response.content_type().unwrap(), &ContentType::JSON);

        let res = parse_api_response(response, ResponseType::Error).await.unwrap_err();
        let error = res.error;
        assert_eq!(error.status, 401);
        assert_eq!(error.message, "Unauthorized");
        assert_eq!(error.body, "Email or Password does not match");
        // client.destrory_db().await;
        client.clean_redis(user_id.to_string()).await.unwrap()
    }
}