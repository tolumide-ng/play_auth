use sqlx::{Pool, Postgres, error::DatabaseError};

use crate::{helpers::commons::{Str, DbResult}, errors::app::ApiError};


#[derive(Debug)]
pub struct DbUser {
    pub user_id: uuid::Uuid,
    pub email: String,
    pub hash: String,
}

struct Row {
    email: String,
    username: String,
}

impl DbUser {
    pub async fn email_exist(pool: &Pool<Postgres>, email: String, username: String) -> DbResult<bool> {
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

    pub async fn create_user(pool: &Pool<Postgres>, email: String, username: String, hash: String) -> DbResult<bool> {
        let user = sqlx::query!(r#"INSERT INTO play_user (email, username, hash) VALUES ($1, $2, $3)"#, email, username, hash)
            .execute(pool).await;

        if let Err(e) = user {
            // todo!() - tracing!
            return Err(ApiError::DatabaseError(e))
        }

        return Ok(true);
    }
}