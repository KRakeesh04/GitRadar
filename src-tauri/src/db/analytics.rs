use crate::models::contributor::Contributor;
use crate::models::RepoActivityDaily;
use rusqlite::{params, Connection, Result};

pub fn insert_repo_activity_daily(
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
        params![
            repo_id,
            activity_date,
            commit_count,
            additions,
            deletions,
            files_changed
        ],
    )?;

    Ok(())
}

pub fn get_repo_activity_daily(
    conn: &Connection,
    repo_id: i64,
    start_date: Option<&str>,
    end_date: Option<&str>,
) -> Result<Vec<RepoActivityDaily>> {
    let mut sql =
        "SELECT id, repo_id, activity_date, commit_count, additions, deletions, files_changed 
                   FROM repo_activity_daily WHERE repo_id = ?1"
            .to_string();

    let mut params: Vec<Box<dyn rusqlite::ToSql>> = vec![Box::new(repo_id)];
    let mut param_index = 2;

    if let Some(start_date) = start_date {
        sql.push_str(&format!(" AND activity_date >= ?{}", param_index));
        params.push(Box::new(start_date.clone()));
        param_index += 1;
    }

    if let Some(end_date) = end_date {
        sql.push_str(&format!(" AND activity_date <= ?{}", param_index));
        params.push(Box::new(end_date.clone()));
    }

    sql.push_str(" ORDER BY activity_date DESC");

    let mut stmt = conn.prepare(&sql)?;

    let activities = stmt.query_map(
        params
            .iter()
            .map(|p| p.as_ref())
            .collect::<Vec<&dyn rusqlite::ToSql>>()
            .as_slice(),
        |row| {
            Ok(RepoActivityDaily {
                id: row.get(0)?,
                repo_id: row.get(1)?,
                activity_date: row.get(2)?,
                commit_count: row.get(3)?,
                additions: row.get(4)?,
                deletions: row.get(5)?,
                files_changed: row.get(6)?,
            })
        },
    )?;

    Ok(activities.filter_map(Result::ok).collect())
}

pub fn get_contributors(conn: &Connection, repo_id: i64) -> Result<Vec<Contributor>> {
    let mut stmt = conn.prepare(
        "SELECT id, repo_id, author_name, author_email, commit_count, 
                additions, deletions, active_days, last_commit_at, updated_at 
         FROM contributors WHERE repo_id = ?1 
         ORDER BY commit_count DESC",
    )?;

    let contributors = stmt.query_map([repo_id], |row| {
        Ok(Contributor {
            id: row.get(0)?,
            repo_id: row.get(1)?,
            author_name: row.get(2)?,
            author_email: row.get(3)?,
            commit_count: row.get(4)?,
            additions: row.get(5)?,
            deletions: row.get(6)?,
            active_days: row.get(7)?,
            last_commit_at: row.get(8)?,
            updated_at: row.get(9)?,
        })
    })?;

    Ok(contributors.filter_map(Result::ok).collect())
}
