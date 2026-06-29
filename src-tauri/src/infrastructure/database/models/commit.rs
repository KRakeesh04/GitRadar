use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub id: i64,
    pub repo_id: i64,
    pub hash: String,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub committer_name: Option<String>,
    pub committer_email: Option<String>,
    pub subject: String,
    pub body: Option<String>,
    pub parent_count: i32,
    pub committed_at: String,
    pub inserted_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitGraphNode {
    pub hash: String,
    pub branch_names: Vec<String>,
    pub author_name: String,
    pub author_email: String,
    pub subject: String,
    pub committed_at: String,
    pub additions: i32,
    pub deletions: i32,
    pub total_changed_files_count: i32,
    pub parent_hashes: Vec<String>,
}
