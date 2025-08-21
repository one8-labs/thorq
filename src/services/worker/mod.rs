use tokio::time::{sleep, Duration};
use crate::models::Job;
use chrono::{Datelike, Timelike, TimeZone, Utc};
use sqlx::PgPool;

const JOB_WORKER_INTERVAL: u64 = 35; 
const MAX_RETRIES: u32 = 3; 

pub async fn job_worker(pool: &PgPool) {
    println!("🛠️ Job worker started. Checking for due jobs every {} seconds...", JOB_WORKER_INTERVAL);

    loop {
        let utc_time = Utc::now();
        let start_of_the_minute = Utc
        .with_ymd_and_hms(
            utc_time.year(),
            utc_time.month(),
            utc_time.day(),
            utc_time.hour(),
            utc_time.minute(),
            0,
        )
        .unwrap()
        .with_nanosecond(0).unwrap(); 

        let end_of_the_minute = start_of_the_minute + chrono::Duration::minutes(1) - chrono::Duration::nanoseconds(1);

        let due_jobs: Vec<Job> = match sqlx::query_as!(
            Job,
            r#"SELECT * FROM jobs 
               WHERE run_at >= $1 AND run_at <= $2
               ORDER BY run_at ASC"#,
            start_of_the_minute,
            end_of_the_minute
        )
        .fetch_all(pool)
        .await
        {
            Ok(jobs) => jobs,
            Err(e) => {
                eprintln!("❌ Error fetching due jobs: {}", e);
                sleep(Duration::from_secs(JOB_WORKER_INTERVAL)).await;
                continue;
            }
        };

        if !due_jobs.is_empty() {
            for job in due_jobs {
                println!("🔄 Processing job: {}", job.id);

                // TODO: add actual job processing here

                match sqlx::query!("DELETE FROM jobs WHERE id = $1", job.id)
                    .execute(pool)
                    .await
                {
                    Ok(_) => println!("✅ Job {} processed successfully", job.id),
                    Err(e) => eprintln!("❌ Error deleting job {}: {}", job.id, e),
                }
            }
        }

        sleep(Duration::from_secs(JOB_WORKER_INTERVAL)).await;
    }
}
