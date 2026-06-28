use serde::Serialize;
use tauri::State;

use crate::{
    domain::RepositoryActivityDaily, infrastructure::database::connection::get_connection,
    services::analytics_service, state::AppState,
};

#[derive(Debug, Serialize)]
pub struct RepositoryActivityDailyResponse {
    pub id: i64,
    pub repo_id: i64,
    pub activity_date: String,
    pub commit_count: i32,
    pub additions: i32,
    pub deletions: i32,
    pub files_changed: i32,
}

impl From<RepositoryActivityDaily> for RepositoryActivityDailyResponse {
    fn from(a: RepositoryActivityDaily) -> Self {
        Self {
            id: a.id,
            repo_id: a.repo_id,
            activity_date: a.activity_date,
            commit_count: a.commit_count,
            additions: a.additions,
            deletions: a.deletions,
            files_changed: a.files_changed,
        }
    }
}

#[tauri::command]
pub fn get_repository_activity(
    repo_id: i64,
    start_date: Option<String>,
    end_date: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<RepositoryActivityDailyResponse>, String> {
    let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    analytics_service::get_repository_activity(
        &conn,
        repo_id,
        start_date.as_deref(),
        end_date.as_deref(),
    )
    .map(|activity| {
        activity
            .into_iter()
            .map(RepositoryActivityDailyResponse::from)
            .collect()
    })
    .map_err(|e| e.to_string())
}
