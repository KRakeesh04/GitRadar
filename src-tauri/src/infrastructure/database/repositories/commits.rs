use crate::infrastructure::database::models::commit::{Commit, CommitGraphNode};
use rusqlite::{params, Connection, Result};

pub fn upsert_commit(
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
        ON CONFLICT(repo_id, hash)
        DO UPDATE SET
            author_name = excluded.author_name,
            author_email = excluded.author_email,
            committer_name = excluded.committer_name,
            committer_email = excluded.committer_email,
            subject = excluded.subject,
            body = excluded.body,
            parent_count = excluded.parent_count,
            committed_at = excluded.committed_at
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
        super::commit_parents::upsert_commit_parents_batch(conn, repo_id, hash, parent_hashes)?;
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

pub fn get_commits_by_repo(
    conn: &Connection,
    repo_id: i64,
    count: usize,
    offset: usize,
) -> Result<Option<Vec<Commit>>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, repo_id, hash, author_name, author_email, committer_name, committer_email, subject, 
                body, parent_count, committed_at, inserted_at 
        FROM commits 
        WHERE repo_id = ? 
        ORDER BY committed_at DESC
        LIMIT ? OFFSET ?
        "#
    )?;
    let commits = stmt.query_map(params![repo_id, count, offset], |row| {
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
    Ok(Some(result))
}

pub fn get_commit_graph(
    conn: &Connection,
    repo_id: i64,
    limit: usize,
    offset: usize,
) -> Result<Vec<CommitGraphNode>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT
            c.hash,
            GROUP_CONCAT(b.name),
            c.author_name,
            c.author_email,
            c.subject,
            c.committed_at,
            COALESCE(fs.additions, 0),
            COALESCE(fs.deletions, 0),
            COALESCE(fs.file_count, 0),
            GROUP_CONCAT(cp.parent_hash)

        FROM commits c

        LEFT JOIN commit_parents cp
            ON cp.repo_id = c.repo_id
           AND cp.commit_hash = c.hash

        LEFT JOIN (
            SELECT
                repo_id,
                commit_hash,
                SUM(additions) AS additions,
                SUM(deletions) AS deletions,
                COUNT(*) AS file_count
            FROM commit_file_stats
            GROUP BY repo_id, commit_hash
        ) fs
            ON fs.repo_id = c.repo_id
           AND fs.commit_hash = c.hash

        LEFT JOIN branches b
            ON b.repo_id = c.repo_id
           AND b.last_commit_hash = c.hash

        WHERE c.repo_id = ?

        GROUP BY
            c.hash,
            c.author_name,
            c.author_email,
            c.subject,
            c.committed_at,
            fs.additions,
            fs.deletions,
            fs.file_count

        ORDER BY c.committed_at DESC

        LIMIT ? OFFSET ?
        "#,
    )?;

    let commits = stmt.query_map(params![repo_id, limit as i64, offset as i64], |row| {
        let parent_hashes = row
            .get::<_, Option<String>>(9)?
            .unwrap_or_default()
            .split(',')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect();

        let branch_names = row
            .get::<_, Option<String>>(1)?
            .unwrap_or_default()
            .split(',')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect();

        Ok(CommitGraphNode {
            hash: row.get(0)?,
            branch_names,
            author_name: row.get(2)?,
            author_email: row.get(3)?,
            subject: row.get(4)?,
            committed_at: row.get(5)?,
            additions: row.get(6)?,
            deletions: row.get(7)?,
            total_changed_files_count: row.get(8)?,
            parent_hashes,
        })
    })?;

    commits.collect()
}

pub fn get_commit_hashes_by_repo_and_file(
    conn: &Connection,
    repo_id: i64,
    file_path: &str,
    count: usize,
    offset: usize,
) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT DISTINCT c.hash
        FROM commits c
        JOIN commit_file_stats fs ON fs.repo_id = c.repo_id AND fs.commit_hash = c.hash
        WHERE c.repo_id = ? AND fs.file_path = ?
        ORDER BY c.committed_at DESC
        LIMIT ? OFFSET ?
        "#,
    )?;
    let commit_hashes =
        stmt.query_map(params![repo_id, file_path, count, offset], |row| row.get(0))?;

    let mut result = Vec::new();
    for commit_hash in commit_hashes {
        result.push(commit_hash?);
    }
    Ok(result)
}
