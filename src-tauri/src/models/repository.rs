use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub id: i64,
    pub root_id: i64,
    pub name: String,
    pub path: String,
    pub git_dir_path: String,
    pub default_branch: Option<String>,
    pub head_branch: Option<String>,
    pub is_dirty: bool,
    pub last_commit_hash: Option<String>,
    pub last_commit_at: Option<String>,
    pub last_scanned_at: Option<String>,
    pub last_indexed_at: Option<String>,
    pub index_status: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}