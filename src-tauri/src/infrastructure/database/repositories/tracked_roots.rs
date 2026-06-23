use crate::infrastructure::database::models::TrackIgnoreRoot;
use crate::infrastructure::database::models::TrackedRoot;
use rusqlite::{params, Connection, Result};

pub fn insert_tracked_root(conn: &Connection, path: &str, is_enabled: bool) -> Result<i64> {
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        r#"
        INSERT INTO tracked_roots (path, is_enabled, created_at, updated_at)
        VALUES (?1, ?2, ?3, ?4)
        "#,
        params![path, is_enabled, now, now],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn get_tracked_root_by_path(conn: &Connection, path: &str) -> Result<Option<TrackedRoot>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, path, is_enabled, created_at, updated_at 
        FROM tracked_roots 
        WHERE path = ?1
        "#,
    )?;
    let root = stmt.query_row([path], |row| {
        Ok(TrackedRoot {
            id: row.get(0)?,
            path: row.get(1)?,
            is_enabled: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        })
    });
    match root {
        Ok(r) => Ok(Some(r)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn get_all_tracked_roots(conn: &Connection) -> Result<Vec<TrackedRoot>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, path, is_enabled, created_at, updated_at 
        FROM tracked_roots 
        ORDER BY path
        "#,
    )?;
    let roots = stmt.query_map([], |row| {
        Ok(TrackedRoot {
            id: row.get(0)?,
            path: row.get(1)?,
            is_enabled: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        })
    })?;
    Ok(roots.filter_map(Result::ok).collect())
}

pub fn update_tracked_root_enabled(
    conn: &Connection,
    path: &str,
    is_enabled: bool,
) -> Result<bool> {
    let now = chrono::Utc::now().to_rfc3339();

    let rows = conn.execute(
        r#"
        UPDATE tracked_roots
        SET
            is_enabled = ?1,
            updated_at = ?2
        WHERE
            path = ?3
            AND is_enabled != ?1
        "#,
        params![is_enabled, now, path,],
    )?;

    Ok(rows > 0)
}

pub fn delete_tracked_root(conn: &Connection, id: i64) -> Result<bool> {
    let rows = conn.execute("DELETE FROM tracked_roots WHERE id = ?1", params![id])?;
    Ok(rows > 0)
}

pub fn insert_track_ignore_root(conn: &Connection, path: &str) -> Result<i64> {
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        r#"
        INSERT INTO track_ignore_roots (path, created_at, updated_at)
        VALUES (?1, ?2, ?3)
        "#,
        params![path, now, now],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn get_track_ignore_root_by_path(
    conn: &Connection,
    path: &str,
) -> Result<Option<TrackIgnoreRoot>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, path, created_at, updated_at 
        FROM track_ignore_roots 
        WHERE path = ?1
        "#,
    )?;
    let root = stmt.query_row([path], |row| {
        Ok(TrackIgnoreRoot {
            id: row.get(0)?,
            path: row.get(1)?,
            created_at: row.get(2)?,
            updated_at: row.get(3)?,
        })
    });
    match root {
        Ok(r) => Ok(Some(r)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn get_all_track_ignore_roots(conn: &Connection) -> Result<Vec<TrackIgnoreRoot>> {
    let mut stmt = conn.prepare(
        r#"
            SELECT id, path, created_at, updated_at 
            FROM track_ignore_roots 
            ORDER BY path
            "#,
    )?;
    let roots = stmt.query_map([], |row| {
        Ok(TrackIgnoreRoot {
            id: row.get(0)?,
            path: row.get(1)?,
            created_at: row.get(2)?,
            updated_at: row.get(3)?,
        })
    })?;
    Ok(roots.filter_map(Result::ok).collect())
}

pub fn delete_track_ignore_root(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM track_ignore_roots WHERE id = ?1", params![id])?;
    Ok(())
}
