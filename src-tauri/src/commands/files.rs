use serde::Serialize;
use tauri::State;

use crate::{
    domain::{CommitFileStat, FileHotspot, LanguageStat, LanguageStats, RepositoryFile},
    infrastructure::database::connection::get_connection,
    services::file_service,
    state::AppState,
};

#[derive(Debug, Serialize)]
pub struct RepositoryFileResponse {
    pub id: i64,
    pub repo_id: i64,
    pub path: String,
    pub name: String,
    pub extension: Option<String>,
    pub size_bytes: Option<i64>,
    pub is_binary: bool,
    pub last_modified_at: Option<String>,
}

impl From<RepositoryFile> for RepositoryFileResponse {
    fn from(f: RepositoryFile) -> Self {
        Self {
            id: f.id,
            repo_id: f.repo_id,
            path: f.path,
            name: f.name,
            extension: f.extension,
            size_bytes: f.size_bytes,
            is_binary: f.is_binary,
            last_modified_at: f.last_modified_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CommitFileStatResponse {
    pub id: i64,
    pub repo_id: i64,
    pub commit_hash: String,
    pub file_path: String,
    pub change_type: String,
    pub additions: i32,
    pub deletions: i32,
    pub total_changes: i32,
}

impl From<CommitFileStat> for CommitFileStatResponse {
    fn from(s: CommitFileStat) -> Self {
        Self {
            id: s.id,
            repo_id: s.repo_id,
            commit_hash: s.commit_hash,
            file_path: s.file_path,
            change_type: s.change_type,
            additions: s.additions,
            deletions: s.deletions,
            total_changes: s.total_changes,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FileHotspotResponse {
    pub id: i64,
    pub repo_id: i64,
    pub file_path: String,
    pub touch_count: i32,
    pub churn_score: f64,
    pub hotspot_score: f64,
    pub last_touched_at: Option<String>,
    pub updated_at: String,
}

impl From<FileHotspot> for FileHotspotResponse {
    fn from(h: FileHotspot) -> Self {
        Self {
            id: h.id,
            repo_id: h.repo_id,
            file_path: h.file_path,
            touch_count: h.touch_count,
            churn_score: h.churn_score,
            hotspot_score: h.hotspot_score,
            last_touched_at: h.last_touched_at,
            updated_at: h.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct LanguageStatResponse {
    pub language: String,
    pub bytes: u64,
}

impl From<LanguageStat> for LanguageStatResponse {
    fn from(l: LanguageStat) -> Self {
        Self {
            language: l.language,
            bytes: l.bytes,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct LanguageStatsResponse {
    pub total_bytes: u64,
    pub languages: Vec<LanguageStatResponse>,
}

impl From<LanguageStats> for LanguageStatsResponse {
    fn from(s: LanguageStats) -> Self {
        Self {
            total_bytes: s.total_bytes,
            languages: s.languages.into_iter().map(Into::into).collect(),
        }
    }
}

#[tauri::command]
pub fn get_repository_files(
    repo_id: i64,
    state: State<'_, AppState>,
) -> Result<Vec<RepositoryFileResponse>, String> {
    let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    file_service::get_repository_files(&conn, repo_id)
        .map(|files| {
            files
                .into_iter()
                .map(RepositoryFileResponse::from)
                .collect()
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_repository_file_by_path(
    repo_id: i64,
    file_path: String,
    state: State<'_, AppState>,
) -> Result<RepositoryFileResponse, String> {
    let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    file_service::get_repository_file_by_path(&conn, repo_id, &file_path)
        .map(RepositoryFileResponse::from)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_files_by_extension(
    repo_id: i64,
    extension: String,
    state: State<'_, AppState>,
) -> Result<Vec<RepositoryFileResponse>, String> {
    let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    file_service::get_files_by_extension(&conn, repo_id, &extension)
        .map(|files| {
            files
                .into_iter()
                .map(RepositoryFileResponse::from)
                .collect()
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_file_stats(
    repo_id: i64,
    state: State<'_, AppState>,
) -> Result<Vec<CommitFileStatResponse>, String> {
    let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    file_service::get_file_stats(&conn, repo_id)
        .map(|stats| {
            stats
                .into_iter()
                .map(CommitFileStatResponse::from)
                .collect()
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_file_stats_by_path(
    repo_id: i64,
    file_path: String,
    state: State<'_, AppState>,
) -> Result<Vec<CommitFileStatResponse>, String> {
    let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    file_service::get_file_stats_by_path(&conn, repo_id, &file_path)
        .map(|stats| {
            stats
                .into_iter()
                .map(CommitFileStatResponse::from)
                .collect()
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_file_hotspots(
    repo_id: i64,
    state: State<'_, AppState>,
) -> Result<Vec<FileHotspotResponse>, String> {
    let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    file_service::get_file_hotspots(&conn, repo_id)
        .map(|hotspots| {
            hotspots
                .into_iter()
                .map(FileHotspotResponse::from)
                .collect()
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_repo_languages_stats(
    repo_id: i64,
    state: State<'_, AppState>,
) -> Result<LanguageStatsResponse, String> {
    let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    file_service::get_repo_languages_stats(&conn, repo_id)
        .map(LanguageStatsResponse::from)
        .map_err(|e| e.to_string())
}
