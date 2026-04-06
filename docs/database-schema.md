# GitRadar Database Schema

## Overview

GitRadar uses SQLite as its local storage engine. The database stores repository metadata, indexed Git history, working tree summaries, and computed analytics.

The schema is designed to support:
- fast local reads
- incremental indexing
- analytics caching
- safe startup recovery

---

## Design Principles

- Keep raw repository identity separate from computed analytics
- Store stable Git facts once
- Store derived metrics in dedicated tables
- Allow re-computation of analytics without losing source data
- Use timestamps for refresh and staleness tracking

---

## Core Tables

## `tracked_roots`
Stores user-approved top-level folders that GitRadar is allowed to scan.

Columns:
- `id` INTEGER PRIMARY KEY
- `path` TEXT NOT NULL UNIQUE
- `is_enabled` INTEGER NOT NULL DEFAULT 1
- `created_at` TEXT NOT NULL
- `updated_at` TEXT NOT NULL

---

## `tracked_repositories`
Stores discovered Git repositories under tracked roots.

Columns:
- `id` INTEGER PRIMARY KEY
- `root_id` INTEGER NOT NULL
- `name` TEXT NOT NULL
- `path` TEXT NOT NULL UNIQUE
- `git_dir_path` TEXT NOT NULL
- `default_branch` TEXT
- `head_branch` TEXT
- `last_scanned_at` TEXT
- `last_indexed_commit_hash` TEXT
- `is_enabled` INTEGER NOT NULL DEFAULT 1
- `created_at` TEXT NOT NULL
- `updated_at` TEXT NOT NULL

Foreign keys:
- `root_id` -> `tracked_roots.id`

Indexes:
- index on `root_id`
- index on `path`

---

## `branches`
Stores branches for a repository.

Columns:
- `id` INTEGER PRIMARY KEY
- `repo_id` INTEGER NOT NULL
- `name` TEXT NOT NULL
- `is_head` INTEGER NOT NULL DEFAULT 0
- `last_commit_hash` TEXT
- `last_commit_at` TEXT
- `created_at` TEXT NOT NULL
- `updated_at` TEXT NOT NULL

Foreign keys:
- `repo_id` -> `tracked_repositories.id`

Indexes:
- composite index on `(repo_id, name)`

---

## `commits`
Stores commit metadata.

Columns:
- `id` INTEGER PRIMARY KEY
- `repo_id` INTEGER NOT NULL
- `hash` TEXT NOT NULL
- `author_name` TEXT
- `author_email` TEXT
- `committer_name` TEXT
- `committer_email` TEXT
- `message_subject` TEXT NOT NULL
- `message_body` TEXT
- `parent_count` INTEGER NOT NULL DEFAULT 0
- `committed_at` TEXT NOT NULL
- `inserted_at` TEXT NOT NULL

Foreign keys:
- `repo_id` -> `tracked_repositories.id`

Constraints:
- unique `(repo_id, hash)`

Indexes:
- index on `(repo_id, committed_at)`
- index on `(repo_id, author_email)`

---

## `commit_file_stats`
Stores file-level change summaries for each commit.

Columns:
- `id` INTEGER PRIMARY KEY
- `repo_id` INTEGER NOT NULL
- `commit_hash` TEXT NOT NULL
- `file_path` TEXT NOT NULL
- `change_type` TEXT NOT NULL
- `additions` INTEGER NOT NULL DEFAULT 0
- `deletions` INTEGER NOT NULL DEFAULT 0
- `total_changes` INTEGER NOT NULL DEFAULT 0

Foreign keys:
- `repo_id` -> `tracked_repositories.id`

Indexes:
- index on `(repo_id, file_path)`
- index on `(repo_id, commit_hash)`

---

## `working_tree_snapshots`
Stores summary of current uncommitted state over time.

Columns:
- `id` INTEGER PRIMARY KEY
- `repo_id` INTEGER NOT NULL
- `captured_at` TEXT NOT NULL
- `modified_count` INTEGER NOT NULL DEFAULT 0
- `staged_count` INTEGER NOT NULL DEFAULT 0
- `untracked_count` INTEGER NOT NULL DEFAULT 0
- `deleted_count` INTEGER NOT NULL DEFAULT 0

Foreign keys:
- `repo_id` -> `tracked_repositories.id`

Indexes:
- index on `(repo_id, captured_at)`

---

## `file_hotspots`
Stores derived hotspot metrics for files.

Columns:
- `id` INTEGER PRIMARY KEY
- `repo_id` INTEGER NOT NULL
- `file_path` TEXT NOT NULL
- `touch_count` INTEGER NOT NULL DEFAULT 0
- `churn_score` REAL NOT NULL DEFAULT 0
- `last_touched_at` TEXT
- `updated_at` TEXT NOT NULL

Foreign keys:
- `repo_id` -> `tracked_repositories.id`

Constraints:
- unique `(repo_id, file_path)`

Indexes:
- index on `(repo_id, churn_score DESC)`

---

## `repo_health_metrics`
Stores derived repository-level health indicators.

Columns:
- `id` INTEGER PRIMARY KEY
- `repo_id` INTEGER NOT NULL
- `stale_branch_count` INTEGER NOT NULL DEFAULT 0
- `hotspot_file_count` INTEGER NOT NULL DEFAULT 0
- `large_commit_count` INTEGER NOT NULL DEFAULT 0
- `avg_commit_size` REAL NOT NULL DEFAULT 0
- `health_score` REAL NOT NULL DEFAULT 0
- `computed_at` TEXT NOT NULL

Foreign keys:
- `repo_id` -> `tracked_repositories.id`

Constraints:
- unique `repo_id`

---

## `settings`
Stores app-level configuration.

Columns:
- `key` TEXT PRIMARY KEY
- `value` TEXT NOT NULL
- `updated_at` TEXT NOT NULL

Examples:
- `theme`
- `scan_interval_seconds`
- `watcher_enabled`
- `analytics_refresh_mode`

---

## Suggested Migration Order

1. `tracked_roots`
2. `tracked_repositories`
3. `branches`
4. `commits`
5. `commit_file_stats`
6. `working_tree_snapshots`
7. `file_hotspots`
8. `repo_health_metrics`
9. `settings`

---

## Notes

- Timestamps should be stored in ISO 8601 UTC strings
- SQLite foreign keys should be explicitly enabled
- Derived tables can be recomputed if analytics logic changes
- Commit file stats should store summary data only for MVP, not full patches