use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexingJob {
    pub id: i64,
    pub repo_id: i64,
    pub job_type: String,
    pub status: String,
    pub progress: i32,
    pub total_items: Option<i32>,
    pub processed_items: i32,
    pub error_message: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
