use sqlx::PgPool;

pub async fn create_queue(
    pool: &PgPool,
    name: String,
    concurrency: Option<i32>,
    rate_per_sec: Option<f64>,
) -> Result<i64, sqlx::Error> {
    let rec = sqlx::query!(
        r#"
        INSERT INTO queues (name, concurrency, rate_per_sec, created_at)
        VALUES ($1, $2, $3, now())
        RETURNING id
        "#,
        name,
        concurrency.unwrap_or(10),
        rate_per_sec.unwrap_or(100.0)
    )
    .fetch_one(pool)
    .await?;

    println!("✅ Queue '{}' created with id {}", name, rec.id);

    Ok(rec.id)
}