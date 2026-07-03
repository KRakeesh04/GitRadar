# GitRadar Requirements Specification

Version: 2.0
Status: Approved for Architecture Update

---

# 1. Product Vision

GitRadar is a Linux-first desktop application that provides local Git repository discovery, indexing, analytics, commit visualization, file exploration, and repository insights.

The application is completely local-first and does not require internet access for core functionality.

The primary goal is to provide a fast and secure Git intelligence dashboard for developers managing multiple local repositories.

---

# 2. Functional Requirements

## FR-001 Repository Root Management

The user shall be able to:

- Add one or more root folders.
- Edit tracked root folders.
- Remove tracked root folders.
- Enable or disable tracked root folders.
- Trigger manual rescans.

Examples:

Valid:

/home/user/projects
/home/user/work
/home/user/freelance

---

## FR-002 Parent Folder Tracking

When a parent folder is added:

Example:

/home/user/projects

The system shall automatically discover all Git repositories inside:

/home/user/projects/app1
/home/user/projects/app2
/home/user/projects/client/project-x

No additional configuration is required.

---

## FR-003 Multiple Root Support

Users may register multiple unrelated root folders.

Example:

/projects
/work
/personal
/client-projects

All roots shall be indexed independently.

---

## FR-004 Repository Discovery

The system shall:

- Discover repositories recursively.
- Detect valid Git repositories.
- Ignore invalid .git directories.
- Detect deleted repositories.
- Detect moved repositories.
- Handle symbolic links safely.

---

## FR-005 Repository Metadata

Each repository shall expose:

- Name
- Path
- Current branch
- Default branch
- Last activity time
- Last indexed time
- Working tree status
- Repository size
- Total commits

---

## FR-006 Repository Details View

Repository detail pages shall display:

### Repository Overview

- Name
- Path
- Current branch
- Last commit
- Last activity
- Health score

### Commit History

- Commit graph/tree
- Branch relationships
- Merge commits
- Commit details

### File Explorer

- Repository file tree
- File preview
- File metadata

### Working Tree

- Modified files
- Staged files
- Deleted files
- Untracked files

---

## FR-007 Commit Visualization

The system shall display:

- Commit tree graph
- Branch graph
- Merge relationships
- Commit timeline

Similar to:

- GitKraken
- SourceTree
- GitHub Network Graph

---

## FR-008 File Diff Viewer

Users shall be able to view:

- Current changes
- Commit-to-commit differences
- Branch comparison differences

Display:

- Added lines
- Removed lines
- Modified blocks

Support:

- Side-by-side mode
- Unified mode

---

## FR-009 Analytics Dashboard

Dashboard shall display:

### Global Metrics

- Total repositories
- Active repositories
- Total commits
- Recent activity

### Repository Metrics

- Commit frequency
- Hotspot files
- Churn score
- Health score
- Contributor metrics

---

## FR-010 Search

Search shall support:

- Repository name
- File path
- Branch name
- Commit message
- Commit hash

---

## FR-011 Background Synchronization

Synchronization shall run only when:

- Application is running.

Synchronization shall stop when:

- Application closes.

No background daemon shall exist in MVP.

---

## FR-012 Incremental Indexing

The system shall:

- Index only changed repositories.
- Avoid full rescans whenever possible.
- Recompute analytics incrementally.

---

## FR-013 Folder Configuration

Users shall be able to modify:

- Root paths
- Scan settings
- Refresh intervals
- Analytics preferences

Without restarting the application.

---

# 3. Performance Requirements

## PR-001 Startup

Application startup:

Target:
< 2 seconds

---

## PR-002 Repository List

Repository list rendering:

Target:
< 200ms

---

## PR-003 Repository Detail Page

Target:
< 500ms

---

## PR-004 Analytics

Dashboard refresh:

Target:
< 1 second

---

## PR-005 Database

SQLite queries shall use:

- Indexed columns
- Prepared statements
- Pagination

---

# 4. Security Requirements

## SR-001 Local First

All repository data remains local.

No outbound communication by default.

---

## SR-002 Repository Isolation

Only approved root folders may be scanned.

The application shall never scan arbitrary directories.

---

## SR-003 Secure Command Execution

Future terminal features:

- Pull
- Push
- Fetch
- Merge

Must be executed through a restricted command layer.

Direct shell execution is prohibited.

---

## SR-004 Secret Protection

The application shall never:

- Read .env values for analytics.
- Upload repository contents.
- Store credentials.

---

## SR-005 Database Security

SQLite shall use:

- WAL mode
- Foreign keys enabled
- Transaction protection

---

## SR-006 File Access Control

Only repository metadata required for analytics shall be indexed.

Large binary files shall be excluded.

---

## SR-007 Audit Logging

Security-sensitive actions shall be logged:

- Root addition
- Root removal
- Settings changes
- Future Git operations

---

# 5. Future Features

## WakaTime Integration

Display:

- Coding hours
- Daily activity
- Weekly activity
- Project duration analytics

---

## Git Operations Terminal

Restricted terminal panel:

Allowed:

- git pull
- git push
- git fetch
- git merge
- git checkout

Blocked:

- Arbitrary shell execution

---

## Export System

Support:

- JSON
- CSV
- PDF

---

## Remote Repository Insights

Future support:

- GitHub
- GitLab
- Gitea

---

# 6. MVP Scope

Included:

✓ Multi-root tracking

✓ Repository discovery

✓ Commit indexing

✓ Commit graph

✓ File explorer

✓ Diff viewer

✓ Dashboard analytics

✓ SQLite persistence

✓ Background sync while app is open

✓ Security controls

Excluded:

✗ Internet connectivity

✗ WakaTime integration

✗ GitHub integration

✗ Push/Pull terminal

✗ Multi-device synchronization
