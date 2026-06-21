use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitParent {
    pub id: i64,
    pub repo_id: i64,
    pub commit_hash: String,
    pub parent_hash: String,
    pub parent_index: i32,
}
