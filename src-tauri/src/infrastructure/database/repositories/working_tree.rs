use crate::infrastructure::database::models::working_tree::WorkingTreeStatus;
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
            repo_id, modified_count, staged_count, untracked_count,
            deleted_count, renamed_count, captured_at
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

pub fn get_working_tree_status(
    conn: &Connection,
    repo_id: i64,
) -> Result<Option<WorkingTreeStatus>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT 
            id, repo_id, modified_count, staged_count, untracked_count,
            deleted_count, renamed_count, captured_at
        FROM working_tree_status
        WHERE repo_id = ?1
        ORDER BY captured_at DESC
        LIMIT 1
        "#,
    )?;

    let status = stmt.query_row(params![repo_id], |row| {
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
    });

    match status {
        Ok(s) => Ok(Some(s)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}
