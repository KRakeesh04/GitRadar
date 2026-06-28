use serde::Serialize;
use tauri::State;

use crate::{
    domain::Contributor, infrastructure::database::connection::get_connection,
    services::contributor_service, state::AppState,
};

#[derive(Debug, Serialize)]
pub struct ContributorResponse {
    pub id: i64,
    pub repo_id: i64,
    pub name: String,
    pub email: String,
    pub commit_count: u32,
    pub additions: u32,
    pub deletions: u32,
    pub active_days: u32,
    pub last_commit_at: Option<String>,
    pub impact_score: f32,
    pub contributor_level: String,
    pub is_active: bool,
}

impl From<Contributor> for ContributorResponse {
    fn from(c: Contributor) -> Self {
        let impact_score = c.impact_score();
        let level = c.contributor_level().to_string();
        let is_active = c.is_active();
        let last_commit_at = c.last_commit_at;
        Self {
            id: c.id.0,
            repo_id: c.repo_id,
            name: c.name,
            email: c.email,
            commit_count: c.commit_count,
            additions: c.additions,
            deletions: c.deletions,
            active_days: c.active_days,
            last_commit_at,
            impact_score,
            contributor_level: level,
            is_active,
        }
    }
}

#[tauri::command]
pub fn get_contributors(
    repo_id: i64,
    state: State<'_, AppState>,
) -> Result<Vec<ContributorResponse>, String> {
    let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    contributor_service::get_contributors(&conn, repo_id)
        .map(|contributors| {
            contributors
                .into_iter()
                .map(ContributorResponse::from)
                .collect()
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_top_contributors(
    repo_id: i64,
    limit: Option<i32>,
    state: State<'_, AppState>,
) -> Result<Vec<ContributorResponse>, String> {
    let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    contributor_service::get_top_contributors(&conn, repo_id, limit)
        .map(|contributors| {
            contributors
                .into_iter()
                .map(ContributorResponse::from)
                .collect()
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_contributor_by_email(
    repo_id: i64,
    email: String,
    state: State<'_, AppState>,
) -> Result<ContributorResponse, String> {
    let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    contributor_service::get_contributor_by_email(&conn, repo_id, &email)
        .map(ContributorResponse::from)
        .map_err(|e| e.to_string())
}
