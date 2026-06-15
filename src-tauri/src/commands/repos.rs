use crate::db::connection::get_connection;
use crate::db::repositories::get_all_repositories as get_repos_from_db;
use crate::models::Repository;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub fn get_all_repositories(state: State<'_, AppState>) -> Result<Vec<Repository>, String> {
    let conn = get_connection(&state.db_path).map_err(|e: rusqlite::Error| e.to_string())?;
    let repositories = get_repos_from_db(&conn).map_err(|e: rusqlite::Error| e.to_string())?;
    Ok(repositories)
}
