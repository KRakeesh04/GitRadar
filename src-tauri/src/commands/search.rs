use serde::Serialize;
use tauri::{AppHandle, Manager, State};

use crate::{
    infrastructure::{
        database::connection::get_connection, database::models::search_index::SearchHit,
    },
    services::{search_index_service, sync_service},
    state::AppState,
};

#[derive(Debug, Serialize)]
pub struct SearchHitResponse {
    pub repo_id: i64,
    pub repo_name: String,
    pub entity_type: String,
    pub entity_id: i64,
    pub title: String,
    pub body: String,
}

impl From<SearchHit> for SearchHitResponse {
    fn from(hit: SearchHit) -> Self {
        Self {
            repo_id: hit.repo_id,
            repo_name: hit.repo_name,
            entity_type: hit.entity_type,
            entity_id: hit.entity_id,
            title: hit.title,
            body: hit.body,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub query: String,
    pub items: Vec<SearchHitResponse>,
    pub total_count: usize,
}

/// Cross-entity search across the searchable-text index (commits, contributors,
/// files, branches). Runs on a blocking thread so it never stalls the event loop.
#[tauri::command]
pub async fn search_everything(
    query: String,
    limit: Option<usize>,
    offset: Option<usize>,
    state: State<'_, AppState>,
) -> Result<SearchResponse, String> {
    let db_path = state.db_path.clone();
    tauri::async_runtime::spawn_blocking(move || -> Result<SearchResponse, String> {
        let conn = get_connection(&db_path).map_err(|e| e.to_string())?;
        let lim = limit.unwrap_or(50);
        let off = offset.unwrap_or(0);
        let hits = search_index_service::search(&conn, &query, lim, off)
            .map_err(|e| e.to_string())?;
        let total_count = hits.len();
        let items = hits
            .into_iter()
            .map(SearchHitResponse::from)
            .collect();
        Ok(SearchResponse {
            query,
            items,
            total_count,
        })
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Rebuild the entire cross-entity searchable-text index from existing DB data.
/// Used at app startup so search works before any full sync has run.
#[tauri::command]
pub async fn rebuild_search_index(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<usize, String> {
    let db_path = state.db_path.clone();
    tauri::async_runtime::spawn_blocking(move || -> Result<usize, String> {
        let mut conn = get_connection(&db_path).map_err(|e| e.to_string())?;
        search_index_service::rebuild_search_index_from_db(&mut conn).map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| {
        let _ = app;
        e.to_string()
    })?
}

#[tauri::command]
pub async fn reindex_search_index(
    repo_id: i64,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db_path = state.db_path.clone();
    tauri::async_runtime::spawn_blocking(move || -> Result<(), String> {
        let mut conn = get_connection(&db_path).map_err(|e| e.to_string())?;
        sync_service::sync_search_index(&mut conn, repo_id).map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}
