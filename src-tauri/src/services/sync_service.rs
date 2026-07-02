use chrono::{DateTime, Utc};
use rusqlite::Connection;

use crate::{
    domain::{
        repository::RepositoryCalculatedMetrics, ActivityLevel, DomainError, DomainResult,
        HealthScore,
    },
    infrastructure::{
        database::repositories::{branches, repositories},
        git,
    },
};

pub fn calculate_repository_metrics(
    conn: &Connection,
    repo_id: i64,
) -> DomainResult<RepositoryCalculatedMetrics> {
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

    Ok(RepositoryCalculatedMetrics {
        total_commits,
        weekly_commits,
        unique_contributors,
        health_score: HealthScore::new(health_score)
            .unwrap_or_else(|_| HealthScore::new(0.0).unwrap()),
        activity_level: ActivityLevel::from_weekly_commits(weekly_commits),
    })
}

// TODO-01: discover branches and store in db
pub fn sync_repository(conn: &Connection, repo_id: i64) -> DomainResult<()> {
    sync_branches(conn, repo_id)
}

pub fn sync_branches(conn: &Connection, repo_id: i64) -> DomainResult<()> {
    let repo_path = match repositories::get_repository_path(conn, repo_id) {
        Ok(Some(path)) => path,
        Ok(None) => {
            return Err(DomainError::InvalidRepository(
                "Repository Not Found".into(),
            ))
        }
        Err(error) => {
            return Err(DomainError::InvalidRepository(format!(
                "Failed to load repo path: {error}"
            )))
        }
    };

    let branches = match git::branch::get_branches(&repo_path) {
        Ok(br) => br,
        Err(error) => {
            return Err(DomainError::InvalidBranch(format!(
                "Failed to find branches from git2 : {error}"
            )))
        }
    };

    let repo_info = git::repo::get_repository_info(&repo_path).unwrap();
    let default_branch = repo_info.default_branch.unwrap_or("".into());
    for branch in branches {
        let is_default = branch.name == default_branch;
        let last_commit =
            git::commit::last_commit_info_by_branch(&repo_path, &branch.name).unwrap();
        let ahead_behind_from_default = if is_default {
            git::commit::find_ahead_behind_given_vs_default(
                &repo_path,
                &default_branch,
                &branch.name,
            )
            .unwrap()
        } else {
            (0, 0)
        };
        let ahead_behind_from_remote =
            git::commit::find_ahead_behind_local_vs_remote(&repo_path, &branch.name).unwrap();

        branches::upsert_branch(
            conn,
            repo_id,
            &branch.name,
            branch.is_head,
            is_default,
            Some(last_commit.hash.as_str()),
            Some(last_commit.committed_at.as_str()),
            ahead_behind_from_default.0,
            ahead_behind_from_default.1,
            ahead_behind_from_remote.0,
            ahead_behind_from_remote.1,
        )
        .map_err(|e| DomainError::InvalidBranch(format!("Failed to upsert branch in db: {e}")))?;
    }

    Ok(())
}
// TODO-02: discover commits and store in db and update commits stats in db
// TODO-03: discover contributors and store in db
// TODO-04: discover repo files and store meta data in db
// TODO-05: update working_tree_status in db
// TODO-06: repository_health, repo_activity_daily, file_hotspots details and store them in db
