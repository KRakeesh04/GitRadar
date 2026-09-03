use std::path::PathBuf;

use rusqlite::Connection;

use crate::{
    domain::{
        value_objects::CommitCount, ActivityLevel, DomainError, DomainResult, HealthScore,
        Repository, RepositoryId,
    },
    infrastructure::database::{
        models::repository::RepositorySummary,
        repositories::repositories::{self, PaginatedRepositories},
    },
};

fn summary_to_domain(repo: RepositorySummary) -> Repository {
    Repository {
        id: RepositoryId(repo.id),
        root_ids: repo.root_ids.clone(),
        root_id: repo.root_ids.first().copied(),
        created_at: repo.created_at,
        updated_at: repo.updated_at,
        name: repo.name,
        path: PathBuf::from(repo.path),
        git_dir: PathBuf::from(repo.git_dir_path),
        is_enabled: repo.is_enabled,
        is_starred: repo.is_starred,
        starred_at: repo.starred_at,
        health_score: HealthScore::new(repo.health_score.unwrap_or(0.0))
            .unwrap_or_else(|_| HealthScore::new(0.0).unwrap()),
        activity_level: ActivityLevel::from_weekly_commits(repo.weekly_commits.unwrap_or(0) as u32),
        default_branch: repo.default_branch,
        head_branch: repo.head_branch,
        remote_url: repo.remote_url,
        is_dirty: repo.is_dirty,
        total_commits: CommitCount::new(repo.total_commits.unwrap_or(0) as u32),
        unique_contributors: repo.unique_contributors.unwrap_or(0) as u32,
    }
}

pub fn get_repository_info_by_id(conn: &Connection, repo_id: i64) -> DomainResult<Repository> {
    let repo = match repositories::get_repository_by_id(conn, repo_id) {
        Ok(Some(repo)) => repo,
        Ok(None) => {
            return Err(DomainError::InvalidRepository(
                "Repository not found".into(),
            ))
        }
        Err(e) => {
            return Err(DomainError::InvalidRepository(format!(
                "Failed to load repository: {}",
                e
            )))
        }
    };

    Ok(summary_to_domain(repo))
}

pub fn get_all_repositories(
    conn: &Connection,
    count: usize,
    offset: usize,
) -> DomainResult<Vec<Repository>> {
    let repos = match repositories::get_all_repositories(conn, count, offset) {
        Ok(Some(repos)) => repos,
        Ok(None) => return Ok(Vec::new()),
        Err(e) => {
            return Err(DomainError::InvalidRepository(format!(
                "Failed to load repositories: {}",
                e
            )))
        }
    };

    Ok(repos.into_iter().map(summary_to_domain).collect())
}

pub fn get_repositories_by_root(
    conn: &Connection,
    root_id: i64,
) -> DomainResult<Vec<Repository>> {
    let repos = match repositories::get_repositories_by_root_id(conn, root_id) {
        Ok(repos) => repos,
        Err(e) => {
            return Err(DomainError::InvalidRepository(format!(
                "Failed to load repositories for root {}: {}",
                root_id, e
            )))
        }
    };

    Ok(repos.into_iter().map(summary_to_domain).collect())
}

pub fn get_paginated_repositories(
    conn: &Connection,
    search: Option<&str>,
    filter: Option<&str>,
    limit: usize,
    cursor: Option<i64>,
) -> DomainResult<PaginatedRepositories> {
    repositories::get_paginated_repositories(conn, search, filter, limit, cursor).map_err(|e| {
        DomainError::InvalidRepository(format!("Failed to query paginated repositories: {e}"))
    })
}

/// Search repositories via the FTS5 index (fast, ranked MATCH over name/path/remote_url).
pub fn search_repositories(
    conn: &Connection,
    query: &str,
    filter: Option<&str>,
    limit: usize,
    cursor: Option<i64>,
) -> DomainResult<PaginatedRepositories> {
    repositories::search_repositories(conn, query, filter, limit, cursor).map_err(|e| {
        DomainError::InvalidRepository(format!("Failed to FTS search repositories: {e}"))
    })
}
