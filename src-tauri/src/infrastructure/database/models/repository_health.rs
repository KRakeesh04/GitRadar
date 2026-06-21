use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryHealth {
    pub repo_id: i64,
    pub health_score: f64,
    pub issues_count: i32,
    pub warnings_count: i32,
    pub check_status: String,
    pub last_check_at: String,
}
