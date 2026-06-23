use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
    pub id: i64,
    pub repo_id: i64,
    pub name: String,
    pub is_head: bool,
    pub is_default: bool,
    pub last_commit_hash: Option<String>,
    pub last_commit_at: Option<String>,
    pub ahead_count_from_default: i32,
    pub behind_count_from_default: i32,
    pub ahead_count_from_remote: i32,
    pub behind_count_from_remote: i32,
    pub updated_at: String,
}
