#[cfg(test)]
mod test {
    const FORGOT: &'static str = "/api/v1/forgot";
    const MESSAGE: &'static str = "Please check your email for the link to reset your password";

    use auth::{helpers::{commons::{RedisKey, RedisPrefix}, mails::email::Email, jwt_tokens::jwt::{ForgotPasswordJwt, Jwt}, passwords::pwd::Password, test_helpers::get_appsettings}, base_repository::user::DbUser};
    use rocket::http::{ContentType, Status};
    use redis::{AsyncCommands};

    use crate::helpers::{app::get_client, response::{parse_api_response, ResponseType}, utils::{get_email, get_pwd, get_invalid_email}};


    #[rocket::async_test]
    async fn test_no_request_body() {
        let client = get_client().await;
        let response = client.app()
            .post(FORGOT)
            .header(ContentType::JSON)
            .dispatch().await;

        assert_eq!(&response.status(), &Status::BadRequest);
        assert_eq!(&response.content_type().unwrap(), &ContentType::JSON);
        let body = parse_api_response(response, ResponseType::Error).await.unwrap_err();
        let error = body.error;
        assert_eq!(error.status, 400);
        assert_eq!(error.body, "Bad request");
        assert_eq!(error.message, "Bad Request");
    }

    #[rocket::async_test]
    async fn test_invalid_password() {
        let client = get_client().await;
        let req_body = serde_json::json!({
            "email": get_invalid_email(),
        }).to_string();

        let response = client.app().post(FORGOT)
            .header(ContentType::JSON)
            .body(req_body)
            .dispatch().await;

        let body = parse_api_response(response, ResponseType::Error).await.unwrap_err();
        let error = body.error;

        assert_eq!(error.status, 400);
        assert_eq!(error.body, "Please provide a valid email address");
        assert_eq!(error.message, "Bad Request");
    }


    #[rocket::async_test]
    async fn test_response_is_success_if_email_does_not_exist() {
        let client = get_client().await;

        let req_body = serde_json::json!({
            "email": get_email(),
        }).to_string();

        let response = client.app().post(FORGOT)
            .header(ContentType::JSON)
            .body(req_body)
            .dispatch().await;

        let body = parse_api_response(response, ResponseType::Success).await.unwrap();
        assert_eq!(body.message, "Success");
        assert_eq!(body.status, 200);
        assert_eq!(body.body, MESSAGE);
    }

    #[rocket::async_test]
    async fn test_new_forgot_password_request() {
        let client = get_client().await;
        let email = Email::parse(get_email()).unwrap();
        let pwd = Password::new(get_pwd().to_string(), &get_appsettings()).unwrap();
        DbUser::create_user(&client.db(), &email, pwd).await.unwrap();

        let req_body = serde_json::json!({
            "email": email.to_string(),
        }).to_string();

        let response = client.app().post(FORGOT)
            .header(ContentType::JSON)
            .body(req_body)
            .dispatch().await;

        let body = parse_api_response(response, ResponseType::Success).await.unwrap();
        assert_eq!(body.message, "Success");
        assert_eq!(body.status, 200);
        assert_eq!(body.body, MESSAGE);

        // client.destrory_db().await;
    }

    #[rocket::async_test]
    async fn test_email_that_last_requested_in_the_last_one_hour() {
        let client = get_client().await;
        let email = Email::parse(get_email()).unwrap();
        let pwd = Password::new(get_pwd().to_string(), &get_appsettings()).unwrap();
        // create the user
        let user_id = DbUser::create_user(&client.db(), &email, pwd).await.unwrap();
        // Simulate that this user has previously requested for password change
        let key = RedisKey::new(RedisPrefix::Forgot, user_id).make_key();
        let jwt = ForgotPasswordJwt::new(user_id).encode(&client.config().app).unwrap();
        let mut redis_conn = client.redis().get_async_connection().await.unwrap();
        let _res: String = redis_conn.set(&key, &jwt).await.unwrap();
        // redis_conn.expire(&key, 60).await.unwrap();

        let req_body = serde_json::json!({
            "email": email.to_string(),
        }).to_string();

        let response = client.app().post(FORGOT)
            .header(ContentType::JSON)
            .body(req_body)
            .dispatch().await;
            
        let body = parse_api_response(response, ResponseType::Success).await.unwrap();
        assert_eq!(body.message, "Success");
        assert_eq!(body.status, 200);
        assert_eq!(body.body, MESSAGE);

        client.clean_redis(key).await.unwrap();
        // client.destrory_db().await;
    }
}