use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitFileStat {
    pub id: i64,
    pub repo_id: i64,
    pub commit_hash: String,
    pub file_path: String,
    pub change_type: String,
    pub additions: i32,
    pub deletions: i32,
    pub total_changes: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileHotspot {
    pub id: i64,
    pub repo_id: i64,
    pub file_path: String,
    pub touch_count: i32,
    pub churn_score: f64,
    pub hotspot_score: f64,
    pub last_touched_at: Option<String>,
    pub updated_at: String,
}
