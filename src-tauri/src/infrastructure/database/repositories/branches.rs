use crate::infrastructure::database::models::branch::Branch;
use rusqlite::{params, Connection, Result};

pub fn insert_branch(
    conn: &Connection,
    repo_id: i64,
    name: &str,
    is_head: bool,
    is_default: bool,
    last_commit_hash: Option<&str>,
    last_commit_at: Option<&str>,
    ahead_count: i32,
    behind_count: i32,
) -> Result<i64> {
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(r#"INSERT INTO branches (repo_id, name, is_head, is_default, last_commit_hash, last_commit_at, ahead_count, behind_count, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)"#, params![repo_id, name, is_head, is_default, last_commit_hash, last_commit_at, ahead_count, behind_count, now])?;
    Ok(conn.last_insert_rowid())
}

pub fn get_branch_by_name(conn: &Connection, repo_id: i64, name: &str) -> Result<Option<Branch>> {
    let mut stmt = conn.prepare("SELECT id, repo_id, name, is_head, is_default, last_commit_hash, last_commit_at, ahead_count, behind_count, updated_at FROM branches WHERE repo_id = ?1 AND name = ?2")?;
    let branch = stmt.query_row(params![repo_id, name], |row| {
        Ok(Branch {
            id: row.get(0)?,
            repo_id: row.get(1)?,
            name: row.get(2)?,
            is_head: row.get(3)?,
            is_default: row.get(4)?,
            last_commit_hash: row.get(5)?,
            last_commit_at: row.get(6)?,
            ahead_count: row.get(7)?,
            behind_count: row.get(8)?,
            updated_at: row.get(9)?,
        })
    });
    match branch {
        Ok(b) => Ok(Some(b)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn get_all_branches(conn: &Connection, repo_id: i64) -> Result<Vec<Branch>> {
    let mut stmt = conn.prepare("SELECT id, repo_id, name, is_head, is_default, last_commit_hash, last_commit_at, ahead_count, behind_count, updated_at FROM branches WHERE repo_id = ?1 ORDER BY name")?;
    let branches = stmt.query_map([repo_id], |row| {
        Ok(Branch {
            id: row.get(0)?,
            repo_id: row.get(1)?,
            name: row.get(2)?,
            is_head: row.get(3)?,
            is_default: row.get(4)?,
            last_commit_hash: row.get(5)?,
            last_commit_at: row.get(6)?,
            ahead_count: row.get(7)?,
            behind_count: row.get(8)?,
            updated_at: row.get(9)?,
        })
    })?;
    Ok(branches.filter_map(Result::ok).collect())
}

pub fn get_or_create_branch(
    conn: &Connection,
    repo_id: i64,
    name: &str,
) -> Result<(i64, bool), rusqlite::Error> {
    if let Some(branch) = get_branch_by_name(conn, repo_id, name)? {
        Ok((branch.id, false))
    } else {
        let id = insert_branch(conn, repo_id, name, false, false, None, None, 0, 0)?;
        Ok((id, true))
    }
}
