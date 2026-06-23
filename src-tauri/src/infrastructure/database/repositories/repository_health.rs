use crate::infrastructure::database::models::RepositoryHealth;
use rusqlite::{params, Connection, Result};

pub fn upsert_repository_health(
    conn: &Connection,
    repo_id: i64,
    health_score: f64,
    issues_count: i32,
    warnings_count: i32,
    check_status: &str,
) -> Result<()> {
    let last_check_at = chrono::Utc::now().to_rfc3339();
    conn.execute(
        r#"
        INSERT OR REPLACE INTO repository_health (
            repo_id, health_score, issues_count, warnings_count, check_status, last_check_at
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        "#,
        params![
            repo_id,
            health_score,
            issues_count,
            warnings_count,
            check_status,
            last_check_at
        ],
    )?;
    Ok(())
}

pub fn get_repository_health(conn: &Connection, repo_id: i64) -> Result<Option<RepositoryHealth>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT repo_id, health_score, issues_count, warnings_count, check_status, last_check_at 
        FROM repository_health 
        WHERE repo_id = ?1
        "#,
    )?;
    let health = stmt.query_row([repo_id], |row| {
        Ok(RepositoryHealth {
            repo_id: row.get(0)?,
            health_score: row.get(1)?,
            issues_count: row.get(2)?,
            warnings_count: row.get(3)?,
            check_status: row.get(4)?,
            last_check_at: row.get(5)?,
        })
    });
    match health {
        Ok(h) => Ok(Some(h)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn delete_repository_health(conn: &Connection, repo_id: i64) -> Result<()> {
    conn.execute(
        "DELETE FROM repository_health WHERE repo_id = ?1",
        params![repo_id],
    )?;
    Ok(())
}
