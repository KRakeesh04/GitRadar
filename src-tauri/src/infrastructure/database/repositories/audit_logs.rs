use crate::infrastructure::database::models::AuditLog;
use rusqlite::{params, Connection, Result};

pub fn insert_audit_log(
    conn: &Connection,
    action: &str,
    entity_type: &str,
    entity_id: &str,
    details: Option<&str>,
) -> Result<i64> {
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        r#"
        INSERT INTO audit_logs (action, entity_type, entity_id, details, created_at)
        VALUES (?1, ?2, ?3, ?4, ?5)
        "#,
        params![action, entity_type, entity_id, details, now],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn get_audit_logs(
    conn: &Connection,
    entity_type: Option<&str>,
    entity_id: Option<&str>,
    limit: Option<i32>,
) -> Result<Vec<AuditLog>> {
    let mut sql = "SELECT id, action, entity_type, entity_id, details, created_at FROM audit_logs"
        .to_string();

    match (entity_type, entity_id) {
        (Some(et), Some(ei)) => {
            sql.push_str(&format!(
                " WHERE entity_type = '{}' AND entity_id = '{}' ORDER BY created_at DESC",
                et, ei
            ));
        }
        (Some(et), None) => {
            sql.push_str(&format!(
                " WHERE entity_type = '{}' ORDER BY created_at DESC",
                et
            ));
        }
        (None, Some(ei)) => {
            sql.push_str(&format!(
                " WHERE entity_id = '{}' ORDER BY created_at DESC",
                ei
            ));
        }
        (None, None) => {
            sql.push_str(" ORDER BY created_at DESC");
        }
    }

    if let Some(limit) = limit {
        sql.push_str(&format!(" LIMIT {}", limit));
    }

    let mut stmt = conn.prepare(&sql)?;

    let logs = stmt.query_map([], |row| {
        Ok(AuditLog {
            id: row.get(0)?,
            action: row.get(1)?,
            entity_type: row.get(2)?,
            entity_id: row.get(3)?,
            details: row.get(4)?,
            created_at: row.get(5)?,
        })
    })?;
    Ok(logs.filter_map(Result::ok).collect())
}

pub fn get_audit_logs_by_action(
    conn: &Connection,
    action: &str,
    limit: Option<i32>,
) -> Result<Vec<AuditLog>> {
    let mut sql = "SELECT id, action, entity_type, entity_id, details, created_at 
                   FROM audit_logs WHERE action = ?1 ORDER BY created_at DESC"
        .to_string();

    if let Some(limit) = limit {
        sql.push_str(&format!(" LIMIT {}", limit));
    }

    let mut stmt = conn.prepare(&sql)?;
    let logs = stmt.query_map([action], |row| {
        Ok(AuditLog {
            id: row.get(0)?,
            action: row.get(1)?,
            entity_type: row.get(2)?,
            entity_id: row.get(3)?,
            details: row.get(4)?,
            created_at: row.get(5)?,
        })
    })?;
    Ok(logs.filter_map(Result::ok).collect())
}

pub fn delete_old_audit_logs(conn: &Connection, days_old: i32) -> Result<i64> {
    let cutoff_date = chrono::Utc::now() - chrono::Duration::days(days_old as i64);
    let cutoff_str = cutoff_date.to_rfc3339();
    let result = conn.execute(
        "DELETE FROM audit_logs WHERE created_at < ?1",
        params![cutoff_str],
    )?;
    Ok(result as i64)
}
