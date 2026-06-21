use crate::infrastructure::database::models::contributor::Contributor;
use rusqlite::{params, Connection, Result};

pub fn insert_contributor(
    conn: &Connection,
    repo_id: i64,
    author_name: &str,
    author_email: Option<&str>,
    commit_count: i32,
    additions: i32,
    deletions: i32,
    active_days: i32,
    last_commit_at: Option<&str>,
) -> Result<i64> {
    let updated_at = chrono::Utc::now().to_rfc3339();
    conn.execute(r#"INSERT INTO contributors (repo_id, author_name, author_email, commit_count, additions, deletions, active_days, last_commit_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)"#, params![repo_id, author_name, author_email, commit_count, additions, deletions, active_days, last_commit_at, updated_at])?;
    Ok(conn.last_insert_rowid())
}

pub fn upsert_contributor(
    conn: &Connection,
    repo_id: i64,
    author_name: &str,
    author_email: Option<&str>,
    commit_count: i32,
    additions: i32,
    deletions: i32,
    active_days: i32,
    last_commit_at: Option<&str>,
) -> Result<i64> {
    let updated_at = chrono::Utc::now().to_rfc3339();
    conn.execute(r#"INSERT OR REPLACE INTO contributors (repo_id, author_name, author_email, commit_count, additions, deletions, active_days, last_commit_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)"#, params![repo_id, author_name, author_email, commit_count, additions, deletions, active_days, last_commit_at, updated_at])?;
    Ok(conn.last_insert_rowid())
}

pub fn get_contributors_by_repo(conn: &Connection, repo_id: i64) -> Result<Vec<Contributor>> {
    let mut stmt = conn.prepare("SELECT id, repo_id, author_name, author_email, commit_count, additions, deletions, active_days, last_commit_at, updated_at FROM contributors WHERE repo_id = ?1 ORDER BY commit_count DESC")?;
    let contributors = stmt.query_map([repo_id], |row| {
        Ok(Contributor {
            id: row.get(0)?,
            repo_id: row.get(1)?,
            author_name: row.get(2)?,
            author_email: row.get(3)?,
            commit_count: row.get(4)?,
            additions: row.get(5)?,
            deletions: row.get(6)?,
            active_days: row.get(7)?,
            last_commit_at: row.get(8)?,
            updated_at: row.get(9)?,
        })
    })?;
    Ok(contributors.filter_map(Result::ok).collect())
}

pub fn get_contributor_by_email(
    conn: &Connection,
    repo_id: i64,
    author_email: &str,
) -> Result<Option<Contributor>> {
    let mut stmt = conn.prepare("SELECT id, repo_id, author_name, author_email, commit_count, additions, deletions, active_days, last_commit_at, updated_at FROM contributors WHERE repo_id = ?1 AND author_email = ?2")?;
    let contributor = stmt.query_map(params![repo_id, author_email], |row| {
        Ok(Contributor {
            id: row.get(0)?,
            repo_id: row.get(1)?,
            author_name: row.get(2)?,
            author_email: row.get(3)?,
            commit_count: row.get(4)?,
            additions: row.get(5)?,
            deletions: row.get(6)?,
            active_days: row.get(7)?,
            last_commit_at: row.get(8)?,
            updated_at: row.get(9)?,
        })
    })?;
    let result: Vec<Contributor> = contributor.filter_map(Result::ok).collect();
    Ok(result.into_iter().next())
}

pub fn update_contributor_stats(
    conn: &Connection,
    repo_id: i64,
    author_email: &str,
    commit_count: i32,
    additions: i32,
    deletions: i32,
    active_days: i32,
    last_commit_at: Option<&str>,
) -> Result<()> {
    let updated_at = chrono::Utc::now().to_rfc3339();
    conn.execute(r#"UPDATE contributors SET commit_count = ?1, additions = ?2, deletions = ?3, active_days = ?4, last_commit_at = ?5, updated_at = ?6 WHERE repo_id = ?7 AND author_email = ?8"#, params![commit_count, additions, deletions, active_days, last_commit_at, updated_at, repo_id, author_email])?;
    Ok(())
}

pub fn get_top_contributors(
    conn: &Connection,
    repo_id: i64,
    limit: Option<i32>,
) -> Result<Vec<Contributor>> {
    let mut sql = "SELECT id, repo_id, author_name, author_email, commit_count, additions, deletions, active_days, last_commit_at, updated_at FROM contributors WHERE repo_id = ?1 ORDER BY commit_count DESC".to_string();
    if let Some(limit) = limit {
        sql.push_str(&format!(" LIMIT {}", limit));
    }
    let mut stmt = conn.prepare(&sql)?;
    let contributors = stmt.query_map([repo_id], |row| {
        Ok(Contributor {
            id: row.get(0)?,
            repo_id: row.get(1)?,
            author_name: row.get(2)?,
            author_email: row.get(3)?,
            commit_count: row.get(4)?,
            additions: row.get(5)?,
            deletions: row.get(6)?,
            active_days: row.get(7)?,
            last_commit_at: row.get(8)?,
            updated_at: row.get(9)?,
        })
    })?;
    Ok(contributors.filter_map(Result::ok).collect())
}
