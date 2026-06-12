# GitRadar Architecture

Version: 2.0

---

# Overview

GitRadar is a Linux-first desktop application designed to discover, index, analyze, and visualize local Git repositories while maintaining strict local-first security principles.

The architecture prioritizes:

* Fast startup
* Low latency UI
* Incremental indexing
* Secure filesystem access
* Local-only processing
* Future extensibility

---

# Technology Stack

## Frontend

* React
* TypeScript
* TanStack Query
* React Router
* Zustand

---

## Desktop Layer

* Tauri

Responsibilities:

* Native window
* File dialogs
* OS integration
* Secure IPC bridge

---

## Backend

* Rust

Responsibilities:

* Repository discovery
* Git indexing
* Analytics
* Sync scheduling
* Security enforcement

---

## Storage

* SQLite

Modes:

* WAL Enabled
* Foreign Keys Enabled
* Prepared Statements Only

---

# High-Level Architecture

```text
+--------------------------------------------------+
|                    React UI                      |
+--------------------------------------------------+
                      |
                      |
                      v
+--------------------------------------------------+
|                Tauri Command Layer               |
+--------------------------------------------------+
                      |
                      |
                      v
+--------------------------------------------------+
|               Application Services               |
+--------------------------------------------------+
| Repository Service                               |
| Commit Service                                   |
| Diff Service                                     |
| Analytics Service                                |
| Search Service                                   |
| Settings Service                                 |
+--------------------------------------------------+
                      |
                      |
                      v
+--------------------------------------------------+
|                  Indexing Engine                 |
+--------------------------------------------------+
| Discovery Engine                                 |
| Incremental Indexer                              |
| Sync Scheduler                                   |
| Working Tree Scanner                             |
+--------------------------------------------------+
                      |
                      |
                      v
+--------------------------------------------------+
|                 Security Layer                   |
+--------------------------------------------------+
| Path Validation                                  |
| Permission Validation                            |
| Git Command Sandbox                              |
| Audit Logger                                     |
+--------------------------------------------------+
                      |
                      |
                      v
+--------------------------------------------------+
|                    SQLite                        |
+--------------------------------------------------+
```

---

# Core Architectural Principles

## Local First

Everything operates locally.

No internet access is required.

No repository data leaves the machine.

---

## Explicit Permission Model

Only user-approved root folders may be scanned.

Example:

Allowed:

/home/user/projects

Blocked:

/home/user/Documents

Unless explicitly approved.

---

## Incremental Indexing

Never perform a full repository re-index unless necessary.

Track:

* Last indexed commit
* Last scan timestamp
* Changed working tree state

Only update deltas.

---

## Read Optimized

Dashboard loads from cached analytics.

Analytics are precomputed and persisted.

Avoid expensive Git operations during UI rendering.

---

# Component Design

## Repository Discovery Engine

Responsible for:

* Root scanning
* Git repository detection
* Repository registration

Workflow:

1. User adds root.
2. Discovery begins.
3. Repositories identified.
4. Database updated.

---

## Incremental Indexer

Responsible for:

* Commit indexing
* Branch indexing
* Analytics refresh

Algorithm:

```text
HEAD changed?
    |
    +-- No --> Skip
    |
    +-- Yes --> Index new commits only
```

---

## Working Tree Scanner

Responsible for:

* Modified files
* Staged files
* Deleted files
* Untracked files

Runs in background only while application is open.

---

## Sync Scheduler

Responsible for:

* Background refresh
* Debouncing filesystem events
* Queueing indexing jobs

Rules:

* Runs only when app is running
* Stops completely when app closes

No daemon mode in MVP.

---

# Security Layer

## Path Validator

Validates every filesystem request.

Rules:

* Must be inside approved root
* Must not escape root via symlink
* Must not access restricted paths

---

## Git Sandbox

All Git commands execute through a controlled abstraction.

Allowed:

* status
* log
* diff
* branch

Future:

* pull
* push
* fetch
* merge

Blocked:

* arbitrary shell commands

---

## Audit Logger

Records:

* Root added
* Root removed
* Settings changes
* Future Git operations

---

# Caching Strategy

## UI Cache

TanStack Query

Purpose:

* Prevent repetitive requests
* Instant page transitions

---

## Database Cache

Precomputed:

* Repository metrics
* Health score
* Hotspots

Stored in SQLite.

---

# Repository Detail Architecture

Repository page contains:

```text
Repository Details
│
├── Overview
│
├── Commit Graph
│
├── Branches
│
├── Contributors
│
├── File Explorer
│
├── Working Tree
│
├── Diffs
│
└── Analytics
```

---

# Commit Graph System

The commit graph must visualize:

* Parent relationships
* Branches
* Merge commits
* Branch divergence

Data source:

git log --graph equivalent data

Stored as:

Commit Node
Parent Hashes
Branch References

---

# File Explorer System

Supports:

* Repository tree navigation
* File preview
* File metadata

Supported:

✓ Text files

Future:

✓ Markdown preview

✓ Code syntax highlighting

---

# Diff Viewer

Supports:

## Working Tree Diff

Current changes.

## Commit Diff

Commit A → Commit B

## Branch Diff

Branch A → Branch B

Modes:

* Unified
* Side-by-side

---

# Future WakaTime Integration

New Service:

WakaTime Adapter

Responsibilities:

* Read WakaTime API
* Cache durations
* Map activity to repositories

Will not affect existing architecture.

---

# Future Git Operations Module

New Module:

Git Operations Service

Responsibilities:

* Pull
* Push
* Fetch
* Merge

Must use Git Sandbox.

Never allow arbitrary shell execution.

---

# Deployment Architecture

```text
User
 |
 v
GitRadar Desktop App
 |
 +-- React UI
 |
 +-- Tauri
 |
 +-- Rust Backend
 |
 +-- SQLite Database
 |
 +-- Local Git Repositories
```

No cloud services.

No external dependencies.

No always-running background processes.
