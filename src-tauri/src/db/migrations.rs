use rusqlite::{Connection, Result};

pub fn run_migrations(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        PRAGMA foreign_keys = ON;

        CREATE TABLE IF NOT EXISTS tracked_roots (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            path            TEXT NOT NULL UNIQUE,
            is_enabled      INTEGER NOT NULL DEFAULT 1,
            created_at      TEXT NOT NULL,
            updated_at      TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS repositories (
            id                  INTEGER PRIMARY KEY AUTOINCREMENT,
            root_id             INTEGER NOT NULL,
            name                TEXT NOT NULL,
            path                TEXT NOT NULL UNIQUE,
            git_dir_path        TEXT NOT NULL,
            default_branch      TEXT,
            head_branch         TEXT,
            is_dirty            INTEGER NOT NULL DEFAULT 0,
            last_commit_hash    TEXT,
            last_commit_at      TEXT,
            last_scanned_at     TEXT,
            last_indexed_at     TEXT,
            index_status        TEXT,
            created_at          TEXT NOT NULL,
            updated_at          TEXT NOT NULL,
            FOREIGN KEY (root_id) REFERENCES tracked_roots(id) ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_repositories_root_id
            ON repositories(root_id);

        CREATE INDEX IF NOT EXISTS idx_repositories_path
            ON repositories(path);

        CREATE TABLE IF NOT EXISTS branches (
            id                  INTEGER PRIMARY KEY AUTOINCREMENT,
            repo_id             INTEGER NOT NULL,
            name                TEXT NOT NULL,
            is_head             INTEGER NOT NULL DEFAULT 0,
            is_default          INTEGER NOT NULL DEFAULT 0,
            last_commit_hash    TEXT,
            last_commit_at      TEXT,
            ahead_count         INTEGER NOT NULL DEFAULT 0,
            behind_count        INTEGER NOT NULL DEFAULT 0,
            updated_at          TEXT NOT NULL,
            FOREIGN KEY (repo_id) REFERENCES repositories(id) ON DELETE CASCADE,
            UNIQUE(repo_id, name)
        );

        CREATE INDEX IF NOT EXISTS idx_branches_repo_id
            ON branches(repo_id);

        CREATE TABLE IF NOT EXISTS commits (
            id                  INTEGER PRIMARY KEY AUTOINCREMENT,
            repo_id             INTEGER NOT NULL,
            hash                TEXT NOT NULL,
            author_name         TEXT,
            author_email        TEXT,
            committer_name      TEXT,
            committer_email     TEXT,
            subject             TEXT NOT NULL,
            body                TEXT,
            parent_count        INTEGER NOT NULL DEFAULT 0,
            committed_at        TEXT NOT NULL,
            inserted_at         TEXT NOT NULL,
            FOREIGN KEY (repo_id) REFERENCES repositories(id) ON DELETE CASCADE,
            UNIQUE(repo_id, hash)
        );

        CREATE INDEX IF NOT EXISTS idx_commits_repo_id
            ON commits(repo_id);

        CREATE INDEX IF NOT EXISTS idx_commits_repo_committed_at
            ON commits(repo_id, committed_at);

        CREATE TABLE IF NOT EXISTS commit_file_stats (
            id                  INTEGER PRIMARY KEY AUTOINCREMENT,
            repo_id             INTEGER NOT NULL,
            commit_hash         TEXT NOT NULL,
            file_path           TEXT NOT NULL,
            change_type         TEXT NOT NULL,
            additions           INTEGER NOT NULL DEFAULT 0,
            deletions           INTEGER NOT NULL DEFAULT 0,
            total_changes       INTEGER NOT NULL DEFAULT 0,
            FOREIGN KEY (repo_id) REFERENCES repositories(id) ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_commit_file_stats_repo_id
            ON commit_file_stats(repo_id);

        CREATE INDEX IF NOT EXISTS idx_commit_file_stats_repo_file
            ON commit_file_stats(repo_id, file_path);

        CREATE TABLE IF NOT EXISTS commit_branches (
            id                  INTEGER PRIMARY KEY AUTOINCREMENT,
            commit_id           INTEGER NOT NULL,
            branch_id           INTEGER NOT NULL,
            FOREIGN KEY (commit_id) REFERENCES commits(id) ON DELETE CASCADE,
            FOREIGN KEY (branch_id) REFERENCES branches(id) ON DELETE CASCADE,
            UNIQUE(commit_id, branch_id)
        );

        CREATE INDEX IF NOT EXISTS idx_commit_branches_commit_id
            ON commit_branches(commit_id);

        CREATE INDEX IF NOT EXISTS idx_commit_branches_branch_id
            ON commit_branches(branch_id);

        CREATE TABLE IF NOT EXISTS working_tree_status (
            id                  INTEGER PRIMARY KEY AUTOINCREMENT,
            repo_id             INTEGER NOT NULL,
            modified_count      INTEGER NOT NULL DEFAULT 0,
            staged_count        INTEGER NOT NULL DEFAULT 0,
            untracked_count     INTEGER NOT NULL DEFAULT 0,
            deleted_count       INTEGER NOT NULL DEFAULT 0,
            renamed_count       INTEGER NOT NULL DEFAULT 0,
            captured_at         TEXT NOT NULL,
            FOREIGN KEY (repo_id) REFERENCES repositories(id) ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_working_tree_status_repo_id
            ON working_tree_status(repo_id);

        CREATE TABLE IF NOT EXISTS repo_activity_daily (
            id                  INTEGER PRIMARY KEY AUTOINCREMENT,
            repo_id             INTEGER NOT NULL,
            activity_date       TEXT NOT NULL,
            commit_count        INTEGER NOT NULL DEFAULT 0,
            additions           INTEGER NOT NULL DEFAULT 0,
            deletions           INTEGER NOT NULL DEFAULT 0,
            files_changed       INTEGER NOT NULL DEFAULT 0,
            FOREIGN KEY (repo_id) REFERENCES repositories(id) ON DELETE CASCADE,
            UNIQUE(repo_id, activity_date)
        );

        CREATE TABLE IF NOT EXISTS file_hotspots (
            id                  INTEGER PRIMARY KEY AUTOINCREMENT,
            repo_id             INTEGER NOT NULL,
            file_path           TEXT NOT NULL,
            touch_count         INTEGER NOT NULL DEFAULT 0,
            churn_score         REAL NOT NULL DEFAULT 0,
            hotspot_score       REAL NOT NULL DEFAULT 0,
            last_touched_at     TEXT,
            updated_at          TEXT NOT NULL,
            FOREIGN KEY (repo_id) REFERENCES repositories(id) ON DELETE CASCADE,
            UNIQUE(repo_id, file_path)
        );

        CREATE TABLE IF NOT EXISTS contributors (
            id                  INTEGER PRIMARY KEY AUTOINCREMENT,
            repo_id             INTEGER NOT NULL,
            author_name         TEXT NOT NULL,
            author_email        TEXT,
            commit_count        INTEGER NOT NULL DEFAULT 0,
            additions           INTEGER NOT NULL DEFAULT 0,
            deletions           INTEGER NOT NULL DEFAULT 0,
            active_days         INTEGER NOT NULL DEFAULT 0,
            last_commit_at      TEXT,
            updated_at          TEXT NOT NULL,
            FOREIGN KEY (repo_id) REFERENCES repositories(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS snapshots (
            id                  INTEGER PRIMARY KEY AUTOINCREMENT,
            repo_id             INTEGER NOT NULL,
            snapshot_type       TEXT CHECK(snapshot_type IN (
                                                    'dashboard_summary', 
                                                    'repo_overview', 
                                                    'activity_chart', 
                                                    'top_hotspots', 
                                                    'contributors_summary')),
            snapshot_key        TEXT NOT NULL,
            data_json           TEXT NOT NULL,
            created_at          TEXT NOT NULL,
            FOREIGN KEY (repo_id) REFERENCES repositories(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS settings (
            key             TEXT PRIMARY KEY,
            value           TEXT NOT NULL,
            updated_at      TEXT NOT NULL
        );
        "#,
    )?;

    Ok(())
}