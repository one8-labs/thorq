use std::path::Path;
use std::sync::Arc;
use dotenv::from_path;
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use chrono::{DateTime, Utc};
use thorq::services::database::init_db_pool;
use thorq::services::db_writer::db_writer;
use thorq::controllers::queue_controller::create_queue;
use thorq::services::worker::worker;
use axum::{
    Json,
    extract::State, 
    Router,
    routing::post,
    http::StatusCode 
};


#[derive(Debug, Deserialize)]
pub struct CreateJobRequest {
    pub job_type: String,
    pub payload: serde_json::Value,
    pub run_at: Option<DateTime<Utc>>,
    pub queue_id: i64,
    pub priority: Option<i32>,
    pub max_attempts: Option<i32>,
    pub timeout_sec: Option<i32>,
    pub idempotency_key: Option<String>,
    pub tenant_id: Option<String>,
}


async fn write_to_db(
    State(pool): State<Arc<Pool<Postgres>>>,
    Json(req): Json<CreateJobRequest>
) -> Result<&'static str, StatusCode> {
    let run_at = req.run_at.unwrap_or_else(Utc::now);

    match db_writer(
        &pool,
        req.queue_id,
        req.job_type,
        req.payload,
        run_at,
        req.priority.unwrap_or(0),
        req.max_attempts.unwrap_or(25),
        req.timeout_sec.unwrap_or(300),
        req.idempotency_key,
        req.tenant_id,
    ).await {
        Ok(_) => Ok("Job created successfully"),
        Err(e) => {
            eprint!("Database write failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateQueueRequest {
    pub id: i64,                        
    pub name: String,                   
    pub concurrency: Option<i32>,               
    pub rate_per_sec: Option<i64>,      
    pub created_at: DateTime<Utc>, 
}

async fn add_queue_to_db(
    State(pool): State<Arc<Pool<Postgres>>>,
    Json(req): Json<CreateQueueRequest>
) -> Result<&'static str, StatusCode> {
    match create_queue(&pool, req.name, None, None).await {
        Ok(_) => Ok("Queue created successfully"),
        Err(e) => {
            eprint!("Error while creating queue: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env_path = Path::new(".env");
    from_path(env_path).expect("Failed to load .env file");

   let worker_pool = init_db_pool().await?;

    tokio::spawn(async move {
        let _ = worker(worker_pool).await;
    });

    let pool = init_db_pool().await?;
    let shared_pool = Arc::new(pool);

    let app = Router::new()
    .route("/create-queue", post(add_queue_to_db)).with_state(shared_pool.clone())
    .route("/create-job", post(write_to_db)).with_state(shared_pool.clone());


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server is running on port 3000");
    axum::serve(listener, app).await?;

    Ok(())
}
