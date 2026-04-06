# GitRadar Architecture

## Overview

GitRadar is a Linux-first desktop application for discovering, tracking, and analyzing local Git repositories. It provides GitHub-style insights for repositories stored on the user's machine, while keeping all data local.

The system is built with:

- **Frontend:** React + TypeScript + TanStack + Vite
- **Desktop shell / backend bridge:** Tauri
- **Core backend:** Rust
- **Storage:** SQLite

---

## High-Level Architecture

GitRadar is organized into four main layers:

1. **Presentation Layer**
   - React UI
   - Route-based pages
   - Charts and analytics views
   - Search, filtering, and navigation

2. **Application Layer**
   - Tauri commands
   - Frontend-backend communication
   - Query orchestration
   - State coordination

3. **Core Domain Layer**
   - Git repository discovery
   - Git history parsing
   - File status inspection
   - Analytics computation
   - Watcher scheduling

4. **Persistence Layer**
   - SQLite storage
   - Repository metadata
   - Commit/file statistics
   - Cached analytics
   - App settings

---

## Main Components

### Frontend
Responsible for:
- Rendering dashboards
- Showing repository and file insights
- Navigating between pages
- Triggering backend commands
- Presenting analytics graphs

Key frontend modules:
- `app/router`
- `features/dashboard`
- `features/repositories`
- `features/repository-details`
- `features/commits`
- `features/files`
- `features/analytics`
- `features/settings`

### Tauri Command Layer
Acts as the interface between UI and Rust backend logic.

Responsibilities:
- Expose callable commands to the frontend
- Validate input
- Return serialized results
- Map errors into UI-safe responses

Example command groups:
- `repos`
- `commits`
- `files`
- `analytics`
- `settings`

### Core Services
Implements the main business logic.

Core responsibilities:
- Discover repositories under approved directories
- Read Git metadata and current working tree state
- Schedule indexing jobs
- Handle filesystem change events
- Compute hotspot and churn metrics

Important services:
- `repo_discovery`
- `git_service`
- `diff_service`
- `watcher`
- `scheduler`
- `permissions`

### Database Layer
Stores local indexed state so the app does not need to fully rescan every time.

Stores:
- tracked roots
- tracked repositories
- branches
- commits
- file-level change summaries
- working tree snapshots
- computed analytics
- user settings

---

## Data Flow

## 1. Repository Discovery
1. User selects a root directory
2. Backend recursively scans allowed paths
3. Directories containing `.git` are identified
4. Repositories are registered in the database
5. Initial indexing jobs are created

## 2. Initial Indexing
1. Git history is read for a repository
2. Commit metadata is extracted
3. File-level diff stats are computed
4. Aggregated analytics are generated
5. Results are saved into SQLite

## 3. Incremental Updates
1. Filesystem watcher detects repo-related changes
2. Scheduler debounces repeated events
3. Only affected repositories are refreshed
4. Only new commits or changed working tree state are re-indexed
5. Analytics are partially recomputed

## 4. UI Rendering
1. Frontend queries backend through Tauri commands
2. Data is cached with TanStack Query
3. Charts and repository views are rendered
4. User can filter, search, and inspect analytics

---

## Architectural Principles

### Local-first
All analytics and indexing happen locally. No external service is required.

### Incremental over full rescans
GitRadar should avoid recomputing everything whenever possible.

### Feature isolation
Frontend and backend modules should be grouped by responsibility.

### Read-heavy optimization
The app will read analytics often, so cached summaries and indexed data should be designed for fast retrieval.

### Safe filesystem interaction
The app should handle:
- missing folders
- permission errors
- symlinks
- deleted repositories
- invalid Git directories

---

## Non-Goals for MVP

The first version will not focus on:
- remote Git hosting integration
- cloud sync
- multi-device support
- patch-level diff rendering for every commit
- advanced merge conflict visualization

---

## Future Extensions

Possible future additions:
- branch comparison analytics
- export reports as JSON/CSV/PDF
- system tray integration
- background daemon mode
- plugin architecture
- support for submodule insights
- commit graph visualization