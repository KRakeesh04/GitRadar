use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Repository {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub git_dir_path: String,
    pub repo_type: String,
    pub is_enabled: bool,
    pub remote_url: Option<String>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepositoryRoot {
    pub id: i64,
    pub root_id: i64,
    pub repo_id: i64,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepositorySummary {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub git_dir_path: String,
    pub repo_type: String,
    pub is_enabled: bool,
    pub is_starred: bool,
    pub starred_at: Option<String>,
    pub remote_url: Option<String>,
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
    pub health_score: Option<f32>,
    pub total_commits: Option<i64>,
    pub weekly_commits: Option<i64>,
    pub unique_contributors: Option<i64>,
    #[serde(default)]
    pub root_ids: Vec<i64>,
}

