use chrono::{DateTime, Utc};

pub async fn db_writer(pool: &sqlx::PgPool, payload: String, run_at: DateTime<Utc>) -> Result<(), sqlx::Error> {
    sqlx::query!("INSERT INTO Jobs (payload, run_at) VALUES ($1, $2)", payload, run_at).execute(pool).await?;
    println!("✅ Job scheduled successfully");
    Ok(())
}