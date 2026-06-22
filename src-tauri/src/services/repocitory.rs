use std::path::PathBuf;

use crate::{
    domain::{ActivityLevel, DomainError, DomainResult, Repository},
    infrastructure::{
        database::repositories::{repositories, tracked_roots},
        git::{branch, repo},
    },
};
use chrono::{DateTime, Utc};
use rayon::prelude::*;
use rusqlite::Connection;

// pub fn get_repositories() -> Vec<Repository> {}

pub fn discover_repositories(conn: &mut Connection) -> Result<(), String> {
    let roots = tracked_roots::get_all_tracked_roots(conn)
        .map_err(|e| format!("Failed to load tracked roots: {}", e))?;

    let discovered: Vec<_> = roots
        .into_par_iter()
        .filter(|root| root.is_enabled)
        .flat_map(
            |root| match repo::scan_repos_from_root(&PathBuf::from(&root.path)) {
                Ok(repositories) => repositories
                    .into_iter()
                    .map(|repo| (root.id, repo))
                    .collect::<Vec<_>>(),

                Err(error) => {
                    eprintln!("Failed to scan '{}': {}", root.path, error);
                    Vec::new()
                }
            },
        )
        .collect();

    let tx = conn.transaction().map_err(|e| e.to_string())?;

    for (root_id, repo) in discovered {
        let repo_path = match repo.path.to_str() {
            Some(path) => path,
            None => continue,
        };

        let head_branch = branch::current_head_branch(repo_path).ok();

        let repo_info = match repo::get_repository_info(repo_path) {
            Ok(info) => info,
            Err(error) => {
                eprintln!("Failed to read repository info '{}': {}", repo_path, error);
                continue;
            }
        };

        repositories::upsert_repository(
            &tx,
            root_id,
            &repo.name,
            repo_path,
            repo.git_dir.to_str().unwrap_or(""),
            &format!("{:?}", repo.repo_type),
            repo_info.remote_url.as_deref(),
            repo_info.default_branch.as_deref(),
            head_branch.as_deref(),
        )
        .map_err(|e| e.to_string())?;
    }

    tx.commit().map_err(|e| e.to_string())?;

    Ok(())
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

    let is_dirty = repo.is_dirty;
    let mut repo_info = Repository::new(
        repo_id,
        repo.name,
        PathBuf::from(repo.path),
        PathBuf::from(repo.git_dir_path),
        repo.remote_url,
        repo.default_branch,
        repo.head_branch,
    )?;

    let week_ago = (Utc::now() - chrono::Duration::days(7)).to_rfc3339();
    let metrics = repositories::get_repository_metrics(conn, repo_id, &week_ago).map_err(|e| {
        DomainError::InvalidRepository(format!("Failed to load repository metrics: {e}"))
    })?;

    let total_commits = u32::try_from(metrics.total_commits.max(0)).unwrap_or(u32::MAX);
    let weekly_commits = u32::try_from(metrics.weekly_commits.max(0)).unwrap_or(u32::MAX);
    let unique_contributors = u32::try_from(metrics.unique_contributors.max(0)).unwrap_or(u32::MAX);

    let recency_score = metrics
        .last_commit_at
        .as_deref()
        .and_then(|timestamp| DateTime::parse_from_rfc3339(timestamp).ok())
        .map(
            |timestamp| match (Utc::now() - timestamp.with_timezone(&Utc)).num_days() {
                ..=7 => 1.0,
                8..=30 => 0.8,
                31..=90 => 0.6,
                91..=180 => 0.4,
                181..=365 => 0.2,
                _ => 0.0,
            },
        )
        .unwrap_or(0.0);
    let history_score = (total_commits as f32 / 100.0).min(1.0);
    let contributor_score = (unique_contributors as f32 / 3.0).min(1.0);
    let health_score = (recency_score * 0.6) + (history_score * 0.2) + (contributor_score * 0.2);

    repo_info.update_metrics(total_commits, unique_contributors);
    repo_info.set_activity_level(ActivityLevel::from_weekly_commits(weekly_commits));
    repo_info.set_health_score(health_score)?;
    repo_info.set_dirty(is_dirty);

    Ok(repo_info)
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

pub fn add_tracked_root_path(conn: &mut Connection, path: &str) -> DomainResult<i64> {
    let added = match tracked_roots::insert_tracked_root(conn, path, true) {
        Ok(added_id) => added_id,
        Err(e) => return Err(DomainError::AddTrackedRootPathFailed(e.to_string())),
    };

    Ok(added)
}

pub fn disable_track_root_path(conn: &mut Connection, path: &str) -> DomainResult<()> {
    match tracked_roots::update_tracked_root_enabled(conn, path, false) {
        Ok(_) => Ok(()),
        Err(e) => Err(DomainError::DisableTrackedRootPathFailed(e.to_string())),
    }
}
