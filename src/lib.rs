use serde::{Serialize, Deserialize};
use chrono::prelude::{DateTime, Utc};

pub mod sse;

#[derive(Clone, Serialize, Deserialize)]
pub struct Student {
    pub name: String,
    pub comment: String,
    pub join_time: DateTime<Utc>,
}

