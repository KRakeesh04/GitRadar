use serde::Serialize;
use tauri::State;

use crate::{
    domain::{Repository, TrackedRoot},
    infrastructure::database::{
        connection::get_connection, repositories::repositories,
    },
    services::{repository_discovery_service, repository_query_service, tracked_root_service},
    state::AppState,
};

#[derive(Debug, Serialize)]
pub struct RepositoryResponse {
    pub id: i64,
    pub root_ids: Vec<i64>,
    pub root_id: Option<i64>,
    pub is_enabled: bool,
    pub is_starred: bool,
    pub starred_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
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
            root_ids: repository.root_ids.clone(),
            root_id: repository.root_id.or_else(|| repository.root_ids.first().copied()),
            is_enabled: repository.is_enabled,
            is_starred: repository.is_starred,
            starred_at: repository.starred_at,
            created_at: repository.created_at,
            updated_at: repository.updated_at,
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

#[derive(Debug, Serialize)]
pub struct PaginatedRepositoriesResponse {
    pub items: Vec<RepositoryResponse>,
    pub next_cursor: Option<i64>,
    pub has_more: bool,
    pub total_count: usize,
}

#[derive(Debug, Serialize)]
pub struct TrackedRootResponse {
    pub id: i64,
    pub path: String,
    pub is_enabled: bool,
    pub updated_at: String,
}

impl From<TrackedRoot> for TrackedRootResponse {
    fn from(root: TrackedRoot) -> Self {
        Self {
            id: root.id.0,
            path: root.path.to_string_lossy().into_owned(),
            is_enabled: root.is_enabled,
            updated_at: root.updated_at,
        }
    }
}

#[tauri::command]
pub fn get_repository_info(
    repo_id: i64,
    state: State<'_, AppState>,
) -> Result<RepositoryResponse, String> {
    let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    repository_query_service::get_repository_info_by_id(&conn, repo_id)
        .map(RepositoryResponse::from)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_all_repositories(state: State<'_, AppState>) -> Result<Vec<RepositoryResponse>, String> {
    let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    repository_query_service::get_all_repositories(&conn, 500, 0)
        .map(|repositories| {
            repositories
                .into_iter()
                .map(RepositoryResponse::from)
                .collect()
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_repositories_by_root_id(
    root_id: i64,
    state: State<'_, AppState>,
) -> Result<Vec<RepositoryResponse>, String> {
    let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    repository_query_service::get_repositories_by_root(&conn, root_id)
        .map(|repositories| {
            repositories
                .into_iter()
                .map(RepositoryResponse::from)
                .collect()
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_paginated_repositories(
    search: Option<String>,
    filter: Option<String>,
    limit: Option<usize>,
    cursor: Option<i64>,
    state: State<'_, AppState>,
) -> Result<PaginatedRepositoriesResponse, String> {
    let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    let page_limit = limit.unwrap_or(20);
    let result = repository_query_service::get_paginated_repositories(
        &conn,
        search.as_deref(),
        filter.as_deref(),
        page_limit,
        cursor,
    )
    .map_err(|e| e.to_string())?;

    let items = result
        .items
        .into_iter()
        .map(|r| RepositoryResponse {
            id: r.id,
            root_ids: r.root_ids.clone(),
            root_id: r.root_ids.first().copied(),
            is_enabled: r.is_enabled,
            is_starred: r.is_starred,
            starred_at: r.starred_at,
            created_at: r.created_at,
            updated_at: r.updated_at,
            name: r.name,
            path: r.path,
            git_dir: r.git_dir_path,
            health_score: r.health_score.unwrap_or(0.0),
            activity_level: format!("{:?}", crate::domain::ActivityLevel::from_weekly_commits(r.weekly_commits.unwrap_or(0) as u32)),
            default_branch: r.default_branch,
            head_branch: r.head_branch,
            remote_url: r.remote_url,
            is_dirty: r.is_dirty,
            total_commits: r.total_commits.unwrap_or(0) as u32,
            unique_contributors: r.unique_contributors.unwrap_or(0) as u32,
        })
        .collect();

    Ok(PaginatedRepositoriesResponse {
        items,
        next_cursor: result.next_cursor,
        has_more: result.has_more,
        total_count: result.total_count,
    })
}

#[tauri::command]
pub fn set_repository_enabled(
    repo_id: i64,
    enabled: bool,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    let mut conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    repositories::set_repository_enabled(&mut conn, repo_id, enabled).map_err(|e| e.to_string())
}

/// FTS5-backed repository search. Runs on a blocking thread so it never stalls
/// Tauri's event loop, then returns the same paginated shape as the list query.
#[tauri::command]
pub async fn search_repositories(
    query: String,
    filter: Option<String>,
    limit: Option<usize>,
    cursor: Option<i64>,
    state: State<'_, AppState>,
) -> Result<PaginatedRepositoriesResponse, String> {
    let db_path = state.db_path.clone();
    tauri::async_runtime::spawn_blocking(move || -> Result<PaginatedRepositoriesResponse, String> {
        let conn = get_connection(&db_path).map_err(|e| e.to_string())?;
        let page_limit = limit.unwrap_or(20);
        let result = repository_query_service::search_repositories(
            &conn,
            &query,
            filter.as_deref(),
            page_limit,
            cursor,
        )
        .map_err(|e| e.to_string())?;

        let items = result
            .items
            .into_iter()
            .map(|r| RepositoryResponse {
                id: r.id,
                root_ids: r.root_ids.clone(),
                root_id: r.root_ids.first().copied(),
                is_enabled: r.is_enabled,
                is_starred: r.is_starred,
                starred_at: r.starred_at,
                created_at: r.created_at,
                updated_at: r.updated_at,
                name: r.name,
                path: r.path,
                git_dir: r.git_dir_path,
                health_score: r.health_score.unwrap_or(0.0),
                activity_level: format!(
                    "{:?}",
                    crate::domain::ActivityLevel::from_weekly_commits(
                        r.weekly_commits.unwrap_or(0) as u32
                    )
                ),
                default_branch: r.default_branch,
                head_branch: r.head_branch,
                remote_url: r.remote_url,
                is_dirty: r.is_dirty,
                total_commits: r.total_commits.unwrap_or(0) as u32,
                unique_contributors: r.unique_contributors.unwrap_or(0) as u32,
            })
            .collect();

        Ok(PaginatedRepositoriesResponse {
            items,
            next_cursor: result.next_cursor,
            has_more: result.has_more,
            total_count: result.total_count,
        })
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub fn discover_repositories(state: State<'_, AppState>) -> Result<(), String> {
    let mut conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    repository_discovery_service::discover_repositories(&mut conn)
}

#[tauri::command]
pub fn add_tracked_root_path(path: String, state: State<'_, AppState>) -> Result<i64, String> {
    let mut conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    tracked_root_service::add_tracked_root_path(&mut conn, &path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_all_tracked_root_paths(
    state: State<'_, AppState>,
) -> Result<Vec<TrackedRootResponse>, String> {
    let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    tracked_root_service::get_all_tracked_root_paths(&conn)
        .map(|roots| roots.into_iter().map(TrackedRootResponse::from).collect())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_tracked_root_enabled(
    path: String,
    enabled: bool,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    let mut conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    tracked_root_service::enable_or_disable_track_root_path(&mut conn, &path, enabled)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_tracked_root_path(root_id: i64, state: State<'_, AppState>) -> Result<bool, String> {
    let mut conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    tracked_root_service::delete_tracked_root_path(&mut conn, root_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_repository_starred(
    repo_id: i64,
    is_starred: bool,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    let mut conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    repositories::set_repository_starred(&mut conn, repo_id, is_starred).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_starred_repositories(
    limit: Option<usize>,
    offset: Option<usize>,
    state: State<'_, AppState>,
) -> Result<Vec<RepositoryResponse>, String> {
    let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    let limit = limit.unwrap_or(10);
    let offset = offset.unwrap_or(0);
    repositories::get_starred_repositories(&conn, limit, offset)
        .map(|repos| {
            repos
                .into_iter()
                .map(|r| RepositoryResponse {
                    id: r.id,
                    root_ids: r.root_ids.clone(),
                    root_id: r.root_ids.first().copied(),
                    is_enabled: r.is_enabled,
                    is_starred: r.is_starred,
                    starred_at: r.starred_at,
                    created_at: r.created_at,
                    updated_at: r.updated_at,
                    name: r.name,
                    path: r.path,
                    git_dir: r.git_dir_path,
                    health_score: r.health_score.unwrap_or(0.0),
                    activity_level: format!(
                        "{:?}",
                        crate::domain::ActivityLevel::from_weekly_commits(r.weekly_commits.unwrap_or(0) as u32)
                    ),
                    default_branch: r.default_branch,
                    head_branch: r.head_branch,
                    remote_url: r.remote_url,
                    is_dirty: r.is_dirty,
                    total_commits: r.total_commits.unwrap_or(0) as u32,
                    unique_contributors: r.unique_contributors.unwrap_or(0) as u32,
                })
                .collect()
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_recent_repositories(
    limit: Option<usize>,
    offset: Option<usize>,
    state: State<'_, AppState>,
) -> Result<Vec<RepositoryResponse>, String> {
    let conn = get_connection(&state.db_path).map_err(|e| e.to_string())?;
    let limit = limit.unwrap_or(10);
    let offset = offset.unwrap_or(0);
    repositories::get_recent_repositories(&conn, limit, offset)
        .map(|repos| {
            repos
                .into_iter()
                .map(|r| RepositoryResponse {
                    id: r.id,
                    root_ids: r.root_ids.clone(),
                    root_id: r.root_ids.first().copied(),
                    is_enabled: r.is_enabled,
                    is_starred: r.is_starred,
                    starred_at: r.starred_at,
                    created_at: r.created_at,
                    updated_at: r.updated_at,
                    name: r.name,
                    path: r.path,
                    git_dir: r.git_dir_path,
                    health_score: r.health_score.unwrap_or(0.0),
                    activity_level: format!(
                        "{:?}",
                        crate::domain::ActivityLevel::from_weekly_commits(r.weekly_commits.unwrap_or(0) as u32)
                    ),
                    default_branch: r.default_branch,
                    head_branch: r.head_branch,
                    remote_url: r.remote_url,
                    is_dirty: r.is_dirty,
                    total_commits: r.total_commits.unwrap_or(0) as u32,
                    unique_contributors: r.unique_contributors.unwrap_or(0) as u32,
                })
                .collect()
        })
        .map_err(|e| e.to_string())
}
