use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod sse;

#[derive(Clone, Serialize, Deserialize)]
pub struct Student {
    pub name: String,
    pub comment: String,
    pub join_time: DateTime<Utc>,
}
