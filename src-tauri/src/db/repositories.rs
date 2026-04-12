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

pub fn upsert_repo_activity(
    conn: &Connection,
    repo_id: i64,
    activity_date: &str,
    commit_count: i32,
    additions: i32,
    deletions: i32,
    files_changed: i32,
) -> Result<()> {
    conn.execute(
        r#"
        INSERT OR REPLACE INTO repo_activity_daily (
            repo_id, activity_date, commit_count, additions, deletions, files_changed
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        "#,
        params![repo_id, activity_date, commit_count, additions, deletions, files_changed],
    )?;

    Ok(())
}

pub fn get_repo_activity(
    conn: &Connection,
    repo_id: i64,
    start_date: Option<&str>,
    end_date: Option<&str>,
    limit: Option<i32>,
) -> Result<Vec<crate::models::RepoActivityDaily>> {
    let mut sql = "SELECT id, repo_id, activity_date, commit_count, additions, deletions, files_changed 
                   FROM repo_activity_daily 
                   WHERE repo_id = ?1".to_string();

    if let Some(start_date) = start_date {
        sql.push_str(&format!(" AND activity_date >= '{}'", start_date));
    }
    if let Some(end_date) = end_date {
        sql.push_str(&format!(" AND activity_date <= '{}'", end_date));
    }

    sql.push_str(" ORDER BY activity_date DESC");

    if let Some(limit) = limit {
        sql.push_str(&format!(" LIMIT {}", limit));
    }

    let mut stmt = conn.prepare(&sql)?;

    let activities = stmt.query_map([repo_id], |row| {
        Ok(crate::models::RepoActivityDaily {
            id: row.get(0)?,
            repo_id: row.get(1)?,
            activity_date: row.get(2)?,
            commit_count: row.get(3)?,
            additions: row.get(4)?,
            deletions: row.get(5)?,
            files_changed: row.get(6)?,
        })
    })?;

    Ok(activities.filter_map(Result::ok).collect())
}

pub fn get_activity_summary(
    conn: &Connection,
    repo_id: i64,
    start_date: Option<&str>,
    end_date: Option<&str>,
) -> Result<(i32, i32, i32, i32)> {
    let mut sql = "SELECT SUM(commit_count), SUM(additions), SUM(deletions), SUM(files_changed) 
                   FROM repo_activity_daily 
                   WHERE repo_id = ?1".to_string();

    if let Some(start_date) = start_date {
        sql.push_str(&format!(" AND activity_date >= '{}'", start_date));
    }
    if let Some(end_date) = end_date {
        sql.push_str(&format!(" AND activity_date <= '{}'", end_date));
    }

    let mut stmt = conn.prepare(&sql)?;

    let result = stmt.query_row([repo_id], |row| {
        Ok((
            row.get::<_, Option<i32>>(0)?.unwrap_or(0),
            row.get::<_, Option<i32>>(1)?.unwrap_or(0),
            row.get::<_, Option<i32>>(2)?.unwrap_or(0),
            row.get::<_, Option<i32>>(3)?.unwrap_or(0),
        ))
    })?;

    Ok(result)
}

