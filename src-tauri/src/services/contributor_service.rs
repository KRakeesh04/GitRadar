use rusqlite::Connection;

use crate::{
    domain::{Contributor, DomainError, DomainResult},
    infrastructure::database::{
        models::contributor::Contributor as DatabaseContributor, repositories::contributors,
    },
};

pub fn get_contributors(conn: &Connection, repo_id: i64) -> DomainResult<Vec<Contributor>> {
    contributors::get_contributors_by_repo(conn, repo_id)
        .map_err(|error| analytics_database_error("load contributors", error))?
        .into_iter()
        .map(map_contributor)
        .collect()
}

pub fn get_top_contributors(
    conn: &Connection,
    repo_id: i64,
    limit: Option<i32>,
) -> DomainResult<Vec<Contributor>> {
    contributors::get_top_contributors(conn, repo_id, limit)
        .map_err(|error| analytics_database_error("load top contributors", error))?
        .into_iter()
        .map(map_contributor)
        .collect()
}

pub fn get_contributor_by_email(
    conn: &Connection,
    repo_id: i64,
    email: &str,
) -> DomainResult<Contributor> {
    contributors::get_contributor_by_email(conn, repo_id, email)
        .map_err(|error| analytics_database_error("load contributor", error))?
        .map(map_contributor)
        .transpose()?
        .ok_or_else(|| DomainError::InvalidCommit("Contributor not found".into()))
}

fn map_contributor(contributor: DatabaseContributor) -> DomainResult<Contributor> {
    let mut domain_contributor = Contributor::new(
        contributor.id,
        contributor.repo_id,
        contributor.author_name,
        contributor.author_email.unwrap_or_default(),
    )?;
    domain_contributor.commit_count = non_negative_u32(contributor.commit_count, "commit count")?;
    domain_contributor.additions = non_negative_u32(contributor.additions, "additions")?;
    domain_contributor.deletions = non_negative_u32(contributor.deletions, "deletions")?;
    domain_contributor.active_days = non_negative_u32(contributor.active_days, "active days")?;
    domain_contributor.last_commit_at = contributor.last_commit_at;
    Ok(domain_contributor)
}

fn non_negative_u32(value: i32, field: &str) -> DomainResult<u32> {
    u32::try_from(value)
        .map_err(|_| DomainError::InvalidCommit(format!("Contributor {field} cannot be negative")))
}

fn analytics_database_error(action: &str, error: rusqlite::Error) -> DomainError {
    DomainError::InvalidRepository(format!("Failed to {action}: {error}"))
}
