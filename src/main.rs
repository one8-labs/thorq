use std::path::Path;
use dotenv::from_path;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use thorq::services::database::init_db_pool;
use thorq::services::db_writer::db_writer;
use std::sync::Arc;
use axum::{
    extract::State, 
    Router,
    routing::post,
    http::StatusCode 
};


async fn write_to_db(
    State(pool): State<Arc<Pool<Postgres>>>
) -> Result<&'static str, StatusCode> {
    let now_utc: DateTime<Utc> = Utc::now();
    match db_writer(&pool, String::from("sample_job_function"), now_utc).await {
        Ok(_) => Ok("Job created successfully"),
        Err(e) => {
            eprint!("Database write failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env_path = Path::new(".env");
    from_path(env_path).expect("Failed to load .env file");

    let pool = init_db_pool().await?;
    let shared_pool = Arc::new(pool);

    let app = Router::new()
    .route("/create-job", post(write_to_db)).with_state(shared_pool);


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server is running on port 3000");
    axum::serve(listener, app).await?;

    Ok(())
}
