use crate::models::setting::Setting;
use rusqlite::{params, Connection, Result};

pub fn get_setting(conn: &Connection, key: &str) -> Result<Option<Setting>> {
    let mut stmt = conn.prepare("SELECT key, value, updated_at FROM settings WHERE key = ?1")?;

    let setting = stmt.query_map([key], |row| {
        Ok(Setting {
            key: row.get(0)?,
            value: row.get(1)?,
            updated_at: row.get(2)?,
        })
    })?;

    let result: Vec<Setting> = setting.filter_map(Result::ok).collect();
    Ok(result.into_iter().next())
}

pub fn upsert_setting(conn: &Connection, key: &str, value: &str) -> Result<()> {
    let updated_at = chrono::Utc::now().to_rfc3339();

    conn.execute(
        r#"
        INSERT OR REPLACE INTO settings (key, value, updated_at)
        VALUES (?1, ?2, ?3)
        "#,
        params![key, value, updated_at],
    )?;

    Ok(())
}

pub fn get_all_settings(conn: &Connection) -> Result<Vec<Setting>> {
    let mut stmt = conn.prepare("SELECT key, value, updated_at FROM settings ORDER BY key")?;

    let settings = stmt.query_map([], |row| {
        Ok(Setting {
            key: row.get(0)?,
            value: row.get(1)?,
            updated_at: row.get(2)?,
        })
    })?;

    Ok(settings.filter_map(Result::ok).collect())
}

pub fn insert_default_settings(conn: &Connection) -> Result<()> {
    let defaults = vec![
        ("scan_interval_minutes", "30"),
        ("max_repo_depth", "3"),
        ("enable_file_watching", "true"),
        ("theme", "system"),
        ("language", "en"),
    ];

    for (key, value) in defaults {
        upsert_setting(conn, key, value)?;
    }

    Ok(())
}
