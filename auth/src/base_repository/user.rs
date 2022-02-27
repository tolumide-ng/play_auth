use sqlx::{Pool, Postgres};
use anyhow::Result;


#[derive(Debug)]
pub struct User {
    pub user_id: uuid::Uuid,
    pub email: String,
    pub hash: String,
}

impl User {
    pub async fn find_by_id(id: u32, pool: &Pool<Postgres>) -> Result<User> {
        let user = sqlx::query_as!(User, r#"SELECT * FROM user WHERE user_id=$1"#)
            .fetch_one(&*pool)
            .await?;

        Ok(user)
    }
}