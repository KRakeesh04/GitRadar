use crate::infrastructure::database::models::repository::RepositorySummary;
use rusqlite::{params, Connection, Result};

pub struct RepositoryMetrics {
    pub total_commits: i64,
    pub weekly_commits: i64,
    pub last_commit_at: Option<String>,
    pub unique_contributors: i64,
}

pub fn upsert_repository(
    conn: &Connection,
    name: &str,
    path: &str,
    git_dir_path: &str,
    repo_type: &str,
    remote_url: Option<&str>,
    default_branch: Option<&str>,
    head_branch: Option<&str>,
) -> Result<i64> {
    let now = chrono::Utc::now().to_rfc3339();

    conn.execute(
        r#"
        INSERT INTO repositories (
            name,
            path,
            git_dir_path,
            repo_type,
            remote_url,
            default_branch,
            head_branch,
            created_at,
            updated_at
        )
        VALUES (
            ?1, ?2, ?3, ?4, ?5,
            ?6, ?7, ?8, ?9
        )
        ON CONFLICT(path)
        DO UPDATE SET
            name = excluded.name,
            git_dir_path = excluded.git_dir_path,
            repo_type = excluded.repo_type,
            remote_url = excluded.remote_url,
            default_branch = excluded.default_branch,
            head_branch = excluded.head_branch,
            updated_at = excluded.updated_at
        "#,
        params![
            name,
            path,
            git_dir_path,
            repo_type,
            remote_url,
            default_branch,
            head_branch,
            now,
            now
        ],
    )?;

    // Retrieve the repo_id (whether inserted or existing)
    let repo_id: i64 = conn.query_row(
        "SELECT id FROM repositories WHERE path = ?1",
        params![path],
        |row| row.get(0),
    )?;

    Ok(repo_id)
}

pub fn link_repository_to_root(conn: &Connection, root_id: i64, repo_id: i64) -> Result<()> {
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        r#"
        INSERT INTO repository_roots (root_id, repo_id, created_at)
        VALUES (?1, ?2, ?3)
        ON CONFLICT(root_id, repo_id) DO NOTHING
        "#,
        params![root_id, repo_id, now],
    )?;
    Ok(())
}

pub fn set_repository_enabled(conn: &Connection, repo_id: i64, is_enabled: bool) -> Result<bool> {
    let now = chrono::Utc::now().to_rfc3339();
    let rows = conn.execute(
        r#"
        UPDATE repositories
        SET is_enabled = ?1, updated_at = ?2
        WHERE id = ?3
        "#,
        params![is_enabled, now, repo_id],
    )?;
    Ok(rows > 0)
}

pub fn is_repository_enabled_for_sync(conn: &Connection, repo_id: i64) -> Result<bool> {
    // A repo is eligible for sync if it is enabled AND at least one of its tracked roots is enabled
    let result: Option<bool> = conn.query_row(
        r#"
        SELECT (r.is_enabled = 1 AND EXISTS (
            SELECT 1 FROM repository_roots rr
            JOIN tracked_roots tr ON tr.id = rr.root_id
            WHERE rr.repo_id = r.id AND tr.is_enabled = 1
        ))
        FROM repositories r
        WHERE r.id = ?1
        "#,
        params![repo_id],
        |row| row.get(0),
    ).ok();

    Ok(result.unwrap_or(false))
}

pub fn get_root_ids_for_repository(conn: &Connection, repo_id: i64) -> Result<Vec<i64>> {
    let mut stmt = conn.prepare(
        "SELECT root_id FROM repository_roots WHERE repo_id = ?1 ORDER BY root_id ASC",
    )?;
    let root_ids = stmt
        .query_map(params![repo_id], |row| row.get(0))?
        .filter_map(Result::ok)
        .collect();
    Ok(root_ids)
}

pub fn update_repository_sync_state(
    conn: &Connection,
    repo_id: i64,
    last_scanned_at: Option<&str>,
    last_indexed_at: Option<&str>,
    index_status: Option<&str>,
) -> Result<()> {
    conn.execute(
        r#"
        UPDATE repositories
        SET
            last_scanned_at = ?1,
            last_indexed_at = COALESCE(?2, last_indexed_at),
            index_status = ?3,
            updated_at = ?4
        WHERE id = ?5
        "#,
        params![
            last_scanned_at,
            last_indexed_at,
            index_status,
            chrono::Utc::now().to_rfc3339(),
            repo_id
        ],
    )?;

    Ok(())
}

fn row_to_repository_summary(row: &rusqlite::Row) -> Result<RepositorySummary> {
    Ok(RepositorySummary {
        id: row.get(0)?,
        name: row.get(1)?,
        path: row.get(2)?,
        git_dir_path: row.get(3)?,
        repo_type: row.get(4)?,
        is_enabled: row.get(5)?,
        health_score: row.get(6)?,
        default_branch: row.get(7)?,
        head_branch: row.get(8)?,
        remote_url: row.get(9)?,
        is_dirty: row.get(10)?,
        last_commit_hash: row.get(11)?,
        last_commit_at: row.get(12)?,
        last_scanned_at: row.get(13)?,
        last_indexed_at: row.get(14)?,
        index_status: row.get(15)?,
        created_at: row.get(16)?,
        updated_at: row.get(17)?,
        total_commits: row.get(18)?,
        weekly_commits: row.get(19)?,
        unique_contributors: row.get(20)?,
        root_ids: Vec::new(),
    })
}

pub fn get_repository_by_id(conn: &Connection, id: i64) -> Result<Option<RepositorySummary>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, path, git_dir_path, repo_type, is_enabled, health_score, default_branch, head_branch,
                remote_url, is_dirty, last_commit_hash, last_commit_at, last_scanned_at, last_indexed_at, index_status, 
                created_at, updated_at, total_commits, weekly_commits, unique_contributors
         FROM repository_summary WHERE id = ?1",
    )?;

    let mut repo = stmt.query_row(params![id], row_to_repository_summary).ok();

    if let Some(ref mut r) = repo {
        r.root_ids = get_root_ids_for_repository(conn, r.id)?;
    }

    Ok(repo)
}

pub fn get_all_repositories(
    conn: &Connection,
    count: usize,
    offset: usize,
) -> Result<Option<Vec<RepositorySummary>>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, path, git_dir_path, repo_type, is_enabled, health_score, default_branch, head_branch,
                remote_url, is_dirty, last_commit_hash, last_commit_at, last_scanned_at, last_indexed_at, index_status, 
                created_at, updated_at, total_commits, weekly_commits, unique_contributors
         FROM repository_summary ORDER BY updated_at DESC LIMIT ?1 OFFSET ?2",
    )?;

    let repos_iter = stmt.query_map([count, offset], row_to_repository_summary)?;
    let mut repos: Vec<RepositorySummary> = repos_iter.filter_map(Result::ok).collect();

    for r in &mut repos {
        r.root_ids = get_root_ids_for_repository(conn, r.id).unwrap_or_default();
    }

    Ok(Some(repos))
}

pub fn get_repositories_by_root_id(
    conn: &Connection,
    root_id: i64,
) -> Result<Vec<RepositorySummary>> {
    // When retrieving for a specific root, we use the root-specific created_at from repository_roots
    let mut stmt = conn.prepare(
        "SELECT r.id, r.name, r.path, r.git_dir_path, r.repo_type, r.is_enabled, r.health_score, r.default_branch, r.head_branch,
                r.remote_url, r.is_dirty, r.last_commit_hash, r.last_commit_at, r.last_scanned_at, r.last_indexed_at, r.index_status, 
                rr.created_at, r.updated_at, r.total_commits, r.weekly_commits, r.unique_contributors
         FROM repository_roots rr
         JOIN repository_summary r ON r.id = rr.repo_id
         WHERE rr.root_id = ?1
         ORDER BY r.name ASC",
    )?;

    let repos_iter = stmt.query_map([root_id], row_to_repository_summary)?;
    let mut repos: Vec<RepositorySummary> = repos_iter.filter_map(Result::ok).collect();

    for r in &mut repos {
        r.root_ids = vec![root_id];
    }

    Ok(repos)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PaginatedRepositories {
    pub items: Vec<RepositorySummary>,
    pub next_cursor: Option<i64>,
    pub has_more: bool,
    pub total_count: usize,
}

pub fn get_paginated_repositories(
    conn: &Connection,
    search: Option<&str>,
    filter: Option<&str>,
    limit: usize,
    cursor: Option<i64>,
) -> Result<PaginatedRepositories> {
    let mut where_clauses: Vec<String> = Vec::new();
    let mut param_values: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(query) = search.filter(|s| !s.trim().is_empty()) {
        let pattern = format!("%{}%", query.trim());
        param_values.push(Box::new(pattern));
        let idx = param_values.len();
        where_clauses.push(format!("(name LIKE ?{idx} OR path LIKE ?{idx})"));
    }

    if let Some(f) = filter {
        match f.to_lowercase().as_str() {
            "clean" => where_clauses.push("is_dirty = 0".to_string()),
            "modified" => where_clauses.push("is_dirty = 1".to_string()),
            "unhealthy" => where_clauses.push("health_score < 0.7".to_string()),
            "enabled" => where_clauses.push("is_enabled = 1".to_string()),
            "disabled" => where_clauses.push("is_enabled = 0".to_string()),
            _ => {}
        }
    }

    // Count total matches
    let where_sql = if where_clauses.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", where_clauses.join(" AND "))
    };

    let count_query = format!("SELECT COUNT(*) FROM repository_summary {where_sql}");
    let total_count: usize = {
        let params_slice: Vec<&dyn rusqlite::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();
        conn.query_row(&count_query, rusqlite::params_from_iter(params_slice), |row| row.get(0))?
    };

    // Cursor clause for pagination (using id < cursor if ordered by id DESC)
    if let Some(c) = cursor {
        param_values.push(Box::new(c));
        let idx = param_values.len();
        where_clauses.push(format!("id < ?{idx}"));
    }

    let paginated_where_sql = if where_clauses.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", where_clauses.join(" AND "))
    };

    param_values.push(Box::new((limit + 1) as i64));
    let limit_idx = param_values.len();

    let sql = format!(
        "SELECT id, name, path, git_dir_path, repo_type, is_enabled, health_score, default_branch, head_branch,
                remote_url, is_dirty, last_commit_hash, last_commit_at, last_scanned_at, last_indexed_at, index_status, 
                created_at, updated_at, total_commits, weekly_commits, unique_contributors
         FROM repository_summary
         {paginated_where_sql}
         ORDER BY id DESC
         LIMIT ?{limit_idx}"
    );

    let params_slice: Vec<&dyn rusqlite::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();
    let mut stmt = conn.prepare(&sql)?;
    let repos_iter = stmt.query_map(rusqlite::params_from_iter(params_slice), row_to_repository_summary)?;
    let mut items: Vec<RepositorySummary> = repos_iter.filter_map(Result::ok).collect();

    let has_more = items.len() > limit;
    if has_more {
        items.truncate(limit);
    }

    let next_cursor = if has_more {
        items.last().map(|r| r.id)
    } else {
        None
    };

    for r in &mut items {
        r.root_ids = get_root_ids_for_repository(conn, r.id).unwrap_or_default();
    }

    Ok(PaginatedRepositories {
        items,
        next_cursor,
        has_more,
        total_count,
    })
}

pub fn get_repository_metrics(
    conn: &Connection,
    repo_id: i64,
    week_ago: &str,
) -> Result<RepositoryMetrics> {
    conn.query_row(
        r#"
        SELECT
            (SELECT COUNT(*) FROM commits WHERE repo_id = ?1),
            (SELECT COUNT(*) FROM commits WHERE repo_id = ?1 AND committed_at >= ?2),
            (SELECT MAX(committed_at) FROM commits WHERE repo_id = ?1),
            (SELECT COUNT(*) FROM contributors WHERE repo_id = ?1)
        "#,
        params![repo_id, week_ago],
        |row| {
            Ok(RepositoryMetrics {
                total_commits: row.get(0)?,
                weekly_commits: row.get(1)?,
                last_commit_at: row.get(2)?,
                unique_contributors: row.get(3)?,
            })
        },
    )
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

pub fn get_repo_activity(
    conn: &Connection,
    repo_id: i64,
    start_date: Option<&str>,
    end_date: Option<&str>,
    limit: Option<i32>,
) -> Result<Vec<crate::infrastructure::database::models::RepoActivityDaily>> {
    let mut sql =
        "SELECT id, repo_id, activity_date, commit_count, additions, deletions, files_changed 
                   FROM repo_activity_daily 
                   WHERE repo_id = ?1"
            .to_string();

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
        Ok(crate::infrastructure::database::models::RepoActivityDaily {
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
                   WHERE repo_id = ?1"
        .to_string();

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

pub fn get_repository_path(conn: &Connection, repo_id: i64) -> Result<Option<String>> {
    let mut stmt = conn.prepare("SELECT path FROM repositories WHERE id = ?1 LIMIT 1")?;
    let path: Option<String> = stmt.query_row([repo_id], |row| row.get(0))?;
    Ok(path)
}
