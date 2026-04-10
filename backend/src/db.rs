use sqlx::postgres::{PgPool, PgPoolOptions};
use dotenvy::dotenv;
use std::env;

pub async fn init_db() -> Result<PgPool, sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    let pool = PgPoolOptions::new()
        .max_connections(5) 
        .connect(&database_url)
        .await?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    println!("✅ Database Connected and Migrated!");
    Ok(pool)
}
