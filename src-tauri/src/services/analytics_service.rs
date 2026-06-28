use rusqlite::Connection;

use crate::{
    domain::{DomainError, DomainResult, RepositoryActivityDaily},
    infrastructure::database::{
        models::analytics::RepoActivityDaily as DatabaseActivity, repositories::analytics,
    },
};

pub fn get_repository_activity(
    conn: &Connection,
    repo_id: i64,
    start_date: Option<&str>,
    end_date: Option<&str>,
) -> DomainResult<Vec<RepositoryActivityDaily>> {
    analytics::get_repo_activity_daily(conn, repo_id, start_date, end_date)
        .map_err(|error| analytics_database_error("load repository activity", error))
        .map(|activity| activity.into_iter().map(map_activity).collect())
}

fn map_activity(activity: DatabaseActivity) -> RepositoryActivityDaily {
    RepositoryActivityDaily {
        id: activity.id,
        repo_id: activity.repo_id,
        activity_date: activity.activity_date,
        commit_count: activity.commit_count,
        additions: activity.additions,
        deletions: activity.deletions,
        files_changed: activity.files_changed,
    }
}

fn analytics_database_error(action: &str, error: rusqlite::Error) -> DomainError {
    DomainError::InvalidRepository(format!("Failed to {action}: {error}"))
}
