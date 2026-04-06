# GitRadar API Contract

GitRadar uses Tauri commands as the communication layer between the frontend (React + TanStack) and the backend (Rust). The frontend invokes commands through a thin client wrapper, and the backend returns structured JSON-compatible responses.

All commands must follow consistent rules:
- Inputs must be well-defined JSON objects
- Outputs must be typed and predictable
- Errors must follow a consistent structure
- Internal Rust logic must not leak into the API surface

---

## Error Format

All errors returned from backend should follow this shape:

{
  "code": "ERROR_CODE",
  "message": "Human readable message",
  "details": null
}

---

## Commands

### select_tracked_root

Registers a root directory for scanning.

Input:
{
  "path": "/home/user/projects"
}

Output:
{
  "id": 1,
  "path": "/home/user/projects",
  "isEnabled": true
}

---

### get_tracked_roots

Returns all tracked root directories.

Input:
{}

Output:
[
  {
    "id": 1,
    "path": "/home/user/projects",
    "isEnabled": true
  }
]

---

### scan_repositories

Scans a tracked root and discovers Git repositories.

Input:
{
  "rootId": 1
}

Output:
{
  "discoveredCount": 12,
  "repositories": [
    {
      "id": 5,
      "name": "gitradar",
      "path": "/home/user/projects/gitradar",
      "defaultBranch": "main",
      "headBranch": "feature/ui",
      "isEnabled": true
    }
  ]
}

---

### get_repositories

Returns tracked repositories with optional filtering.

Input:
{
  "rootId": 1,
  "search": "",
  "includeDisabled": false
}

Output:
[
  {
    "id": 5,
    "name": "gitradar",
    "path": "/home/user/projects/gitradar",
    "defaultBranch": "main",
    "headBranch": "feature/ui",
    "lastScannedAt": "2026-04-07T10:00:00Z",
    "isEnabled": true
  }
]

---

### get_repository_details

Returns detailed information about a repository.

Input:
{
  "repoId": 5
}

Output:
{
  "id": 5,
  "name": "gitradar",
  "path": "/home/user/projects/gitradar",
  "defaultBranch": "main",
  "headBranch": "feature/ui",
  "workingTree": {
    "modifiedCount": 2,
    "stagedCount": 1,
    "untrackedCount": 3,
    "deletedCount": 0
  },
  "lastActivityAt": "2026-04-07T10:10:00Z",
  "healthScore": 82.5
}

---

### get_repository_commits

Returns paginated commit history.

Input:
{
  "repoId": 5,
  "limit": 50,
  "offset": 0
}

Output:
{
  "items": [
    {
      "hash": "abc123",
      "authorName": "User",
      "authorEmail": "user@example.com",
      "messageSubject": "Add initial dashboard",
      "committedAt": "2026-04-07T08:20:00Z",
      "parentCount": 1
    }
  ],
  "total": 314
}

---

### get_commit_activity

Returns commit counts grouped by time.

Input:
{
  "repoId": 5,
  "groupBy": "day",
  "rangeDays": 30
}

Output:
[
  {
    "bucket": "2026-04-01",
    "count": 4
  },
  {
    "bucket": "2026-04-02",
    "count": 2
  }
]

---

### get_file_hotspots

Returns top hotspot files.

Input:
{
  "repoId": 5,
  "limit": 20
}

Output:
[
  {
    "filePath": "src/app/router/routes/dashboard.tsx",
    "touchCount": 14,
    "churnScore": 220,
    "hotspotScore": 58.2,
    "lastTouchedAt": "2026-04-07T08:20:00Z"
  }
]

---

### get_contributor_stats

Returns contributor analytics.

Input:
{
  "repoId": 5
}

Output:
[
  {
    "authorName": "User",
    "authorEmail": "user@example.com",
    "commitCount": 140,
    "churn": 4200,
    "activeDays": 32
  }
]

---

### refresh_repository

Triggers re-indexing for a repository.

Input:
{
  "repoId": 5
}

Output:
{
  "success": true,
  "refreshedAt": "2026-04-07T10:30:00Z"
}

---

### get_settings

Returns application settings.

Input:
{}

Output:
{
  "theme": "dark",
  "watcherEnabled": true,
  "scanIntervalSeconds": 10
}

---

### update_settings

Updates application settings.

Input:
{
  "theme": "light",
  "watcherEnabled": true,
  "scanIntervalSeconds": 15
}

Output:
{
  "success": true
}

---

## Notes

- Frontend should call commands through a centralized client (e.g., `tauriClient`)
- Types should be mirrored in both Rust and TypeScript
- Pagination must be used for large datasets
- Avoid exposing internal database or filesystem structure directly
- Keep API stable even if backend implementation evolves