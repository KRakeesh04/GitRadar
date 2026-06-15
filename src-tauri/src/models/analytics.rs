use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoActivityDaily {
    pub id: i64,
    pub repo_id: i64,
    pub activity_date: String,
    pub commit_count: i32,
    pub additions: i32,
    pub deletions: i32,
    pub files_changed: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoSummary {
    pub repo_id: i64,
    pub total_commits: i32,
    pub total_contributors: i32,
    pub hotspot_file_count: i32,
    pub health_score: f64,
}
