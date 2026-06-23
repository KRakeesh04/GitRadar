use crate::infrastructure::database::models::commit::Commit;
use rusqlite::{params, Connection, Result};

pub fn insert_commit(
    conn: &Connection,
    repo_id: i64,
    hash: &str,
    author_name: &str,
    author_email: &str,
    committer_name: &str,
    committer_email: &str,
    subject: &str,
    body: &str,
    parent_count: i64,
    committed_at: &str,
    parent_hashes: &[String],
) -> Result<i64> {
    let inserted_at = chrono::Utc::now().to_rfc3339();
    conn.execute(
        r#"
        INSERT INTO commits (
            repo_id, hash, author_name, author_email, committer_name, committer_email, subject, 
            body, parent_count, committed_at, inserted_at
        ) 
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
        "#,
        params![
            repo_id,
            hash,
            author_name,
            author_email,
            committer_name,
            committer_email,
            subject,
            body,
            parent_count,
            committed_at,
            inserted_at
        ],
    )?;
    let commit_id = conn.last_insert_rowid();
    if !parent_hashes.is_empty() {
        super::commit_parents::insert_commit_parents_batch(conn, repo_id, hash, parent_hashes)?;
    }
    Ok(commit_id)
}

pub fn get_commit_by_hash(conn: &Connection, repo_id: i64, hash: &str) -> Result<Option<Commit>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, repo_id, hash, author_name, author_email, committer_name, committer_email, subject, 
                body, parent_count, committed_at, inserted_at 
        FROM commits 
        WHERE repo_id = ? AND hash = ?
        "#
    )?;
    let commit = stmt.query_row(params![repo_id, hash], |row| {
        Ok(Commit {
            id: row.get(0)?,
            repo_id: row.get(1)?,
            hash: row.get(2)?,
            author_name: row.get(3)?,
            author_email: row.get(4)?,
            committer_name: row.get(5)?,
            committer_email: row.get(6)?,
            subject: row.get(7)?,
            body: row.get(8)?,
            parent_count: row.get(9)?,
            committed_at: row.get(10)?,
            inserted_at: row.get(11)?,
        })
    });
    match commit {
        Ok(c) => Ok(Some(c)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn get_commits_by_repo(conn: &Connection, repo_id: i64) -> Result<Vec<Commit>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, repo_id, hash, author_name, author_email, committer_name, committer_email, subject, 
                body, parent_count, committed_at, inserted_at 
        FROM commits 
        WHERE repo_id = ? 
        ORDER BY committed_at DESC
        "#
    )?;
    let commits = stmt.query_map(params![repo_id], |row| {
        Ok(Commit {
            id: row.get(0)?,
            repo_id: row.get(1)?,
            hash: row.get(2)?,
            author_name: row.get(3)?,
            author_email: row.get(4)?,
            committer_name: row.get(5)?,
            committer_email: row.get(6)?,
            subject: row.get(7)?,
            body: row.get(8)?,
            parent_count: row.get(9)?,
            committed_at: row.get(10)?,
            inserted_at: row.get(11)?,
        })
    })?;
    let mut result = Vec::new();
    for commit in commits {
        result.push(commit?);
    }
    Ok(result)
}
