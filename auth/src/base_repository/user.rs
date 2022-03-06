use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{helpers::commons::{Str, DbResult}, errors::app::ApiError};


#[derive(Debug)]
pub struct DbUser;

pub struct User {
    user_id: Uuid,
    hash: String,
    email: String,
    verified: bool,
}

impl User {
    pub fn get_hash(&self) -> String {
        self.hash
    }

    pub fn get_user(&self) -> (String, Uuid) {
        (self.email, self.user_id)
    }

    pub fn is_verified(&self) -> bool {
        self.verified
    }

}


impl DbUser {
    pub async fn user_exist(pool: &Pool<Postgres>, email: String, username: String) -> DbResult<bool> {
        let user = sqlx::query!(r#"SELECT email FROM play_user WHERE (email = $1) OR (username = $2)"#, email, username)
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

    pub async fn create_user(pool: &Pool<Postgres>, email: String, hash: String) -> DbResult<bool> {
        let user = sqlx::query!(r#"INSERT INTO play_user (email, hash) VALUES ($1, $2, $3) RETURNING user_id"#, email, hash)
            .fetch_one(pool).await;

        println!("THE INSERTED {:#?}", user);

        if let Err(e) = user {
            // todo!() - tracing!
            return Err(ApiError::DatabaseError(e))
        }

        return Ok(true);
    }

    pub async fn email_exists(pool: &Pool<Postgres>, email: String) -> DbResult<Option<User>> {
        let user = sqlx::query_as!(User, r#"SELECT * FROM play_user WHERE (email = $1)"#)
            .fetch_one(pool)
            .await;

        println!("DB RESPONSE {:#?}", user);
        
        match user {
            Ok(the_user) => {
                println!("THE RECEIVED USER {:#?}", the_user);
                if the_user.is_some() {
                    return Ok(Some(the_user));
                }
                return Ok(the_user)
            },
            Err(e) => {
                return Err(ApiError::DatabaseError(e))
            }
        }
    }
}