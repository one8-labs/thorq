// New arch for job workers

use std::time::Duration;
use sqlx::PgPool;
use chrono::{DateTime, Utc};
use tokio::time;
use crate::models::Job;

async fn fetch_ready_jobs(pool: &PgPool, current_time: DateTime<Utc>) -> Result<Vec<Job>, sqlx::Error> {

    let mut tx = pool.begin().await?;

    let jobs = sqlx::query_as!(Job, 
        r#"
            SELECT 
                id, queue_id, job_type, payload, priority, status, 
                run_at, inserted_at, updated_at, attempts, max_attempts, 
                last_error, timeout_sec, idempotency_key, tenant_id
            FROM jobs
            WHERE status = 'pending' AND run_at <= $1 
            ORDER BY priority DESC, run_at ASC
            FOR UPDATE SKIP LOCKED
        "#,
        current_time
    )
    .fetch_all(&mut *tx)
    .await?;

    let job_ids: Vec<i64> = jobs.iter().map(|job| job.id).collect();
    println!("{:?}", job_ids);

    if !job_ids.is_empty() {
        let updated_at = Utc::now();
        let status = "executing";

        let _ = sqlx::query!(
            r#"
                UPDATE jobs
                SET status = $1, updated_at = $2
                WHERE id = ANY($3)
            "#,
            status,
            updated_at,
            &job_ids
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    Ok(jobs)
}

pub async fn worker(pool: PgPool) -> Result<(), sqlx::Error> {
    println!("⚙️ Worker started");
    
    let sleep_duration = Duration::from_secs(1);

    loop {
        let current_time = Utc::now();
        let jobs = fetch_ready_jobs(&pool, current_time).await?;
    
        for job in jobs {
            println!("{:?}", job);
        }

        time::sleep(sleep_duration).await;
    }
}