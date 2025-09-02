/**
    The workers help in execting the tasks based on the
    given time at which the job should run.
*/
use std::{process::Stdio, time::Duration};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize};
use sqlx::PgPool;
use chrono::{DateTime, Utc};
use tokio::{sync::Semaphore, time};
use crate::models::Job;
use std::sync::Arc;
use tokio::process::Command;

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

async fn perform_api_call(payload: &ApiCallPayload) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
            return Err(Box::<dyn std::error::Error + Send + Sync>::from("Unsupported HTTP method"));
        }
    };

    println!("API call response status: {}", response.status());

    let response_text = response.text().await?;
    println!("Response Body: {}", response_text);

    Ok(())
}

async fn update_job_status(pool: &PgPool, job_id: i64, status: &str) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
            UPDATE jobs
            SET status = $1, updated_at = NOW()
            WHERE id = $2
        "#,
        status,
        job_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

async fn perform_fn_call(interpreter: &str, code: &str, file_name: &str) -> Result<String, String>{
    let tmp_file = format!("/tmp/{}", file_name);
    
    if let Err(e) = tokio::fs::write(&tmp_file, code).await {
        return Err(format!("Failed to write code to file: {}", e));
    }

    let output = Command::new(interpreter)
        .arg(&tmp_file)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("Failed to run {}: {}",interpreter, e))?;

    if let Err(e) = tokio::fs::remove_file(&tmp_file).await {
        eprintln!("Warning: failed to remove temp file {}: {}", tmp_file, e);
    }

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

fn unique_filename(extension: &str) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("job_{}.{}", nanos, extension)
}


async fn process_job(pool: Arc<PgPool>, job: Job) {
    match job.job_type.as_str() {
        "api_call" => {
            match serde_json::from_value::<ApiCallPayload>(job.payload) {
                Ok(api_call_payload) => {
                    match perform_api_call(&api_call_payload).await {
                        Ok(_) => {
                            if let Err(e) = update_job_status(&pool, job.id, "executed").await {
                                eprintln!("Failed to update job status: {}", e);
                            }
                        }
                        Err(e) => {
                            eprintln!("Error handling API call: {}", e);
                            if let Err(e) = update_job_status(&pool, job.id, "failed").await {
                                eprintln!("Failed to mark job as failed: {}", e);

                            }
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Failed to deserialize API call payload: {}", e);
                    if let Err(e) = update_job_status(&pool, job.id, "failed").await {
                        eprintln!("Failed to mark job as failed: {}", e);
                    }
                }
            }
        },
        "py_fn_call" => {
            if let Some(code) = job.payload.as_str() {
                let file_name = unique_filename("py");
                
                match perform_fn_call("python3", code, &file_name).await {
                    Ok(result) => {
                        println!("Python execution result: {}", result);
                        let _ = update_job_status(&pool, job.id, "executed").await;
                    }
                    Err(e) => {
                        eprintln!("Python execution failed: {}", e);
                        let _ = update_job_status(&pool, job.id, "failed").await;
                    }
                }
            } else {
                eprintln!("Payload was not a string for job {}", job.id);
            };
        },
        "js_fn_call" => {
            print!("Function call for nodejs ")
        }
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
    let pool = Arc::new(pool);


    loop {
        let current_time = Utc::now();
        let jobs = fetch_ready_jobs(&pool, current_time).await?;
    
        for job in jobs {
            println!("{:?}", job);
            let permit = Arc::clone(&semaphore).acquire_owned().await.unwrap();
            let pool = Arc::clone(&pool);

            tokio::spawn(async move {
                let _permit = permit;
                process_job(pool, job).await;
            });
        }

        time::sleep(sleep_duration).await;
    }
}