use sqlx::{Pool, Postgres, types::chrono};
use sqlx::Error::{RowNotFound};
use uuid::Uuid;

use crate::helpers::mail::ValidEmail;
use crate::{helpers::commons::{DbResult}, errors::app::ApiError};


#[derive(Debug)]
pub struct DbUser;

#[derive(Debug)]
pub struct User {
    user_id: Uuid,
    hash: String,
    email: String,
    verified: bool,
    username: Option<String>,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}

impl User {
    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    pub fn get_user(&self) -> (String, Uuid) {
        (self.email.clone(), self.user_id.clone())
    }

    pub fn is_verified(&self) -> bool {
        self.verified
    }

}


impl DbUser {
    pub async fn user_exist(pool: &Pool<Postgres>, email: ValidEmail, username: String) -> DbResult<bool> {
        let user = sqlx::query!(r#"SELECT email FROM play_user WHERE (email = $1) OR (username = $2)"#, email.to_string(), username)
            .fetch_optional(pool)
            .await;

        match user {
            Ok(exists) => { Ok(exists.is_some()) }
            Err(e) => {
                // tracing!
                Err(ApiError::DatabaseError(e))
            }
        }
    }

    pub async fn create_user(pool: &Pool<Postgres>, email: ValidEmail, hash: String) -> DbResult<bool> {
        let user = sqlx::query!(r#"INSERT INTO play_user (email, hash) VALUES ($1, $2) RETURNING user_id"#, email.to_string(), hash)
            .fetch_one(pool).await;

        if let Err(e) = user {
            // todo!() - tracing!
            return Err(ApiError::DatabaseError(e))
        }

        return Ok(true);
    }

    pub async fn email_exists(pool: &Pool<Postgres>, email: ValidEmail) -> DbResult<Option<User>> {
        use dotenv::dotenv;
        dotenv().ok();
        let res = sqlx::query_as!(User, r#"SELECT * FROM play_user WHERE (email = $1)"#, email.to_string())
            .fetch_one(pool)
            .await;

        if let Err(e) = res {
            return match e {
                RowNotFound => {Ok(None)},
                _ => {Err(ApiError::DatabaseError(e))}
            }
        }

        Ok(Some(res.unwrap()))
    }
}