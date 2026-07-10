use serde::Serialize;
use tauri::State;

use crate::{
    domain::{commit::CommitGraphNode, Commit},
    infrastructure::{
        database::connection::get_connection,
        git::{CommitDiff, CommitInlineDiff, FileDiff},
    },
    services::commit_service,
    state::AppState,
};

#[derive(Debug, Serialize)]
pub struct CommitResponse {
    pub id: i64,
    pub hash: String,
    pub short_hash: String,
    pub author_name: String,
    pub author_email: String,
    pub committer_name: String,
    pub committer_email: String,
    pub subject: String,
    pub body: Option<String>,
    pub parent_count: u32,
    pub committed_at: String,
    pub is_significant: bool,
    pub is_merge_commit: bool,
    pub is_root_commit: bool,
}

#[derive(Debug, Serialize)]
pub struct CommitGraphNodeResponse {
    pub hash: String,
    pub branch_name: Option<String>,
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

impl From<Commit> for CommitResponse {
    fn from(c: Commit) -> Self {
        let merge = c.is_merge_commit();
        let root = c.is_root_commit();
        Self {
            short_hash: c.hash.short(),
            id: c.id.0,
            hash: c.hash.full().to_string(),
            author_name: c.author_name,
            author_email: c.author_email,
            committer_name: c.committer_name,
            committer_email: c.committer_email,
            subject: c.subject,
            body: c.body,
            parent_count: c.parent_count,
            committed_at: c.committed_at,
            is_significant: c.is_significant,
            is_merge_commit: merge,
            is_root_commit: root,
        }
    }
}

impl From<CommitGraphNode> for CommitGraphNodeResponse {
    fn from(node: CommitGraphNode) -> Self {
        Self {
            branch_name: node.branches.first().cloned(),
            branch_names: node.branches,
            hash: node.hash,
            author_name: node.author_name,
            author_email: node.author_email,
            subject: node.subject,
            committed_at: node.committed_at,
            additions: node.total_additions,
            deletions: node.total_deletions,
            total_changed_files_count: node.total_files_changed,
            parent_hashes: node.parents,
        }
    }
}

#[tauri::command]
pub fn get_commits(
    repo_id: i64,
    count: Option<usize>,
    offset: Option<usize>,
    state: State<'_, AppState>,
) -> Result<Vec<CommitResponse>, String> {
    let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    commit_service::get_commits(&conn, repo_id, count.unwrap_or(50), offset.unwrap_or(0))
        .map(|commits| commits.into_iter().map(CommitResponse::from).collect())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_commit_by_hash(
    repo_id: i64,
    hash: String,
    state: State<'_, AppState>,
) -> Result<CommitResponse, String> {
    let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    commit_service::get_commit_by_hash(&conn, repo_id, &hash)
        .map(CommitResponse::from)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_commit_graph(
    repo_id: i64,
    limit: Option<usize>,
    offset: Option<usize>,
    state: State<'_, AppState>,
) -> Result<Vec<CommitGraphNodeResponse>, String> {
    let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    commit_service::get_commit_graph(&conn, repo_id, limit.unwrap_or(50), offset.unwrap_or(0))
        .map(|commits| {
            commits
                .into_iter()
                .map(CommitGraphNodeResponse::from)
                .collect()
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_commit_diff(
    repo_id: i64,
    commit_hash: String,
    state: State<'_, AppState>,
) -> Result<CommitDiff, String> {
    let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    commit_service::get_commit_diff(&conn, repo_id, &commit_hash).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_commit_inline_diff(
    repo_id: i64,
    commit_hash: String,
    state: State<'_, AppState>,
) -> Result<CommitInlineDiff, String> {
    let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    commit_service::get_commit_inline_diff(&conn, repo_id, &commit_hash).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_file_diff(
    repo_id: i64,
    commit_hash: String,
    file_path: String,
    state: State<'_, AppState>,
) -> Result<FileDiff, String> {
    let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    commit_service::get_file_diff_by_commit_hash(&conn, repo_id, &commit_hash, &file_path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_file_diff_history(
    repo_id: i64,
    file_path: String,
    commit_count: Option<usize>,
    commit_offset: Option<usize>,
    state: State<'_, AppState>,
) -> Result<Vec<FileDiff>, String> {
    let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    commit_service::get_file_diff_history(
        &conn,
        repo_id,
        &file_path,
        commit_count.unwrap_or(10),
        commit_offset.unwrap_or(0),
    )
    .map_err(|e| e.to_string())
}
