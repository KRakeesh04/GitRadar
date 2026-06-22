use serde::Serialize;
use tauri::State;

use crate::{
    domain::Repository, infrastructure::database::connection::get_connection, services::repocitory,
    state::AppState,
};

#[derive(Debug, Serialize)]
pub struct RepositoryResponse {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub git_dir: String,
    pub health_score: f32,
    pub activity_level: String,
    pub default_branch: Option<String>,
    pub head_branch: Option<String>,
    pub remote_url: Option<String>,
    pub is_dirty: bool,
    pub total_commits: u32,
    pub unique_contributors: u32,
}

impl From<Repository> for RepositoryResponse {
    fn from(repository: Repository) -> Self {
        Self {
            id: repository.id.value(),
            name: repository.name,
            path: repository.path.to_string_lossy().into_owned(),
            git_dir: repository.git_dir.to_string_lossy().into_owned(),
            health_score: repository.health_score.value(),
            activity_level: format!("{:?}", repository.activity_level),
            default_branch: repository.default_branch,
            head_branch: repository.head_branch,
            remote_url: repository.remote_url,
            is_dirty: repository.is_dirty,
            total_commits: repository.total_commits.value(),
            unique_contributors: repository.unique_contributors,
        }
    }
}

#[tauri::command]
pub fn get_repository_info(
    repo_id: i64,
    state: State<'_, AppState>,
) -> Result<RepositoryResponse, String> {
    let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    repocitory::get_repository_info_by_id(&conn, repo_id)
        .map(RepositoryResponse::from)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_all_repositories(state: State<'_, AppState>) -> Result<Vec<RepositoryResponse>, String> {
    let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    repocitory::get_all_repositories(&conn)
        .map(|repositories| {
            repositories
                .into_iter()
                .map(RepositoryResponse::from)
                .collect()
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn discover_repositories(state: State<'_, AppState>) -> Result<(), String> {
    let mut conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    repocitory::discover_repositories(&mut conn)
}

#[tauri::command]
pub fn add_tracked_root_path(path: String, state: State<'_, AppState>) -> Result<i64, String> {
    let mut conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    repocitory::add_tracked_root_path(&mut conn, &path).map_err(|e| e.to_string())
}
