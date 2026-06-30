pub mod branch;
pub mod commit;
pub mod file;
pub mod graph;
pub mod repo;
pub mod status;

use std::path::PathBuf;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DiscoveredRepository {
    pub name: String,
    pub path: PathBuf,
    pub git_dir: PathBuf,
    pub repo_type: RepositoryType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RepositoryInfo {
    pub remote_url: Option<String>,
    pub default_branch: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NewBranch {
    pub name: String,
    pub is_head: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LastCommit {
    pub hash: String,
    pub committed_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct WorkingTreeStatus {
    pub added: Vec<String>,
    pub modified: Vec<String>,
    pub deleted: Vec<String>,
    pub renamed: Vec<(String, String)>, // (old_path, new_path)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GraphNode {
    pub hash: String,
    pub message: String,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub committed_at: i64,
    pub parent_hashes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FileHotspotPerCommit {
    pub file_path: String,
    pub addions: i32,
    pub deletions: i32,
    pub change_type: ChangeType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum ChangeType {
    Added,
    Modified,
    Deleted,
    Renamed,
    Copied,
}

impl ChangeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ChangeType::Added => "Added",
            ChangeType::Modified => "Modified",
            ChangeType::Deleted => "Deleted",
            ChangeType::Renamed => "Renamed",
            ChangeType::Copied => "Copied",
        }
    }
}

pub fn change_type(status: git2::Delta) -> ChangeType {
    match status {
        git2::Delta::Added => ChangeType::Added,
        git2::Delta::Deleted => ChangeType::Deleted,
        git2::Delta::Renamed => ChangeType::Renamed,
        git2::Delta::Copied => ChangeType::Copied,
        git2::Delta::Modified | git2::Delta::Typechange => ChangeType::Modified,
        _ => ChangeType::Modified,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommitDiff {
    pub commit_hash: String,
    pub files: Vec<FileDiff>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FileDiff {
    pub old_path: Option<String>,
    pub new_path: String,
    pub change_type: ChangeType,
    pub additions: i32,
    pub deletions: i32,
    pub hunks: Vec<DiffHunk>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DiffHunk {
    pub old_start: i32,
    pub old_lines: i32,
    pub new_start: i32,
    pub new_lines: i32,
    pub lines: Vec<DiffLine>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DiffLine {
    pub line_type: DiffLineType,
    pub old_line_number: Option<u32>,
    pub new_line_number: Option<u32>,
    pub content: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum DiffLineType {
    Context,
    Added,
    Removed,
}
