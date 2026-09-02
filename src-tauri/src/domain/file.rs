use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct RepositoryFile {
    pub id: i64,
    pub repo_id: i64,
    pub path: String,
    pub name: String,
    pub extension: Option<String>,
    pub size_bytes: Option<i64>,
    pub is_binary: bool,
    pub last_modified_at: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
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

#[derive(Debug, Clone, Serialize)]
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

#[derive(Debug, Clone, Serialize)]
pub struct RepositoryActivityDaily {
    pub id: i64,
    pub repo_id: i64,
    pub activity_date: String,
    pub commit_count: i32,
    pub additions: i32,
    pub deletions: i32,
    pub files_changed: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct LanguageStats {
    pub total_bytes: u64,
    pub languages: Vec<LanguageStat>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LanguageStat {
    pub language: String,
    pub bytes: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileTreeNode {
    pub name: String,
    pub path: String,
    pub is_directory: bool,
    pub size_or_file_count: u64,
    pub children: Vec<FileTreeNode>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileData {
    pub mime_type: String,
    pub data: Vec<u8>,
}
