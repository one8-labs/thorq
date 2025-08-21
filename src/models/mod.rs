use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow)]
pub struct Job {
    pub id: i32,
    pub payload: String,
    pub run_at: DateTime<Utc>,
    pub created_at: Option<DateTime<Utc>>,
}

impl Job {
    pub fn new(payload: String, run_at: DateTime<Utc>) -> Self {
        Self {
            id: 0,
            payload,
            run_at,
            created_at: Some(Utc::now()),
        }
    }
}