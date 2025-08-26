/**
    The workers help in execting the tasks based on the
    given time at which the job should run.
*/
use std::time::Duration;
use serde::Deserialize;
use sqlx::PgPool;
use chrono::{DateTime, Utc};
use tokio::{sync::Semaphore, time};
use crate::models::Job;
use std::sync::Arc;

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

#[derive(Debug, Deserialize)]
struct ApiCallPayload {
    url: String,
    method: String,
    body: Option<serde_json::Value>
}

async fn perform_api_call(payload: &ApiCallPayload) -> Result<(), Box<dyn std::error::Error>> {
    print!("Handling API call for {}", payload.url);
    let client = reqwest::Client::new();
    let response = match payload.method.to_uppercase().as_str() {
        "POST" => {
            let mut request = client.post(&payload.url);
            if let Some(body) = &payload.body {
                request = request.json(body);
            }
            request.send().await?
        },
        "GET" => {
            client.get(&payload.url).send().await?
        },
        _ => {
            return Err("Unsupported HTTP method".into());
        }
    };

    println!("API call response status: {}", response.status());

    let response_text = response.text().await?;
    println!("Response Body: {}", response_text);

    Ok(())
}

async fn process_job(job: Job) {
    match job.job_type.as_str() {
        "api_call" => {
            match serde_json::from_value::<ApiCallPayload>(job.payload) {
                Ok(api_call_payload) => {
                    if let Err(e) = perform_api_call(&api_call_payload).await {
                        eprintln!("Error handling API call: {}", e);
                    }
                },
                Err(e) => {
                    eprintln!("Failed to deserialize API call payload: {}", e);
                }
            }
        },
        "function_call" => {
            println!("Function call job type is not yet implemented.");
        },
        _ => {
            eprintln!("Unknown job type: {}", job.job_type);
        }
    }

}

pub async fn worker(pool: PgPool) -> Result<(), sqlx::Error> {
    println!("⚙️ Worker started");
    const CONCURRENCY_LIMIT: usize = 100;
    
    let sleep_duration = Duration::from_secs(1);
    let semaphore = Arc::new(Semaphore::new(CONCURRENCY_LIMIT));


    loop {
        let current_time = Utc::now();
        let jobs = fetch_ready_jobs(&pool, current_time).await?;
    
        for job in jobs {
            println!("{:?}", job);
            let permit = Arc::clone(&semaphore).acquire_owned().await.unwrap();

            tokio::spawn(async move {
                let _permit = permit;
                process_job(job).await;
            });
        }

        time::sleep(sleep_duration).await;
    }
}