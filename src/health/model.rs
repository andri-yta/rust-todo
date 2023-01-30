use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct HealthStatus {
    pub status: String,
    pub date: DateTime<Utc>,
}