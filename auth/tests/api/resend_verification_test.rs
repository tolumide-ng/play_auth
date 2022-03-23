#[cfg(test)]
mod test {
    use redis::AsyncCommands;

    use auth::{helpers::{mails::email::Email, passwords::pwd::Password, test_helpers::get_appsettings, commons::{RedisKey, RedisPrefix}, jwt_tokens::jwt::{SignupJwt, Jwt, LoginJwt}}, base_repository::user::DbUser};
    use rocket::http::{ContentType, Header, Status};
    use crate::helpers::{app::get_client, utils::{get_email, get_pwd, get_invalid_jwt}, response::{ResponseType, parse_api_response}};


    const RESEND: &'static str = "/api/v1/resend_verify";

    #[rocket::async_test]
    async fn test_no_authorization_header() {
        let client = get_client().await;

        let response = client.app().post(RESEND)
            .header(ContentType::JSON)
            .dispatch().await;

        assert_eq!(&response.status(), &Status::Unauthorized);
        assert_eq!(&response.content_type().unwrap(), &ContentType::JSON);
    }

    #[rocket::async_test]
    async fn test_invalid_authorization_header() {
        let client = get_client().await;

        let response = client.app().post(RESEND)
            .header(ContentType::JSON)
            .header(Header::new("Authorization", get_invalid_jwt()))
            .dispatch().await;

        assert_eq!(&response.status(), &Status::Unauthorized);
        assert_eq!(&response.content_type().unwrap(), &ContentType::JSON);


        let res = parse_api_response(response, ResponseType::Error).await.unwrap_err();
        let error = res.error;
        assert_eq!(error.status, 401);
        assert_eq!(error.body, "Authorization header is either missing or invalid");
        assert_eq!(error.message, "Unauthorized");
    }

    #[rocket::async_test]
    async fn test_authorization_token_is_valid_but_user_does_not_exist() {
        // User requests for verification, deletes their account and tries to use the email they got earlier for verification
        let client = get_client().await;
        let email = Email::parse(get_email()).unwrap();
        let pwd = Password::new(get_pwd().to_string(), &get_appsettings()).unwrap();
        let user_id = DbUser::create_user(&client.db(), &email, pwd).await.unwrap();
        let jwt_key = RedisKey::new(RedisPrefix::Login, user_id).make_key();
        let jwt_value = LoginJwt::new(email.clone(), user_id, "user_id".to_string(), false).encode(&client.config().app).unwrap();
        let mut redis_conn = client.redis().get_async_connection().await.unwrap();
        let _res: Option<String> = redis_conn.set(&jwt_key, &jwt_value).await.unwrap();

        let response = client.app().post(RESEND)
            .header(ContentType::JSON)
            .header(Header::new("Authorization", jwt_value))
            .dispatch().await;

        assert_eq!(&response.status(), &Status::Forbidden);
        assert_eq!(&response.content_type().unwrap(), &ContentType::JSON);

        let res = parse_api_response(response, ResponseType::Error).await.unwrap_err();
        let error = res.error;
        assert_eq!(error.status, 403);
        assert_eq!(error.body, "Invalid token");
        assert_eq!(error.message, "Forbidden");
        client.destrory_db().await;
        client.clean_redis(jwt_key).await.unwrap();
    }

    #[rocket::async_test]
    async fn test_authorization_token_is_valid_but_user_is_already_verified() {
        let client = get_client().await;
        let email = Email::parse(get_email()).unwrap();
        let pwd = Password::new(get_pwd().to_string(), &get_appsettings()).unwrap();
        let user_id = DbUser::create_user(&client.db(), &email, pwd).await.unwrap();
        DbUser::verify_user(&client.db(), user_id).await.unwrap();
        let jwt_key = RedisKey::new(RedisPrefix::Login, user_id).make_key();
        let jwt_value = LoginJwt::new(email.clone(), user_id, "user_id".to_string(), false).encode(&client.config().app).unwrap();
        let mut redis_conn = client.redis().get_async_connection().await.unwrap();
        let _res: Option<String> = redis_conn.set(&jwt_key, &jwt_value).await.unwrap();

        let response = client.app().post(RESEND)
            .header(ContentType::JSON)
            .header(Header::new("Authorization", jwt_value))
            .dispatch().await;

        assert_eq!(&response.status(), &Status::Forbidden);
        assert_eq!(&response.content_type().unwrap(), &ContentType::JSON);

        let res = parse_api_response(response, ResponseType::Error).await.unwrap_err();
        let error = res.error;
        assert_eq!(error.status, 403);
        assert_eq!(error.body, "Invalid token");
        assert_eq!(error.message, "Forbidden");
        client.destrory_db().await;
        client.clean_redis(jwt_key).await.unwrap();
    }

    #[rocket::async_test]
    async fn test_resend_verification() {
        let client = get_client().await;
        let email = Email::parse(get_email()).unwrap();
        let pwd = Password::new(get_pwd().to_string(), &get_appsettings()).unwrap();
        let user_id = DbUser::create_user(&client.db(), &email, pwd).await.unwrap();
        let jwt_key = RedisKey::new(RedisPrefix::Login, user_id).make_key();
        let jwt_value = LoginJwt::new(email.clone(), user_id, "user_id".to_string(), false).encode(&client.config().app).unwrap();
        let mut redis_conn = client.redis().get_async_connection().await.unwrap();
        let res: Option<String> = redis_conn.set(&jwt_key, &jwt_value).await.unwrap();

        let response = client.app().post(RESEND)
            .header(ContentType::JSON)
            .header(Header::new("Authorization", jwt_value))
            .dispatch().await;

        assert_eq!(&response.status(), &Status::Ok);
        assert_eq!(&response.content_type().unwrap(), &ContentType::JSON);

        let res = parse_api_response(response, ResponseType::Success).await.unwrap();
        assert_eq!(res.status, 200);
        assert_eq!(res.body, "Please check your email to verify your account");
        assert_eq!(res.message, "Success");
        client.destrory_db().await;
        // client.clean_redis(jwt_key).await.unwrap();
    }
}