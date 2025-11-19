use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Job {
    pub id: i64,                        // BIGSERIAL -> i64
    pub queue_id: i64,                  // BIGINT
    pub job_type: String,               // TEXT (renamed from 'type' since type is reserved)
    pub payload: serde_json::Value,     // JSONB -> serde_json::Value
    pub priority: i32,                  // INT
    pub status: String,                 // TEXT
    pub run_at: DateTime<Utc>,          // TIMESTAMPTZ
    pub inserted_at: DateTime<Utc>,     // TIMESTAMPTZ
    pub updated_at: DateTime<Utc>,      // TIMESTAMPTZ
    pub attempts: i32,                  // INT
    pub max_attempts: i32,              // INT
    pub last_error: Option<String>,     // nullable TEXT
    pub timeout_sec: i32,               // INT
    pub idempotency_key: Option<String>,// nullable TEXT
    pub tenant_id: Option<String>,      // nullable TEXT
}

impl Job {
    pub fn new(
        queue_id: i64,
        job_type: String,
        payload: serde_json::Value,
        run_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id: 0,
            queue_id,
            job_type,
            payload,
            priority: 0,
            status: "pending".to_string(),
            run_at,
            inserted_at: Utc::now(),
            updated_at: Utc::now(),
            attempts: 0,
            max_attempts: 25,
            last_error: None,
            timeout_sec: 300,
            idempotency_key: None,
            tenant_id: None,
        }
    }
}
