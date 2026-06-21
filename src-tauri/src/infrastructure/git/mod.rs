pub mod branch;
pub mod commit;
pub mod file;
pub mod graph;
pub mod repo;
pub mod status;

use std::path::PathBuf;

pub struct DiscoveredRepository {
    pub name: String,
    pub path: PathBuf,
    pub git_dir: PathBuf,
    pub repo_type: RepositoryType,
}

pub struct RepositoryInfo {
    pub remote_url: Option<String>,
    pub default_branch: Option<String>,
}

pub enum RepositoryType {
    Standard,
    Submodule,
    Worktree,
}

impl RepositoryType {
    pub fn as_str(&self) -> &'static str {
        match self {
            RepositoryType::Standard => "Standard",
            RepositoryType::Submodule => "Submodule",
            RepositoryType::Worktree => "Worktree",
        }
    }
}

pub struct NewCommit {
    pub hash: String,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub committer_name: Option<String>,
    pub committer_email: Option<String>,
    pub subject: String,
    pub body: Option<String>,
    pub parent_count: i32,
    pub committed_at: String,
}

pub struct NewBranch {
    pub name: String,
    pub is_head: bool,
}

pub struct LastCommit {
    pub hash: String,
    pub committed_at: String,
}

pub struct WorkingTreeStatus {
    pub added: Vec<String>,
    pub modified: Vec<String>,
    pub deleted: Vec<String>,
    pub renamed: Vec<(String, String)>, // (old_path, new_path)
}

pub struct GraphNode {
    pub hash: String,
    pub message: String,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub committed_at: i64,
    pub parent_hashes: Vec<String>,
}

pub struct FileHotspotPerCommit {
    pub file_path: String,
    pub addions: i32,
    pub deletions: i32,
    pub change_type: ChangeType,
}

#[derive(Debug, Clone)]
pub enum ChangeType {
    Added,
    Modified,
    Deleted,
    Renamed,
}

impl ChangeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ChangeType::Added => "Added",
            ChangeType::Modified => "Modified",
            ChangeType::Deleted => "Deleted",
            ChangeType::Renamed => "Renamed",
        }
    }
}

#[derive(Debug, Clone)]
pub struct FilePatch {
    pub file_path: String,
    pub old_path: Option<String>,
    pub change_type: ChangeType,
    pub patch: String,
}
