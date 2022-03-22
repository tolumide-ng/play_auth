#[cfg(test)]
mod test {
    use redis::AsyncCommands;

    use auth::{helpers::{mails::email::Email, passwords::pwd::Password, test_helpers::get_appsettings, commons::{RedisKey, RedisPrefix}, jwt_tokens::jwt::{SignupJwt, Jwt, LoginJwt}}, base_repository::user::DbUser};
    use rocket::http::{ContentType, Header, Status};
    use crate::helpers::{app::get_client, utils::{get_email, get_pwd}, response::{ResponseType, parse_api_response}};


    const RESEND: &'static str = "/api/v1/resend_verify";

    #[rocket::async_test]
    async fn test_resend_verification() {
        let client = get_client().await;
        let email = Email::parse(get_email()).unwrap();
        let pwd = Password::new(get_pwd().to_string(), &get_appsettings()).unwrap();
        let user_id = DbUser::create_user(&client.db(), &email, pwd).await.unwrap();
        let jwt_key = RedisKey::new(RedisPrefix::Login, user_id).make_key();
        let jwt_value = LoginJwt::new(email, user_id, "user_id".to_string(), false).encode(&client.config().app).unwrap();
        let mut redis_conn = client.redis().get_async_connection().await.unwrap();
        let _res: Option<String> = redis_conn.set(&jwt_key, &jwt_value).await.unwrap();
        let req_body = serde_json::json!({
            "token": jwt_value,
        }).to_string();

        let response = client.app().post(RESEND)
            .header(ContentType::JSON)
            .header(Header::new("Authorization", jwt_value))
            .body(req_body)
            .dispatch().await;

        assert_eq!(&response.status(), &Status::Ok);
        assert_eq!(&response.content_type().unwrap(), &ContentType::JSON);

        let res = parse_api_response(response, ResponseType::Success).await.unwrap();
        assert_eq!(res.status, 200);
        assert_eq!(res.body, "Please check your email to verify your account");
        assert_eq!(res.message, "Success");
    }
}