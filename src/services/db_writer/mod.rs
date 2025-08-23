use chrono::{DateTime, Utc};
use sqlx::PgPool;
use serde_json::Value;

pub async fn db_writer(
    pool: &PgPool,
    queue_id: i64,
    job_type: String,
    payload: Value,
    run_at: DateTime<Utc>,
    priority: i32,
    max_attempts: i32,
    timeout_sec: i32,
    idempotency_key: Option<String>,
    tenant_id: Option<String>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO jobs (
            queue_id, type, payload, priority, status, run_at,
            inserted_at, updated_at, attempts, max_attempts,
            last_error, timeout_sec, idempotency_key, tenant_id
        )
        VALUES (
            $1, $2, $3, $4, 'pending', $5,
            now(), now(), 0, $6,
            NULL, $7, $8, $9
        )
        "#,
        queue_id,
        job_type,
        payload,
        priority,
        run_at,
        max_attempts,
        timeout_sec,
        idempotency_key,
        tenant_id,
    )
    .execute(pool)
    .await?;

    println!("✅ Job scheduled successfully");
    Ok(())
}
