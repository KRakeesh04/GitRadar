use chrono::{DateTime, Utc};
use rusqlite::Connection;

use crate::{
    domain::{
        repository::RepositoryCalculatedMetrics, ActivityLevel, DomainError, DomainResult,
        HealthScore,
    },
    infrastructure::database::repositories::repositories,
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
