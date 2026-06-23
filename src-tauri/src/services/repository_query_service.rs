use std::path::PathBuf;

use rusqlite::Connection;

use crate::{
    domain::{value_objects::CommitCount, DomainError, DomainResult, Repository, RepositoryId},
    infrastructure::database::repositories::repositories,
    services::sync_service::calculate_repository_metrics,
};

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

    let matrices = calculate_repository_metrics(conn, repo_id)?;

    Ok(Repository {
        id: RepositoryId(repo.id),
        name: repo.name,
        path: PathBuf::from(repo.path),
        git_dir: PathBuf::from(repo.git_dir_path),
        health_score: matrices.health_score,
        activity_level: matrices.activity_level,
        default_branch: repo.default_branch,
        head_branch: repo.head_branch,
        remote_url: repo.remote_url,
        is_dirty: repo.is_dirty,
        total_commits: CommitCount::new(matrices.total_commits),
        unique_contributors: matrices.unique_contributors,
    })
}

pub fn get_all_repositories(conn: &Connection) -> DomainResult<Vec<Repository>> {
    let repos = repositories::get_all_repositories(conn).map_err(|e| {
        DomainError::InvalidRepository(format!("Failed to load repositories: {}", e))
    })?;

    let mut result = Vec::new();
    for repo in repos {
        let repo_info = get_repository_info_by_id(conn, repo.id)?;
        result.push(repo_info);
    }

    Ok(result)
}
