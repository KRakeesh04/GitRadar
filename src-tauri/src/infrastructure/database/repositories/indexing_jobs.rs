use crate::infrastructure::database::models::IndexingJob;
use rusqlite::{params, Connection, Result};

pub fn create_indexing_job(conn: &Connection, repo_id: i64, job_type: &str) -> Result<i64> {
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        r#"
        INSERT INTO indexing_jobs (
            repo_id, job_type, status, progress, total_items, processed_items, 
            created_at, updated_at
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
        "#,
        params![repo_id, job_type, "pending", 0, None::<i32>, 0, now, now],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn mark_indexing_job_started(conn: &Connection, job_id: i64) -> Result<()> {
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        r#"
        UPDATE indexing_jobs
        SET status = 'running', started_at = COALESCE(started_at, ?1), updated_at = ?1
        WHERE id = ?2
        "#,
        params![now, job_id],
    )?;
    Ok(())
}

pub fn update_indexing_job_progress(
    conn: &Connection,
    job_id: i64,
    progress: i32,
    processed_items: i32,
) -> Result<()> {
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        r#"
        UPDATE indexing_jobs 
        SET 
            progress = ?1, 
            processed_items = ?2, 
            updated_at = ?3 
        WHERE id = ?4
        "#,
        params![progress, processed_items, now, job_id],
    )?;
    Ok(())
}

pub fn update_indexing_job_status(
    conn: &Connection,
    job_id: i64,
    status: &str,
    error_message: Option<&str>,
) -> Result<()> {
    let now = chrono::Utc::now().to_rfc3339();
    let completed_at = if status == "completed" || status == "failed" {
        Some(now.clone())
    } else {
        None
    };

    if let Some(completed_at) = completed_at {
        conn.execute(
            r#"
            UPDATE indexing_jobs 
            SET 
                status = ?1, 
                error_message = ?2, 
                completed_at = ?3, 
                updated_at = ?4 
            WHERE id = ?5
            "#,
            params![status, error_message, completed_at, now, job_id],
        )?;
    } else {
        conn.execute(
            r#"
            UPDATE indexing_jobs 
            SET 
                status = ?1, 
                error_message = ?2, 
                updated_at = ?3 
            WHERE id = ?4
            "#,
            params![status, error_message, now, job_id],
        )?;
    }
    Ok(())
}

pub fn complete_indexing_job(conn: &Connection, job_id: i64) -> Result<()> {
    update_indexing_job_status(conn, job_id, "completed", None)
}

pub fn fail_indexing_job(conn: &Connection, job_id: i64, error_message: &str) -> Result<()> {
    update_indexing_job_status(conn, job_id, "failed", Some(error_message))
}

pub fn get_indexing_job(conn: &Connection, job_id: i64) -> Result<Option<IndexingJob>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, repo_id, job_type, status, progress, total_items, processed_items, 
            error_message, started_at, completed_at, created_at, updated_at 
        FROM indexing_jobs 
        WHERE id = ?1
        "#,
    )?;
    let job = stmt.query_row([job_id], |row| {
        Ok(IndexingJob {
            id: row.get(0)?,
            repo_id: row.get(1)?,
            job_type: row.get(2)?,
            status: row.get(3)?,
            progress: row.get(4)?,
            total_items: row.get(5)?,
            processed_items: row.get(6)?,
            error_message: row.get(7)?,
            started_at: row.get(8)?,
            completed_at: row.get(9)?,
            created_at: row.get(10)?,
            updated_at: row.get(11)?,
        })
    });
    match job {
        Ok(j) => Ok(Some(j)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn get_indexing_jobs_by_repo(
    conn: &Connection,
    repo_id: i64,
    limit: Option<i32>,
) -> Result<Vec<IndexingJob>> {
    let mut sql = r#"
        SELECT id, repo_id, job_type, status, progress, total_items, processed_items, 
            error_message, started_at, completed_at, created_at, updated_at 
        FROM indexing_jobs 
        WHERE repo_id = ?1 
        ORDER BY created_at DESC
    "#
    .to_string();

    if let Some(limit) = limit {
        sql.push_str(&format!(" LIMIT {}", limit));
    }

    let mut stmt = conn.prepare(&sql)?;
    let jobs = stmt.query_map([repo_id], |row| {
        Ok(IndexingJob {
            id: row.get(0)?,
            repo_id: row.get(1)?,
            job_type: row.get(2)?,
            status: row.get(3)?,
            progress: row.get(4)?,
            total_items: row.get(5)?,
            processed_items: row.get(6)?,
            error_message: row.get(7)?,
            started_at: row.get(8)?,
            completed_at: row.get(9)?,
            created_at: row.get(10)?,
            updated_at: row.get(11)?,
        })
    })?;
    Ok(jobs.filter_map(Result::ok).collect())
}

pub fn get_pending_indexing_jobs(conn: &Connection) -> Result<Vec<IndexingJob>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, repo_id, job_type, status, progress, total_items, processed_items, 
            error_message, started_at, completed_at, created_at, updated_at 
        FROM indexing_jobs 
        WHERE status = 'pending' 
        ORDER BY created_at ASC
        "#,
    )?;
    let jobs = stmt.query_map([], |row| {
        Ok(IndexingJob {
            id: row.get(0)?,
            repo_id: row.get(1)?,
            job_type: row.get(2)?,
            status: row.get(3)?,
            progress: row.get(4)?,
            total_items: row.get(5)?,
            processed_items: row.get(6)?,
            error_message: row.get(7)?,
            started_at: row.get(8)?,
            completed_at: row.get(9)?,
            created_at: row.get(10)?,
            updated_at: row.get(11)?,
        })
    })?;
    Ok(jobs.filter_map(Result::ok).collect())
}

pub fn delete_indexing_job(conn: &Connection, job_id: i64) -> Result<()> {
    conn.execute("DELETE FROM indexing_jobs WHERE id = ?1", params![job_id])?;
    Ok(())
}

pub fn delete_old_indexing_jobs(conn: &Connection, days_old: i32) -> Result<i64> {
    let cutoff_date = chrono::Utc::now() - chrono::Duration::days(days_old as i64);
    let cutoff_str = cutoff_date.to_rfc3339();
    let result = conn.execute(
        "DELETE FROM indexing_jobs WHERE created_at < ?1",
        params![cutoff_str],
    )?;
    Ok(result as i64)
}

pub fn cleanup_completed_indexing_jobs(conn: &Connection, days_old: i32) -> Result<i64> {
    delete_old_indexing_jobs(conn, days_old)
}
