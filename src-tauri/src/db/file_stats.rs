use rusqlite::{params, Connection, Result};
use crate::models::file_change::CommitFileStat;
use crate::models::file_change::FileHotspot;

pub fn insert_commit_file_stat(
    conn: &Connection,
    repo_id: i64,
    file_path: &str,
    additions: i32,
    deletions: i32,
) -> Result<i64> {
    let now = chrono::Utc::now().to_rfc3339();

    conn.execute(
        r#"
        INSERT INTO commit_file_stats (
            repo_id,
            file_path,
            additions,
            deletions,
            captured_at
        )
        VALUES (?1, ?2, ?3, ?4, ?5)
        "#,
        params![repo_id, file_path, additions, deletions, now],
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn get_file_stats(conn: &Connection, repo_id: i64) -> Result<Vec<CommitFileStat>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, repo_id, file_path, additions, deletions, captured_at
        FROM commit_file_stats
        WHERE repo_id = ?
        ORDER BY captured_at DESC
        "#,
    )?;

    let file_stats = stmt.query_map(params![repo_id], |row| {
        Ok(CommitFileStat {
            id: row.get(0)?,
            repo_id: row.get(1)?,
            file_path: row.get(2)?,
            additions: row.get(3)?,
            deletions: row.get(4)?,
            total_changes: row.get(3)? + row.get(4)?,
        })
    })?;

    let mut result = Vec::new();
    for file_stat in file_stats {
        result.push(file_stat?);
    }

    Ok(result)
}

pub fn get_file_stats_by_path(conn: &Connection, repo_id: i64, file_path: &str) -> Result<Vec<CommitFileStat>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, repo_id, file_path, additions, deletions, captured_at
        FROM commit_file_stats
        WHERE repo_id = ? AND file_path = ?
        ORDER BY captured_at DESC
        "#,
    )?;

    let file_stats = stmt.query_map(params![repo_id, file_path], |row| {
        Ok(CommitFileStat {
            id: row.get(0)?,
            repo_id: row.get(1)?,
            file_path: row.get(2)?,
            additions: row.get(3)?,
            deletions: row.get(4)?,
            total_changes: row.get(3)? + row.get(4)?,
        })
    })?;

    let mut result = Vec::new();
    for file_stat in file_stats {
        result.push(file_stat?);
    }

    Ok(result)
}

pub fn delete_file_stats(conn: &Connection, repo_id: i64) -> Result<()> {
    conn.execute(
        r#"
        DELETE FROM file_stats
        WHERE repo_id = ?
        "#,
        params![repo_id],
    )?;

    Ok(())
}

pub fn insert_file_hotspot(
    conn: &Connection,
    repo_id: i64,
    file_path: &str,
    touch_count: i32,
    churn_score: f64,
    hotspot_score: f64,
    last_touched_at: &str,
) -> Result<i64> {
    let now = chrono::Utc::now().to_rfc3339();

    conn.execute(
        r#"
        INSERT INTO file_hotspots (
            repo_id,
            file_path,
            touch_count,
            churn_score,
            hotspot_score,
            last_touched_at,
            updated_at
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        "#,
        params![repo_id, file_path, touch_count, churn_score, hotspot_score, last_touched_at, now],
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn get_file_hotspots(conn: &Connection, repo_id: i64) -> Result<Vec<FileHotspot>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, repo_id, file_path, touch_count, churn_score, hotspot_score, last_touched_at, updated_at
        FROM file_hotspots
        WHERE repo_id = ?
        ORDER BY hotspot_score DESC
        "#,
    )?;

    let file_hotspots = stmt.query_map(params![repo_id], |row| {
        Ok(FileHotspot {
            id: row.get(0)?,
            repo_id: row.get(1)?,
            file_path: row.get(2)?,
            touch_count: row.get(3)?,
            churn_score: row.get(4)?,
            hotspot_score: row.get(5)?,
            last_touched_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    })?;

    let mut result = Vec::new();
    for file_hotspot in file_hotspots {
        result.push(file_hotspot?);
    }

    Ok(result)
}

