<!-- src-tauri/
│
├── main.rs
│
├── app/
│   ├── startup.rs
│   ├── state.rs
│   └── config.rs
│
├── commands/
│   ├── repositories.rs
│   ├── commits.rs
│   ├── files.rs
│   ├── analytics.rs
│   ├── search.rs
│   ├── settings.rs
│   └── sync.rs
│
├── domain/
│   ├── repositories/
│   ├── commits/
│   ├── files/
│   ├── analytics/
│   ├── search/
│   └── settings/
│
├── services/
│   ├── git_service.rs
│   ├── diff_service.rs
│   ├── indexing_service.rs
│   ├── analytics_service.rs
│   ├── search_service.rs
│   └── sync_service.rs
│
├── infrastructure/
│   ├── database/
│   ├── git/
│   ├── filesystem/
│   ├── cache/
│   └── security/
│
├── scheduler/
│   ├── sync_scheduler.rs
│   ├── debounce.rs
│   └── jobs.rs
│
├── security/
│   ├── path_validator.rs
│   ├── git_sandbox.rs
│   ├── permissions.rs
│   └── audit_logger.rs
│
├── indexing/
│   ├── discovery.rs
│   ├── repository_indexer.rs
│   ├── commit_indexer.rs
│   ├── file_indexer.rs
│   └── analytics_indexer.rs
│
└── db/
    ├── migrations/
    ├── repositories/
    └── queries/ -->

# Backend Structure

```
src-tauri/
│
├── commands/
├── domain/
├── services/
├── infrastructure/
├── indexing/
├── scheduler/
├── security/
└── db/
```

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 1. COMMANDS LAYER
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Purpose:
Expose Tauri APIs to the frontend.

Contains:

- repositories.rs
- commits.rs
- files.rs
- analytics.rs
- search.rs
- settings.rs
- sync.rs

Responsibilities:

✓ Validate input
✓ Call services
✓ Map errors
✓ Return DTOs

Example:

Frontend
↓
Tauri Command
↓
RepositoryService

Never:

✗ SQL
✗ Git Commands
✗ Business Logic

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 2. DOMAIN LAYER
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Purpose:
Pure business models.

Contains:

Repository
Branch
Commit
CommitGraph
FileNode
Diff
Analytics
Settings

Example:

struct Repository
struct Commit
struct Branch

enum FileStatus {
Modified,
Staged,
Deleted,
Untracked
}

Responsibilities:

✓ Business entities
✓ Domain rules
✓ Shared types

Never:

✗ SQL
✗ Tauri
✗ Filesystem
✗ Git access

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 3. SERVICES LAYER
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Purpose:
Business logic.

Contains:

RepositoryService
CommitService
DiffService
AnalyticsService
SearchService
SyncService

Responsibilities:

✓ Coordinate operations
✓ Execute use cases
✓ Combine multiple repositories
✓ Apply business rules

Example:

Load Repository +
Load Commits +
Load Metrics
=

Repository Details

Never:

✗ SQL directly
✗ Tauri APIs
✗ Raw filesystem access

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 4. INFRASTRUCTURE LAYER
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Purpose:
External system implementations.

Contains:

database/
filesystem/
git/
cache/

Responsibilities:

database/
-----------

SQLite access

filesystem/
-----------

Directory scanning
File reading
Metadata

git/
-----

git2 wrapper
Git command abstraction

cache/
-------

Memory cache
Query cache

Example:

GitRepositoryProvider
SqliteRepositoryProvider
FileSystemProvider

Never:

✗ Business rules
✗ Analytics calculations

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 5. INDEXING LAYER
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Purpose:
Heavy background processing.

Contains:

discovery.rs
repository_indexer.rs
commit_indexer.rs
file_indexer.rs
analytics_indexer.rs

Responsibilities:

Discovery
---------

Find repositories

Repository Indexer
------------------

Repository metadata

Commit Indexer
--------------

Commits
Branches
Commit graph

File Indexer
------------

File tree
File metadata

Analytics Indexer
-----------------

Hotspots
Churn
Health metrics

Produces:

SQLite cached data

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 6. SCHEDULER LAYER
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Purpose:
Background work management.

Contains:

sync_scheduler.rs
jobs.rs
debounce.rs

Responsibilities:

✓ Queue jobs
✓ Retry failed jobs
✓ Debounce filesystem events
✓ Control indexing frequency

Example:

Filesystem Change
↓
Debounce
↓
Queue Job
↓
Indexer

Runs ONLY while app is open.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 7. SECURITY LAYER
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Purpose:
Central security enforcement.

Contains:

path_validator.rs
permissions.rs
git_sandbox.rs
audit_logger.rs

Responsibilities:

Path Validation
---------------

Prevent traversal attacks

Permissions
-----------

Validate approved roots

Git Sandbox
-----------

Allow only approved git commands

Audit Logs
----------

Record sensitive actions

Example:

Request File
↓
Path Validator
↓
Permission Check
↓
Read File

Never bypass this layer.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 8. DB LAYER
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Purpose:
Raw database operations.

Contains:

RepositoryRepository
CommitRepository
BranchRepository
SettingsRepository

Responsibilities:

✓ CRUD
✓ Transactions
✓ Queries
✓ Migrations

Example:

get_repository()

save_commit()

update_branch()

Never:

✗ Business logic
✗ Analytics calculations
✗ Git operations

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
DEPENDENCY RULE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Frontend
↓
Commands
↓
Services
↓
Domain
↓
Infrastructure
↓
DB

Allowed:

Commands → Services
Services → Infrastructure
Infrastructure → DB

Not Allowed:

Commands → DB
Commands → SQL
Services → Tauri
Domain → DB
Domain → Git
Domain → Filesystem

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
MENTAL MODEL
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Commands
=

API Layer

Domain
=

Business Objects

Services
=

Business Logic

Infrastructure
=

External Systems

Indexing
=

Background Processing

Scheduler
=

Job Management

Security
=

Protection Layer

DB
=

Persistence Layer
