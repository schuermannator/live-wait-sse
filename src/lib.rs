use serde::{Serialize, Deserialize};
use chrono::prelude::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
pub struct Student {
    pub name: String,
    pub comment: String,
    pub join_time: DateTime<Utc>,
}

