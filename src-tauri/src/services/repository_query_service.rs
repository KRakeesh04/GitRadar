use std::path::PathBuf;

use rusqlite::Connection;

use crate::{
    domain::{
        value_objects::CommitCount, ActivityLevel, DomainError, DomainResult, HealthScore,
        Repository, RepositoryId,
    },
    infrastructure::database::repositories::repositories,
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

    Ok(Repository {
        id: RepositoryId(repo.id),
        root_id: repo.root_id,
        updated_at: repo.updated_at,
        name: repo.name,
        path: PathBuf::from(repo.path),
        git_dir: PathBuf::from(repo.git_dir_path),
        health_score: HealthScore::new(repo.health_score.unwrap_or(0.0))
            .unwrap_or(HealthScore::new(0.0).unwrap()),
        activity_level: ActivityLevel::from_weekly_commits(repo.weekly_commits.unwrap_or(0) as u32),
        default_branch: repo.default_branch,
        head_branch: repo.head_branch,
        remote_url: repo.remote_url,
        is_dirty: repo.is_dirty,
        total_commits: CommitCount::new(repo.total_commits.unwrap_or(0) as u32),
        unique_contributors: repo.unique_contributors.unwrap_or(0) as u32,
    })
}

pub fn get_all_repositories(
    conn: &Connection,
    count: usize,
    offset: usize,
) -> DomainResult<Vec<Repository>> {
    let mut result = Vec::new();
    let repos = match repositories::get_all_repositories(conn, count, offset) {
        Ok(Some(repos)) => repos,
        Ok(None) => return Ok(result),
        Err(e) => {
            return Err(DomainError::InvalidRepository(format!(
                "Failed to load repositories: {}",
                e
            )))
        }
    };

    for repo in repos {
        result.push(Repository {
            id: RepositoryId(repo.id),
            root_id: repo.root_id,
            updated_at: repo.updated_at,
            name: repo.name,
            path: PathBuf::from(repo.path),
            git_dir: PathBuf::from(repo.git_dir_path),
            health_score: HealthScore::new(repo.health_score.unwrap_or(0.0))
                .unwrap_or(HealthScore::new(0.0).unwrap()),
            activity_level: ActivityLevel::from_weekly_commits(
                repo.weekly_commits.unwrap_or(0) as u32
            ),
            default_branch: repo.default_branch,
            head_branch: repo.head_branch,
            remote_url: repo.remote_url,
            is_dirty: repo.is_dirty,
            total_commits: CommitCount::new(repo.total_commits.unwrap_or(0) as u32),
            unique_contributors: repo.unique_contributors.unwrap_or(0) as u32,
        });
    }
    Ok(result)
}
