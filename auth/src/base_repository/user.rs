use sqlx::{Pool, Postgres};

use crate::helpers::TResult;
// use anyhow::Result;


#[derive(Debug)]
pub struct User {
    pub user_id: uuid::Uuid,
    pub email: String,
    pub hash: String,
}

impl User {
    // pub async fn find_by_id(id: uuid::Uuid, pool: &Pool<Postgres>) -> TResult<sqlx::Error> {
    //     let user = sqlx::query!(r#"SELECT * FROM play_user WHERE user_id=$1"#, id)
    //         .fetch_one(&*pool)
    //         .await?;

    //     Ok(user)
    // }

    pub async fn email_exist(email: String, pool: Pool<Postgres>) {
        let user = sqlx::query!(r#"SELECT email FROM play_user WHERE email = $1"#, email)
            .fetch_optional(&pool)
            .await.unwrap();
    }
}