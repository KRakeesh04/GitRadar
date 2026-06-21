use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkingTreeStatus {
    pub id: i64,
    pub repo_id: i64,
    pub modified_count: i32,
    pub staged_count: i32,
    pub untracked_count: i32,
    pub deleted_count: i32,
    pub renamed_count: i32,
    pub captured_at: String,
}
