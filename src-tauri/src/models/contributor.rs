use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Contributor {
    pub id: i64,
    pub repo_id: i64,
    pub author_name: String,
    pub author_email: Option<String>,
    pub commit_count: i32,
    pub additions: i32,
    pub deletions: i32,
    pub active_days: i32,
    pub last_commit_at: Option<String>,
    pub updated_at: String,
}
