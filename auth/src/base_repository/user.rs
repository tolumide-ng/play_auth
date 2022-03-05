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
        // handle db connectioin errors with an interceptor
        // let user = sqlx::query!(r#"SELECT * FROM play_user WHERE (email = $1)"#, email)
        //     .fetch_optional(pool)
        //     .await.unwrap();
        let user = sqlx::query!(r#"SELECT email FROM play_user WHERE email = $1"#, email)
            .fetch_optional(pool)
            .await.unwrap();

        // let user = sqlx::query!(r#"SELECT email FROM play_user WHERE username = $1"#, username)
        //     .fetch_optional(pool)
        //     .await.unwrap();

        // println!("the users>>>>> {:#?}", user);

        // if user.len() > 0 {
        //     return true;
        // }

        return true;
    }

    pub async fn create_user(pool: &Pool<Postgres>, email: String, username: String, hash: String) {
        println!("::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
        let user = sqlx::query!(r#"INSERT INTO play_user (email, username, hash) VALUES ($1, $2, $3)"#, email, username, hash)
            .execute(pool).await.unwrap();

        println!("THE OBTAINED USER------------->>>>>>>>>>> {:#?}", user);
    }
}