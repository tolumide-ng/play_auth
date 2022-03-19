use sqlx::{Pool, Postgres, types::chrono};
use sqlx::Error::{RowNotFound};
use uuid::Uuid;

use crate::helpers::mail::ValidEmail;
use crate::helpers::pwd::Password;
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
    pub async fn user_exist(pool: &Pool<Postgres>, email: &ValidEmail, username: String) -> DbResult<bool> {
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

    pub async fn create_user(pool: &Pool<Postgres>, email: &ValidEmail, hash: String) -> DbResult<Uuid> {
        let user = sqlx::query!(r#"INSERT INTO play_user (email, hash) VALUES ($1, $2) RETURNING user_id"#, email.to_string(), hash)
            .fetch_one(pool).await;

        println!("WHAT THE RECORD LOOKS LIKE {:#?}", user);

        if let Err(e) = user {
            // todo!() - tracing!
            return Err(ApiError::DatabaseError(e))
        }

        let user_id = user.unwrap().user_id;

        Ok(user_id)
    }

    pub async fn email_exists(pool: &Pool<Postgres>, email: &ValidEmail) -> DbResult<Option<User>> {
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

    //    // r#"UPDATE play_user SET verified=true WHERE user_id=$1 RETURNING *"#
    // pub async fn update_onerrrr(pool: &Pool<Postgres>, target: &'static str, condition_name: &'static str, condition_value: &'static str) -> DbResult<bool> {
    //     let query = format!("UPDATE play_user SET {} WHERE {}=$1 RETURNING *", target, condition_name);
    //     let res = sqlx::query(&query)
    //         .bind(condition_value)
    //         .execute(&*pool).await;

    //     if let Err(e) = res {
    //         return Err(ApiError::DatabaseError(e))
    //     }
        
    //     return Ok(true)
    // }

    pub async fn verify_user(pool: &Pool<Postgres>, user_id: Uuid) -> DbResult<bool> {
        let res = sqlx::query(r#"UPDATE play_user SET verified=true WHERE user_id=$1 RETURNING *"#)
            .bind(user_id)
            .execute(&*pool).await;

        if let Err(e) = res {
            return Err(ApiError::DatabaseError(e))
        }
        
        return Ok(true)
    }

    pub async fn update_pwd(pool: &Pool<Postgres>, password: Password, email: ValidEmail) -> DbResult<bool> {
        let res = sqlx::query(r#"UPDATE play_user SET hash=$1 WHERE email=$2 RETURNING *"#)
            .bind(password.to_string())
            .bind(email.to_string())
            .execute(&*pool).await;

        if let Err(e) = res {
            return Err(ApiError::DatabaseError(e))
        }

        Ok(true)
    }
}