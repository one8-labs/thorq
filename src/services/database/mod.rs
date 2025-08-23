use std::env;

pub async fn init_db_pool() -> Result<sqlx::PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
    .map_err(|e| sqlx::Error::Configuration(format!("DATABASE_URL is not set: {}", e).into()))?;

    let pool = sqlx::PgPool::connect(&database_url).await?;
    println!("Connected to the database");
    Ok(pool)
}