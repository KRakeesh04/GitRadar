use serde::Serialize;
use tauri::State;

use crate::{
    domain::Branch, infrastructure::database::connection::get_connection, services::branch_service,
    state::AppState,
};

#[derive(Debug, Serialize)]
pub struct BranchResponse {
    pub id: i64,
    pub repo_id: i64,
    pub name: String,
    pub branch_type: String,
    pub is_head: bool,
    pub is_default: bool,
    pub last_commit_hash: Option<String>,
    pub ahead_count_from_remote: u32,
    pub behind_count_from_remote: u32,
    pub ahead_count_from_default: u32,
    pub behind_count_from_default: u32,
    pub status: String,
    pub should_merge: bool,
    pub is_stale: bool,
    pub importance: String,
}

impl From<Branch> for BranchResponse {
    fn from(b: Branch) -> Self {
        let status = b.status().to_string();
        let should_merge = b.should_merge();
        let is_stale = b.is_stale();
        let importance = format!("{:?}", b.importance());
        let last_commit_hash = b.last_commit_hash.clone();
        Self {
            id: b.id.0,
            repo_id: b.repo_id,
            name: b.name.clone(),
            branch_type: format!("{:?}", b.branch_type),
            is_head: b.is_head,
            is_default: b.is_default,
            last_commit_hash,
            ahead_count_from_remote: b.ahead_count_from_remote,
            behind_count_from_remote: b.behind_count_from_remote,
            ahead_count_from_default: b.ahead_count_from_default,
            behind_count_from_default: b.behind_count_from_default,
            status,
            should_merge,
            is_stale,
            importance,
        }
    }
}

#[tauri::command]
pub fn get_repository_branches(
    repo_id: i64,
    state: State<'_, AppState>,
) -> Result<Vec<BranchResponse>, String> {
    let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    branch_service::get_repository_branches(&conn, repo_id)
        .map(|branches| branches.into_iter().map(BranchResponse::from).collect())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_branch_info(
    repo_id: i64,
    name: String,
    state: State<'_, AppState>,
) -> Result<BranchResponse, String> {
    let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    branch_service::get_branch_info_by_name(&conn, repo_id, &name)
        .map(BranchResponse::from)
        .map_err(|e| e.to_string())
}
