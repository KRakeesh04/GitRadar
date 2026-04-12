use rusqlite::{params, Connection, Result};
use crate::models::CommitParent;

pub fn insert_commit_parent(
    conn: &Connection,
    repo_id: i64,
    commit_hash: &str,
    parent_hash: &str,
    parent_index: i32,
) -> Result<i64> {
    conn.execute(
        r#"
        INSERT INTO commit_parents (
            repo_id, commit_hash, parent_hash, parent_index
        )
        VALUES (?1, ?2, ?3, ?4)
        "#,
        params![repo_id, commit_hash, parent_hash, parent_index],
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn insert_commit_parents_batch(
    conn: &Connection,
    repo_id: i64,
    commit_hash: &str,
    parent_hashes: &[String],
) -> Result<()> {
    for (index, parent_hash) in parent_hashes.iter().enumerate() {
        insert_commit_parent(conn, repo_id, commit_hash, parent_hash, index as i32)?;
    }
    Ok(())
}

pub fn get_commit_parents(conn: &Connection, repo_id: i64, commit_hash: &str) -> Result<Vec<CommitParent>> {
    let mut stmt = conn.prepare(
        "SELECT id, repo_id, commit_hash, parent_hash, parent_index 
         FROM commit_parents 
         WHERE repo_id = ?1 AND commit_hash = ?2 
         ORDER BY parent_index ASC"
    )?;

    let parents = stmt.query_map(params![repo_id, commit_hash], |row| {
        Ok(CommitParent {
            id: row.get(0)?,
            repo_id: row.get(1)?,
            commit_hash: row.get(2)?,
            parent_hash: row.get(3)?,
            parent_index: row.get(4)?,
        })
    })?;

    Ok(parents.filter_map(Result::ok).collect())
}

pub fn get_commit_children(conn: &Connection, repo_id: i64, parent_hash: &str) -> Result<Vec<CommitParent>> {
    let mut stmt = conn.prepare(
        "SELECT id, repo_id, commit_hash, parent_hash, parent_index 
         FROM commit_parents 
         WHERE repo_id = ?1 AND parent_hash = ?2 
         ORDER BY commit_hash ASC"
    )?;

    let children = stmt.query_map(params![repo_id, parent_hash], |row| {
        Ok(CommitParent {
            id: row.get(0)?,
            repo_id: row.get(1)?,
            commit_hash: row.get(2)?,
            parent_hash: row.get(3)?,
            parent_index: row.get(4)?,
        })
    })?;

    Ok(children.filter_map(Result::ok).collect())
}

pub fn delete_commit_parents(conn: &Connection, repo_id: i64, commit_hash: &str) -> Result<i64> {
    let result = conn.execute(
        "DELETE FROM commit_parents WHERE repo_id = ?1 AND commit_hash = ?2",
        params![repo_id, commit_hash],
    )?;
    Ok(result)
}

// Get commit graph data for visualization
pub fn get_commit_graph_data(conn: &Connection, repo_id: i64, limit: Option<i32>) -> Result<Vec<(String, Vec<String>)>> {
    let mut sql = "SELECT DISTINCT cp.commit_hash, GROUP_CONCAT(cp.parent_hash, ',') 
                   FROM commit_parents cp 
                   WHERE cp.repo_id = ?1 
                   GROUP BY cp.commit_hash".to_string();

    if let Some(limit) = limit {
        sql.push_str(&format!(" LIMIT {}", limit));
    }

    let mut stmt = conn.prepare(&sql)?;

    let graph_data = stmt.query_map([repo_id], |row| {
        let commit_hash: String = row.get(0)?;
        let parents_csv: Option<String> = row.get(1)?;
        
        let parents = match parents_csv {
            Some(csv) => csv.split(',').map(|s| s.to_string()).collect(),
            None => Vec::new(),
        };

        Ok((commit_hash, parents))
    })?;

    Ok(graph_data.filter_map(Result::ok).collect())
}

// Update commit hash references
pub fn update_commit_hash_references(
    conn: &Connection,
    repo_id: i64,
    old_hash: &str,
    new_hash: &str,
) -> Result<()> {
    conn.execute(
        "UPDATE commit_parents SET commit_hash = ?1 WHERE repo_id = ?2 AND commit_hash = ?3",
        params![new_hash, repo_id, old_hash],
    )?;

    conn.execute(
        "UPDATE commit_parents SET parent_hash = ?1 WHERE repo_id = ?2 AND parent_hash = ?3",
        params![new_hash, repo_id, old_hash],
    )?;

    Ok(())
}
