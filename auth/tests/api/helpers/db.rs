use auth::{settings::database::DbSettings, errors::app::ApiError};
use sqlx::{PgConnection, Connection, Executor, PgPool, migrate::MigrateDatabase, Postgres, Pool};

pub struct TestDb;

impl TestDb {
    pub async fn create_db(config: &DbSettings) -> PgPool {
        println!("\n\n |||||||||||||THE DATABASE NAME {:#?} \n||||||||||||||||||||| \n\n ", &config.database_name);
        let mut connection = PgConnection::connect_with(&config.without_db())
            .await
            .expect("Failed to connect to postgres");


        connection.execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
            .await.expect("Failed to create database");
            
        let connection_pool = PgPool::connect_with(config.with_db())
            .await.expect("Failed to connect postgres");

        sqlx::migrate!("./migrations").run(&connection_pool)
            .await.expect("Failed to migrate the database");
    
        connection_pool
    }
    

    pub async fn drop_db(config: &DbSettings, db_pool: &Pool<Postgres>) -> Result<(), ApiError> {

        db_pool.close().await;
        let DbSettings { host, port, username, password, database_name, .. } = &config;
        println!("CLOSECLOSECLOSECLOSECLOSECLOSECLOSECLOSECLOSE {:#?}", &database_name);
        let url= format!("postgres://{}:{}@{}:{}/{}", username, password, host, port, database_name);

        sqlx::postgres::Postgres::drop_database(&url).await?;
        
        Ok(())
    }
}