use std::env;
use std::path::Path;
use dotenv::from_path;
use thorq::services::db_writer::db_writer;
use chrono::{DateTime, Utc};

async fn init_db_pool() -> Result<sqlx::PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
    .map_err(|e| sqlx::Error::Configuration(format!("DATABASE_URL is not set: {}", e).into()))?;

    let pool = sqlx::PgPool::connect(&database_url).await?;
    println!("Connected to the database");
    Ok(pool)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env_path = Path::new(".env");
    from_path(env_path).expect("Failed to load .env file");

    let pool: sqlx::Pool<sqlx::Postgres> = init_db_pool().await?;
    let now_utc: DateTime<Utc> = Utc::now();

    // push a sample job to the postgres
    db_writer(&pool, String::from("sample_job_function"), now_utc).await?;

    Ok(())
}
