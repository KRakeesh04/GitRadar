use crate::infrastructure::database::models::branch::Branch;
use rusqlite::{params, Connection, Result};

pub fn upsert_branch(
    conn: &Connection,
    repo_id: i64,
    name: &str,
    is_head: bool,
    is_default: bool,
    last_commit_hash: Option<&str>,
    last_commit_at: Option<&str>,
    ahead_count_from_default: i32,
    behind_count_from_default: i32,
    ahead_count_from_remote: i32,
    behind_count_from_remote: i32,
) -> Result<i64> {
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        r#"
        INSERT INTO branches (
            repo_id, name, is_head, is_default, last_commit_hash, last_commit_at, ahead_count_from_default,
            behind_count_from_default, ahead_count_from_remote, behind_count_from_remote, updated_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
        ON CONFLICT(repo_id, name)
        DO UPDATE SET
            is_head = excluded.is_head,
            is_default = excluded.is_default,
            last_commit_hash = excluded.last_commit_hash,
            last_commit_at = excluded.last_commit_at,
            ahead_count_from_default = excluded.ahead_count_from_default,
            behind_count_from_default = excluded.behind_count_from_default,
            ahead_count_from_remote = excluded.ahead_count_from_remote,
            behind_count_from_remote = excluded.behind_count_from_remote,
            updated_at = excluded.updated_at
        "#,
        params![
            repo_id, 
            name, 
            is_head, 
            is_default, 
            last_commit_hash, 
            last_commit_at, 
            ahead_count_from_default, 
            behind_count_from_default, 
            ahead_count_from_remote, 
            behind_count_from_remote, 
            now
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn get_branch_by_name(conn: &Connection, repo_id: i64, name: &str) -> Result<Option<Branch>> {
    let mut stmt = conn.prepare("
        SELECT id, repo_id, name, is_head, is_default, last_commit_hash, last_commit_at, ahead_count_from_default, 
                behind_count_from_default, ahead_count_from_remote, behind_count_from_remote, updated_at 
        FROM branches 
        WHERE repo_id = ?1 AND name = ?2"
    )?;
    let branch = stmt.query_row(params![repo_id, name], |row| {
        Ok(Branch {
            id: row.get(0)?,
            repo_id: row.get(1)?,
            name: row.get(2)?,
            is_head: row.get(3)?,
            is_default: row.get(4)?,
            last_commit_hash: row.get(5)?,
            last_commit_at: row.get(6)?,
            ahead_count_from_default: row.get(7)?,
            behind_count_from_default: row.get(8)?,
            ahead_count_from_remote: row.get(9)?,
            behind_count_from_remote: row.get(10)?,
            updated_at: row.get(11)?,
        })
    });
    match branch {
        Ok(b) => Ok(Some(b)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn get_all_branches(conn: &Connection, repo_id: i64) -> Result<Option<Vec<Branch>>> {
    let mut stmt = conn.prepare("
            SELECT id, repo_id, name, is_head, is_default, last_commit_hash, last_commit_at, ahead_count_from_default,
                    behind_count_from_default, ahead_count_from_remote, behind_count_from_remote, updated_at 
            FROM branches 
            WHERE repo_id = ?1 
            ORDER BY name
            "
        )?;
    let branches = stmt.query_map([repo_id], |row| {
        Ok(Branch {
            id: row.get(0)?,
            repo_id: row.get(1)?,
            name: row.get(2)?,
            is_head: row.get(3)?,
            is_default: row.get(4)?,
            last_commit_hash: row.get(5)?,
            last_commit_at: row.get(6)?,
            ahead_count_from_default: row.get(7)?,
            behind_count_from_default: row.get(8)?,
            ahead_count_from_remote: row.get(9)?,
            behind_count_from_remote: row.get(10)?,
            updated_at: row.get(11)?,
        })
    })?;
    Ok(Some(branches.filter_map(Result::ok).collect()))
}

pub fn get_or_create_branch(
    conn: &Connection,
    repo_id: i64,
    name: &str,
) -> Result<(i64, bool), rusqlite::Error> {
    if let Some(branch) = get_branch_by_name(conn, repo_id, name)? {
        Ok((branch.id, false))
    } else {
        let id = upsert_branch(conn, repo_id, name, false, false, None, None, 0, 0, 0, 0)?;
        Ok((id, true))
    }
}
