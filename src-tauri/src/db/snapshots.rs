use rusqlite::{params, Connection, Result};
use crate::models::Snapshot;

pub fn upsert_snapshot(
    conn: &Connection,
    repo_id: i64,
    snapshot_type: &str,
    snapshot_key: &str,
    data_json: &str,
) -> Result<()> {
    let created_at = chrono::Utc::now().to_rfc3339();

    conn.execute(
        r#"
        INSERT OR REPLACE INTO snapshots (
            id, repo_id, snapshot_type, snapshot_key, data_json, created_at
        )
        VALUES (
            COALESCE(
                (SELECT id FROM snapshots 
                 WHERE repo_id = ?1 AND snapshot_type = ?2 AND snapshot_key = ?3), 
                NULL
            ),
            ?1, ?2, ?3, ?4, ?5
        )
        "#,
        params![repo_id, snapshot_type, snapshot_key, data_json, created_at],
    )?;

    Ok(())
}

pub fn get_latest_snapshot(
    conn: &Connection,
    repo_id: i64,
    snapshot_type: &str,
    snapshot_key: &str,
) -> Result<Option<Snapshot>> {
    let mut stmt = conn.prepare(
        "SELECT id, repo_id, snapshot_type, snapshot_key, data_json, created_at 
         FROM snapshots 
         WHERE repo_id = ?1 AND snapshot_type = ?2 AND snapshot_key = ?3 
         ORDER BY created_at DESC LIMIT 1"
    )?;

    let snapshot = stmt.query_map([repo_id, snapshot_type, snapshot_key], |row| {
        Ok(Snapshot {
            id: row.get(0)?,
            repo_id: row.get(1)?,
            snapshot_type: row.get(2)?,
            snapshot_key: row.get(3)?,
            data_json: row.get(4)?,
            created_at: row.get(5)?,
        })
    })?;

    Ok(snapshot.filter_map(Result::ok).next())
}

pub fn delete_snapshots_by_repo(conn: &Connection, repo_id: i64) -> Result<i64> {
    let result = conn.execute(
        "DELETE FROM snapshots WHERE repo_id = ?1",
        params![repo_id],
    )?;

    Ok(result)
}