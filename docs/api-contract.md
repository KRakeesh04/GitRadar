# GitRadar API Contract

Version: 2.0

---

# API Principles

All Tauri commands must:

* Return typed responses
* Never expose internal database structure
* Return predictable errors
* Support pagination
* Support cancellation

---

# Standard Response

## Success

```json
{
  "success": true,
  "data": {}
}
```

## Error

```json
{
  "success": false,
  "error": {
    "code": "ERROR_CODE",
    "message": "Description"
  }
}
```

---

# Root Folder Commands

## add_root

Input

```json
{
  "path": "/home/user/projects",
  "name": "Projects"
}
```

Output

```json
{
  "id": 1
}
```

---

## update_root

Input

```json
{
  "rootId": 1,
  "name": "Work Projects",
  "isEnabled": true
}
```

---

## delete_root

Input

```json
{
  "rootId": 1
}
```

---

## get_roots

Output

```json
[
  {
    "id": 1,
    "name": "Projects",
    "path": "/home/user/projects",
    "repositoryCount": 22
  }
]
```

---

# Repository Commands

## scan_root

Manual discovery.

```json
{
  "rootId": 1
}
```

---

## get_repositories

Supports:

* Search
* Pagination
* Filtering

```json
{
  "page": 1,
  "pageSize": 50
}
```

---

## get_repository

Returns:

* Metadata
* Statistics
* Activity

```json
{
  "repoId": 12
}
```

---

## refresh_repository

Forces re-index.

---

# Commit Graph Commands

## get_commit_graph

Required for commit tree UI.

Input

```json
{
  "repoId": 12
}
```

Output

```json
{
  "nodes": [
    {
      "hash": "abc123",
      "branch": "main",
      "parents": []
    }
  ]
}
```

---

## get_commit_details

Input

```json
{
  "repoId": 12,
  "commitHash": "abc123"
}
```

Output

```json
{
  "hash": "abc123",
  "message": "Initial commit",
  "author": "John",
  "filesChanged": 12
}
```

---

# File Explorer Commands

## get_repository_tree

Returns directory structure.

Input

```json
{
  "repoId": 12
}
```

Output

```json
{
  "children": []
}
```

---

## get_file_content

Input

```json
{
  "repoId": 12,
  "path": "src/main.rs"
}
```

Output

```json
{
  "content": "...",
  "isBinary": false
}
```

---

# Diff Commands

## get_working_tree_diff

Input

```json
{
  "repoId": 12
}
```

---

## get_commit_diff

Input

```json
{
  "repoId": 12,
  "commitHash": "abc123"
}
```

---

## compare_commits

Input

```json
{
  "repoId": 12,
  "fromCommit": "abc",
  "toCommit": "xyz"
}
```

---

## compare_branches

Input

```json
{
  "repoId": 12,
  "sourceBranch": "main",
  "targetBranch": "feature/auth"
}
```

---

# Analytics Commands

## get_dashboard_metrics

Returns:

* Total repositories
* Active repositories
* Commits
* Activity

---

## get_repository_analytics

Returns:

* Hotspots
* Churn
* Health
* Contributors

---

## get_hotspot_files

Returns ranked hotspot files.

---

## get_contributors

Returns contributor statistics.

---

# Search Commands

## search

Global search.

Input

```json
{
  "query": "auth",
  "limit": 50
}
```

Searches:

* Repositories
* Files
* Branches
* Commits

---

# Sync Commands

## get_sync_status

Output

```json
{
  "running": true,
  "queuedJobs": 3,
  "activeJobs": 1
}
```

---

## pause_sync

---

## resume_sync

---

# Settings Commands

## get_settings

---

## update_settings

Supports:

* Theme
* Refresh interval
* Analytics preferences

---

# Audit Commands

## get_audit_logs

Input

```json
{
  "page": 1,
  "pageSize": 100
}
```

---

# Future WakaTime Commands

## connect_wakatime

Input

```json
{
  "apiKey": "encrypted"
}
```

---

## sync_wakatime

Manual sync.

---

## get_wakatime_stats

Returns:

* Daily time
* Weekly time
* Monthly time
* Languages

---

# Future Git Operations

## git_fetch

## git_pull

## git_push

## git_merge

## git_checkout

All commands must pass through Git Sandbox.

No arbitrary shell execution.

---

# Pagination Standard

All paginated endpoints:

```json
{
  "items": [],
  "total": 1200,
  "page": 1,
  "pageSize": 50
}
```

---

# Security Rules

Commands must:

* Validate repository ownership
* Validate root permissions
* Prevent path traversal
* Prevent symlink escapes
* Sanitize all user input
* Log security-sensitive operations
