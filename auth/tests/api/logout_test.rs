// let ab = std::mem::size_of::<u32>();
#[cfg(test)]
mod test {
    use auth::helpers::{commons::{RedisKey, RedisPrefix}, jwt_tokens::jwt::{LoginJwt, Jwt}, mails::email::Email};
    use redis::{AsyncCommands};
    use rocket::http::{ContentType, Status};

    use crate::helpers::{app::get_client, utils::{get_email, get_invalid_jwt}, response::{parse_api_response, ResponseType}};

    const LOGOUT: &'static str = "/api/v1/logout";

    #[rocket::async_test]
    async fn test_user_logout() {
        let client = get_client().await;
        let mut redis_conn = client.redis().get_async_connection().await.unwrap();
        let user_id = uuid::Uuid::new_v4();
        let email = Email::parse(get_email()).unwrap();
        // Creates 2 login tokens for this user (use is logged in on 2 different browsers/devices)
        let key_zero= format!("{}aaaaa", RedisKey::new(RedisPrefix::Login, user_id).make_key());
        let jwt_value = LoginJwt::new(email.clone(), user_id, "user_id".to_string(), false).encode(&client.config().app).unwrap();
        let _res: Option<String> = redis_conn.set(&key_zero, &jwt_value).await.unwrap();


        let key_one = format!("{}bbbb", RedisKey::new(RedisPrefix::Login, user_id).make_key());
        let token = LoginJwt::new(email.clone(), user_id, "user_id".to_string(), false).encode(&client.config().app).unwrap();
        let _res: Option<String> = redis_conn.set(&key_one, &token).await.unwrap();


        let req_body = serde_json::json!({
            "token": token,
        }).to_string();

        let response = client.app().post(LOGOUT)
            .header(ContentType::JSON)
            .body(req_body)
            .dispatch().await;

        assert_eq!(&response.status(), &Status::Ok);
        assert_eq!(&response.content_type().unwrap(), &ContentType::JSON);

        if let Ok( res ) =  parse_api_response(response, ResponseType::Success).await {
            assert_eq!(res.status, 200);
            assert_eq!(res.message, "Success");
            client.destrory_db().await;
        } else {
            assert!(false)
        }

        let not_logged_out_device: Option<String> = redis_conn.get(&key_zero).await.unwrap();
        let logged_out_device: Option<String> = redis_conn.get(&key_one).await.unwrap();

        assert!(not_logged_out_device.is_some());
        assert!(logged_out_device.is_none());

        client.destrory_db().await;
        assert!(false);
        // client.drop_db().await;
        client.clean_redis(key_zero.to_string()).await.unwrap();
        client.clean_redis(key_one.to_string()).await.unwrap();
    }

    #[rocket::async_test]
    async fn test_user_logout_with_token_thats_already_removed() {
        let client = get_client().await;
        let mut redis_conn = client.redis().get_async_connection().await.unwrap();
        let user_id = uuid::Uuid::new_v4();
        let email = Email::parse(get_email()).unwrap();
        let jwt_key = RedisKey::new(RedisPrefix::Login, user_id).make_key();
        let jwt_value = LoginJwt::new(email.clone(), user_id, "user_id".to_string(), false).encode(&client.config().app).unwrap();
        let _res: Option<String> = redis_conn.set(&jwt_key, &jwt_value).await.unwrap();
        client.clean_redis(jwt_key).await.unwrap();

        let req_body = serde_json::json!({
            "token": jwt_value,
        }).to_string();

        let response = client.app().post(LOGOUT)
            .header(ContentType::JSON)
            .body(req_body)
            .dispatch().await;

        assert_eq!(&response.status(), &Status::Ok);
        assert_eq!(&response.content_type().unwrap(), &ContentType::JSON);

        if let Ok( res ) =  parse_api_response(response, ResponseType::Success).await {
            assert_eq!(res.status, 200);
            assert_eq!(res.message, "Success");
            client.destrory_db().await;
        } else {
            assert!(false)
        }
    }


    #[rocket::async_test]
    async fn test_user_logout_with_invalid_token() {
        let client = get_client().await;
        
        let req_body = serde_json::json!({
            "token": get_invalid_jwt(),
        }).to_string();

        let response = client.app().post(LOGOUT)
            .header(ContentType::JSON)
            .body(req_body)
            .dispatch().await;

        assert_eq!(&response.status(), &Status::Unauthorized);
        assert_eq!(&response.content_type().unwrap(), &ContentType::JSON);
    }
}