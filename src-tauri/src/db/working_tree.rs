use crate::models::working_tree::WorkingTreeStatus;
use rusqlite::{params, Connection, Result};

pub fn insert_working_tree(
    conn: &Connection,
    repo_id: i64,
    modified_count: i32,
    staged_count: i32,
    untracked_count: i32,
    deleted_count: i32,
    renamed_count: i32,
) -> Result<i64> {
    let now = chrono::Utc::now().to_rfc3339();

    conn.execute(
        r#"
        INSERT INTO working_tree_status (
            repo_id,
            modified_count,
            staged_count,
            untracked_count,
            deleted_count,
            renamed_count,
            captured_at
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        "#,
        params![
            repo_id,
            modified_count,
            staged_count,
            untracked_count,
            deleted_count,
            renamed_count,
            now
        ],
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn update_working_tree_status(
    conn: &Connection,
    id: i64,
    modified_count: i32,
    staged_count: i32,
    untracked_count: i32,
    deleted_count: i32,
    renamed_count: i32,
) -> Result<()> {
    let now = chrono::Utc::now().to_rfc3339();

    conn.execute(
        r#"
        UPDATE working_tree_status
        SET 
            modified_count = ?2,
            staged_count = ?3,
            untracked_count = ?4,
            deleted_count = ?5,
            renamed_count = ?6,
            captured_at = ?7
        WHERE id = ?1
        "#,
        params![
            id,
            modified_count,
            staged_count,
            untracked_count,
            deleted_count,
            renamed_count,
            now
        ],
    )?;

    Ok(())
}

pub fn get_working_tree_status(conn: &Connection, repo_id: i64) -> Result<WorkingTreeStatus> {
    let mut stmt = conn.prepare(
        r#"
        SELECT 
            id,
            repo_id,
            modified_count,
            staged_count,
            untracked_count,
            deleted_count,
            renamed_count,
            captured_at
        FROM working_tree_status
        WHERE repo_id = ?1
        ORDER BY captured_at DESC
        LIMIT 1
        "#,
    )?;

    let mut rows = stmt.query(params![repo_id])?;

    if let Some(row) = rows.next()? {
        Ok(WorkingTreeStatus {
            id: row.get(0)?,
            repo_id: row.get(1)?,
            modified_count: row.get(2)?,
            staged_count: row.get(3)?,
            untracked_count: row.get(4)?,
            deleted_count: row.get(5)?,
            renamed_count: row.get(6)?,
            captured_at: row.get(7)?,
        })
    } else {
        Err(rusqlite::Error::QueryReturnedNoRows)
    }
}

pub fn get_all_working_tree_statuses(conn: &Connection) -> Result<Vec<WorkingTreeStatus>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT 
            id,
            repo_id,
            modified_count,
            staged_count,
            untracked_count,
            deleted_count,
            renamed_count,
            captured_at
        FROM working_tree_status
        ORDER BY captured_at DESC
        "#,
    )?;

    let mut rows = stmt.query([])?;

    let mut results = Vec::new();
    while let Some(row) = rows.next()? {
        results.push(WorkingTreeStatus {
            id: row.get(0)?,
            repo_id: row.get(1)?,
            modified_count: row.get(2)?,
            staged_count: row.get(3)?,
            untracked_count: row.get(4)?,
            deleted_count: row.get(5)?,
            renamed_count: row.get(6)?,
            captured_at: row.get(7)?,
        });
    }

    Ok(results)
}

pub fn delete_working_tree_status(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM working_tree_status WHERE id = ?1", params![id])?;
    Ok(())
}
