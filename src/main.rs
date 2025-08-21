use std::env;
use std::path::Path;
use dotenv::from_path;

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

    let pool = init_db_pool().await?;

    Ok(())
}
