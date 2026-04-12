use rusqlite::{params, Connection, Result};
use crate::models::Repository;

pub fn insert_repository(
    conn: &Connection,
    root_id: i64,
    name: &str,
    path: &str,
    git_dir_path: &str,
) -> Result<i64> {
    let now = chrono::Utc::now().to_rfc3339();

    conn.execute(
        r#"
        INSERT INTO repositories (
            root_id,
            name,
            path,
            git_dir_path,
            created_at,
            updated_at
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        "#,
        params![root_id, name, path, git_dir_path, now, now],
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn get_repository_by_id(conn: &Connection, id: i64) -> Result<Option<Repository>> {
    let mut stmt = conn.prepare(
        "SELECT id, root_id, name, path, git_dir_path, default_branch, head_branch, 
                is_dirty, last_commit_hash, last_commit_at, last_scanned_at, 
                last_indexed_at, index_status, created_at, updated_at 
         FROM repositories WHERE id = ?1"
    )?;

    let repo = stmt.query_map([id], |row| {
        Ok(Repository {
            id: row.get(0)?,
            root_id: row.get(1)?,
            name: row.get(2)?,
            path: row.get(3)?,
            git_dir_path: row.get(4)?,
            default_branch: row.get(5)?,
            head_branch: row.get(6)?,
            is_dirty: row.get(7)?,
            last_commit_hash: row.get(8)?,
            last_commit_at: row.get(9)?,
            last_scanned_at: row.get(10)?,
            last_indexed_at: row.get(11)?,
            index_status: row.get(12)?,
            created_at: row.get(13)?,
            updated_at: row.get(14)?,
        })
    })?;

    Ok(repo.filter_map(Result::ok).next())
}

pub fn get_all_repositories(conn: &Connection) -> Result<Vec<Repository>> {
	let mut stmt = conn.prepare(
        "SELECT id, root_id, name, path, git_dir_path, default_branch, head_branch, 
                is_dirty, last_commit_hash, last_commit_at, last_scanned_at, 
                last_indexed_at, index_status, created_at, updated_at 
         FROM repositories ORDER BY updated_at DESC"
	)?;

	let repos = stmt.query_map([], |row| {
			Ok(Repository {
					id: row.get(0)?,
            root_id: row.get(1)?,
            name: row.get(2)?,
            path: row.get(3)?,
            git_dir_path: row.get(4)?,
            default_branch: row.get(5)?,
            head_branch: row.get(6)?,
            is_dirty: row.get(7)?,
            last_commit_hash: row.get(8)?,
            last_commit_at: row.get(9)?,
            last_scanned_at: row.get(10)?,
            last_indexed_at: row.get(11)?,
            index_status: row.get(12)?,
            created_at: row.get(13)?,
            updated_at: row.get(14)?,
			})
	})?;

	Ok(repos.filter_map(Result::ok).collect())
}