use rusqlite::{Connection, Result};
use crate::models::Commit;

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
    branch_id: i64,
    parent_hashes: &[String],
) -> Result<i64> {
    let inserted_at = chrono::Utc::now().to_rfc3339();

    conn.execute(
        r#"
        INSERT INTO commits (
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
        ) 
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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

    // Link commit to branch
    conn.execute(
        r#"
        INSERT INTO commit_branches (commit_id, branch_id) 
        VALUES (?, ?)
        "#,
        params![
            commit_id,
            branch_id
        ],
    )?;

    // Insert parent relationships
    if !parent_hashes.is_empty() {
        crate::db::commit_parents::insert_commit_parents_batch(conn, repo_id, hash, parent_hashes)?;
    }
    
    Ok(commit_id)
}

pub fn get_commit_by_hash(conn: &Connection, repo_id: i64, hash: &str) -> Result<Option<Commit>> {
    let mut stmt = conn.prepare(
        "SELECT id, repo_id, hash, author_name, author_email, committer_name, committer_email, 
                subject, body, parent_count, committed_at, inserted_at 
         FROM commits 
         WHERE repo_id = ? AND hash = ?"
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
        "SELECT id, repo_id, hash, author_name, author_email, committer_name, committer_email, 
                subject, body, parent_count, committed_at, inserted_at 
         FROM commits 
         WHERE repo_id = ?
         ORDER BY committed_at DESC"
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

pub fn get_commits_by_branch(conn: &Connection, repo_id: i64, branch_name: &str) -> Result<Vec<Commit>> {
    let mut stmt = conn.prepare(
        "SELECT c.id, c.repo_id, c.hash, c.author_name, c.author_email, c.committer_name, c.committer_email,
                c.subject, c.body, c.parent_count, c.committed_at, c.inserted_at 
         FROM commits c
         JOIN commit_branches cb ON c.id = cb.commit_id
         JOIN branches b ON cb.branch_id = b.id
         WHERE c.repo_id = ? AND b.name = ?
         ORDER BY c.committed_at DESC"
    )?;
    
    let commits = stmt.query_map(params![repo_id, branch_name], |row| {
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

// Add commit to multiple branches (for merges)
pub fn add_commit_to_branches(conn: &Connection, commit_id: i64, branch_ids: &[i64]) -> Result<()> {
    for branch_id in branch_ids {
        conn.execute(
            "INSERT OR IGNORE INTO commit_branches (commit_id, branch_id) VALUES (?, ?)",
            params![commit_id, branch_id],
        )?;
    }
    Ok(())
}

// Get working tree commits (commits only on local branches)
pub fn get_working_tree_commits(conn: &Connection, repo_id: i64) -> Result<Vec<Commit>> {
    let mut stmt = conn.prepare(
        "SELECT DISTINCT c.id, c.repo_id, c.hash, c.author_name, c.author_email, c.committer_name, c.committer_email, 
                c.subject, c.body, c.parent_count, c.committed_at, c.inserted_at 
         FROM commits c
         JOIN commit_branches cb ON c.id = cb.commit_id
         JOIN branches b ON cb.branch_id = b.id
         WHERE c.repo_id = ? AND b.name NOT LIKE 'origin/%'
         ORDER BY c.committed_at DESC"
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

// Get commits with their parent relationships for graph building
pub fn get_commits_with_parents(conn: &Connection, repo_id: i64, limit: Option<i32>) -> Result<Vec<(Commit, Vec<String>)>> {
    let mut sql = "SELECT c.id, c.repo_id, c.hash, c.author_name, c.author_email, c.committer_name, c.committer_email, 
                          c.subject, c.body, c.parent_count, c.committed_at, c.inserted_at 
                   FROM commits c 
                   WHERE c.repo_id = ?1 
                   ORDER BY c.committed_at ASC".to_string();

    if let Some(limit) = limit {
        sql.push_str(&format!(" LIMIT {}", limit));
    }

    let mut stmt = conn.prepare(&sql)?;

    let commits_with_parents = stmt.query_map(params![repo_id], |row| {
        let commit = Commit {
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
        };

        // Get parents for this commit
        let parents = crate::db::commit_parents::get_commit_parents(conn, repo_id, &commit.hash)?;
        let parent_hashes: Vec<String> = parents.into_iter().map(|p| p.parent_hash).collect();

        Ok((commit, parent_hashes))
    })?;

    let mut result = Vec::new();
    for item in commits_with_parents {
        result.push(item?);
    }

    Ok(result)
}

