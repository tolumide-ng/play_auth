#[cfg(test)]
mod test {
    use redis::{AsyncCommands};

    use auth::{helpers::{mails::email::Email, commons::{RedisKey, RedisPrefix}, passwords::pwd::Password, test_helpers::get_appsettings, jwt_tokens::jwt::{SignupJwt, Jwt}}, base_repository::user::DbUser};
    use rocket::http::{ContentType, Status};
    use crate::helpers::{app::get_client, utils::{get_email, get_pwd, get_invalid_jwt}, response::{parse_api_response, ResponseType}};

    const VERIFY: &'static str = "/api/v1/verify";


    #[rocket::async_test]
    async fn test_invalid_request() {
        let client = get_client().await;

        let response = client.app().patch(VERIFY)
            .header(ContentType::JSON)
            .dispatch().await;

        assert_eq!(&response.status(), &Status::BadRequest);
        assert_eq!(&response.content_type().unwrap(), &ContentType::JSON);
    }

    #[rocket::async_test]
    async fn test_existing_user_with_invalid_token() {
        let client = get_client().await;
        let email = Email::parse(get_email()).unwrap();
        let pwd = Password::new(get_pwd().to_string(), &get_appsettings()).unwrap();
        DbUser::create_user(&client.db(), &email, pwd).await.unwrap();

        let req_body = serde_json::json!({
            "token": get_invalid_jwt(),
        }).to_string();

        let response = client.app().patch(VERIFY)
            .header(ContentType::JSON)
            .body(req_body)
            .dispatch().await;

        assert_eq!(&response.status(), &Status::Unauthorized);
        assert_eq!(&response.content_type().unwrap(), &ContentType::JSON);

        let res = parse_api_response(response, ResponseType::Error).await.unwrap_err();
        let error = res.error;
        assert_eq!(error.status, 401);
        assert_eq!(error.body, "Token is either expired or invalid");
        assert_eq!(error.message, "Unauthorized");
        client.destrory_db().await;
    }

    #[rocket::async_test]
    async fn test_valid_token_that_wasnt_issued_by_the_application() {
        let client = get_client().await;
        let email = Email::parse(get_email()).unwrap();
        let pwd = Password::new(get_pwd().to_string(), &get_appsettings()).unwrap();
        let user_id = DbUser::create_user(&client.db(), &email, pwd).await.unwrap();
        let jwt_value = SignupJwt::new(user_id).encode(&client.config().app).unwrap();

        let req_body = serde_json::json!({
            "token": jwt_value,
        }).to_string();

        let response = client.app().patch(VERIFY)
            .header(ContentType::JSON)
            .body(req_body)
            .dispatch().await;

        assert_eq!(&response.status(), &Status::Unauthorized);
        assert_eq!(&response.content_type().unwrap(), &ContentType::JSON);

        let res = parse_api_response(response, ResponseType::Error).await.unwrap_err();
        let error = res.error;
        assert_eq!(error.status, 401);
        assert_eq!(error.body, "Token is either expired or invalid");
        assert_eq!(error.message, "Unauthorized");
        client.destrory_db().await;
    }


    #[rocket::async_test]
    async fn test_user_with_valid_token() {
        let client = get_client().await;
        let email = Email::parse(get_email()).unwrap();
        let pwd = Password::new(get_pwd().to_string(), &get_appsettings()).unwrap();
        let user_id = DbUser::create_user(&client.db(), &email, pwd).await.unwrap();
        let jwt_key = RedisKey::new(RedisPrefix::Signup, user_id).make_key();
        let jwt_value = SignupJwt::new(user_id).encode(&client.config().app).unwrap();
        let mut redis_conn = client.redis().get_async_connection().await.unwrap();
        let _res: Option<String> = redis_conn.set(&jwt_key, &jwt_value).await.unwrap();
        let req_body = serde_json::json!({
            "token": jwt_value,
        }).to_string();

        let response = client.app().patch(VERIFY)
            .header(ContentType::JSON)
            .body(req_body)
            .dispatch().await;

        assert_eq!(&response.status(), &Status::Ok);
        assert_eq!(&response.content_type().unwrap(), &ContentType::JSON);

        let res = parse_api_response(response, ResponseType::Success).await.unwrap();
        assert_eq!(res.status, 200);
        assert_eq!(res.body, "verified");
        assert_eq!(res.message, "Success");
        client.destrory_db().await;
        client.clean_redis(jwt_key).await.unwrap();
    }
}