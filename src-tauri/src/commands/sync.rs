use tauri::{AppHandle, Emitter, State};

use crate::{
    infrastructure::database::{
        connection::get_connection, models::IndexingJob, repositories::indexing_jobs,
    },
    services::sync_service,
    state::AppState,
};

#[tauri::command]
pub fn sync_repository(
    app: AppHandle,
    repo_id: i64,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    sync_service::sync_repository(&mut conn, repo_id).map_err(|e| e.to_string())?;
    app.emit("repository-sync-finished", repo_id)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn sync_branches(
    app: AppHandle,
    repo_id: i64,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    sync_service::sync_branches(&mut conn, repo_id).map_err(|e| e.to_string())?;
    app.emit("branches-sync-finished", repo_id)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn sync_commits(
    app: AppHandle,
    repo_id: i64,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    sync_service::sync_commits(&mut conn, repo_id).map_err(|e| e.to_string())?;
    app.emit("commits-sync-finished", repo_id)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn sync_contributors(
    app: AppHandle,
    repo_id: i64,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    sync_service::sync_contributors(&mut conn, repo_id).map_err(|e| e.to_string())?;
    app.emit("contributors-sync-finished", repo_id)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn sync_repository_files(
    app: AppHandle,
    repo_id: i64,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    sync_service::sync_repository_files(&mut conn, repo_id).map_err(|e| e.to_string())?;
    app.emit("repository-files-sync-finished", repo_id)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn sync_commit_file_stats(
    app: AppHandle,
    repo_id: i64,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    sync_service::sync_commit_file_stats(&mut conn, repo_id).map_err(|e| e.to_string())?;
    app.emit("commit-file-stats-sync-finished", repo_id)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn sync_repo_activity(
    app: AppHandle,
    repo_id: i64,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    sync_service::sync_repo_activity(&mut conn, repo_id).map_err(|e| e.to_string())?;
    app.emit("repo-activity-sync-finished", repo_id)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn sync_working_tree_status(
    app: AppHandle,
    repo_id: i64,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    sync_service::sync_working_tree_status(&mut conn, repo_id).map_err(|e| e.to_string())?;
    app.emit("working-tree-status-sync-finished", repo_id)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn sync_repository_health(
    app: AppHandle,
    repo_id: i64,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    sync_service::sync_repository_health(&mut conn, repo_id).map_err(|e| e.to_string())?;
    app.emit("repository-health-sync-finished", repo_id)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn sync_file_hotspots(
    app: AppHandle,
    repo_id: i64,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    sync_service::sync_file_hotspots(&mut conn, repo_id).map_err(|e| e.to_string())?;
    app.emit("file-hotspots-sync-finished", repo_id)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_indexing_jobs_by_repo(
    repo_id: i64,
    limit: Option<i32>,
    state: State<'_, AppState>,
) -> Result<Vec<IndexingJob>, String> {
    let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    indexing_jobs::get_indexing_jobs_by_repo(&conn, repo_id, limit).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_pending_indexing_jobs(state: State<'_, AppState>) -> Result<Vec<IndexingJob>, String> {
    let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    indexing_jobs::get_pending_indexing_jobs(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn cleanup_completed_indexing_jobs(
    days_old: i32,
    state: State<'_, AppState>,
) -> Result<i64, String> {
    let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    indexing_jobs::cleanup_completed_indexing_jobs(&conn, days_old).map_err(|e| e.to_string())
}
