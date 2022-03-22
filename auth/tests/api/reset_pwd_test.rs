const RESET: &'static str = "/api/v1/reset";

#[cfg(test)]
mod test {
    use redis::{aio::Connection, AsyncCommands};
    use rocket::http::{ContentType, Header, Status};
    use uuid::Uuid;
    
    use auth::helpers::commons::RedisKey;
    use auth::helpers::jwt_tokens::jwt::{ForgotPasswordJwt, Jwt};
    use crate::helpers::response::{parse_api_response, ResponseType};
    use crate::helpers::utils::{get_email, get_pwd, get_invalid_jwt};
    use crate::helpers::app::{get_client};

    use super::{RESET};


    #[rocket::async_test]
    async fn test_invalid_authorization_header() {
        let client = get_client().await;
        let response = client.app().patch(RESET)
            .header(ContentType::JSON)
            .header(Header::new("Authorization", get_invalid_jwt()))
            .dispatch().await;

        assert_eq!(&response.status(), &Status::Unauthorized);
        assert_eq!(&response.content_type().unwrap(), &ContentType::JSON);
    }

    #[rocket::async_test]
    async fn test_no_authorization_header() {
        let client = get_client().await;
        let response = client.app().patch(RESET)
            .header(ContentType::JSON)
            .dispatch().await;

        assert_eq!(&response.status(), &Status::Unauthorized);
        assert_eq!(&response.content_type().unwrap(), &ContentType::JSON);

        if let Err(res) = parse_api_response(response, ResponseType::Error).await {
            let body = res.error;

            assert_eq!(body.status, 401);
            assert!(body.body.contains("Authorization header is either missing or invalid"));
            assert_eq!(body.message, "Unauthorized".to_string());
        } else {
            assert!(false)
        }
    }

    #[rocket::async_test]
    async fn test_reset_pwd_success() {
        let client = get_client().await;
        let user_id = Uuid::new_v4();
        let jwt = ForgotPasswordJwt::new(user_id).encode(&client.config().app).unwrap();
        let mut redis_conn: Connection = client.redis().get_async_connection().await.unwrap();
        let key = RedisKey::new(auth::helpers::commons::RedisPrefix::Forgot, user_id).make_key();
        let _saved: String = redis_conn.set(&key, &jwt).await.unwrap();
        let _exp: () = redis_conn.expire(&key, 60).await.unwrap();

        let req_body = serde_json::json!({
            "email": get_email(),
            "password": get_pwd(),
        }).to_string();

        let response = client.app().patch(RESET)
            .header(ContentType::JSON)
            .header(Header::new("Authorization", jwt))
            .body(req_body)
            .dispatch().await;

        assert_eq!(&response.status(), &Status::Ok);
        assert_eq!(&response.content_type().unwrap(), &ContentType::JSON);

        if let Ok(res) = parse_api_response(response, ResponseType::Success).await {
            let body = res;
            assert_eq!(body.body, "password reset successful");
            assert_eq!(body.message, "Success");
            assert_eq!(body.status, 200);
        } else {
            assert!(false);
        }
    }
}