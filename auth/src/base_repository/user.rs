use sqlx::{Pool, Postgres};


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
    pub async fn email_exist(pool: &Pool<Postgres>, email: String, username: String) -> bool {
        let user = sqlx::query!(r#"SELECT email FROM play_user WHERE (email = $1) OR (username = $2)"#, email, username)
            .fetch_optional(pool)
            .await.unwrap();

        if user.is_some() {
            return false;
        }

        return false;
    }

    pub async fn create_user(pool: &Pool<Postgres>, email: String, username: String, hash: String) -> Option<bool> {
        let user = sqlx::query!(r#"INSERT INTO play_user (email, username, hash) VALUES ($1, $2, $3)"#, email, username, hash)
            .execute(pool).await;

        if let Err(e) = user {
            // todo!() - this here can be handled with tracing! so we can save it on our log
            println!("THE ERROR THAT OCCURED >>> {:#?}", e);
            return None
        }

        return Some(true);
    }
}