# Domain Layer Documentation

## Overview

The **Domain Layer** contains pure business logic with NO database access, NO serialization, and NO external dependencies. These are the core business entities and their behavior.

---

## File Structure

```
src-tauri/src/domain/
├── mod.rs                    # Layer exports & error types
├── value_objects.rs          # Immutable business values
├── repository.rs             # Repository business entity
├── commit.rs                 # Commit business entity
├── branch.rs                 # Branch business entity
└── contributor.rs            # Contributor business entity
```

---

## 📋 Detailed File Explanations

### 1. **mod.rs** - Domain Layer Module

**Purpose:** Central export point for all domain types and layer-wide error handling

**Key Components:**

```rust
pub mod branch;
pub mod commit;
pub mod contributor;
pub mod repository;
pub mod value_objects;

pub type DomainResult<T> = Result<T, DomainError>;

pub enum DomainError {
    InvalidRepository(String),
    InvalidCommit(String),
    InvalidBranch(String),
    HealthCheckFailed(String),
    ActivityCalculationFailed(String),
}
```

**What It Does:**

- ✓ Exports all domain models
- ✓ Defines domain-level error types (NOT database errors)
- ✓ Provides unified error handling for business logic

**When to Use:**

- When you need domain errors
- When importing domain types

---

### 2. **value_objects.rs** - Immutable Business Values

**Purpose:** Represent meaningful domain concepts with validation

#### **HealthScore** (0.0 to 1.0)

```rust
pub struct HealthScore(f32);
```

**Methods:**

| Method          | Purpose                | Example                                     |
| --------------- | ---------------------- | ------------------------------------------- |
| `new(score)`    | Create with validation | `HealthScore::new(0.8)?`                    |
| `value()`       | Get numeric value      | Returns `0.8`                               |
| `status()`      | Get health status enum | Returns `HealthStatus::Good`                |
| `is_healthy()`  | Is score >= 0.7?       | Returns `true` for 0.8                      |
| `description()` | Human-readable status  | Returns `"Repository is in good condition"` |

**Status Levels:**

- `Excellent` (≥0.8) - No action needed
- `Good` (≥0.6) - Monitoring recommended
- `Fair` (≥0.4) - Minor issues present
- `Poor` (≥0.2) - Significant issues
- `Critical` (<0.2) - Urgent attention needed

**Example Usage:**

```rust
let score = HealthScore::new(0.85)?;
if score.is_healthy() {
    println!("{}", score.description()); // "Repository is in excellent condition"
}
```

---

#### **ActivityLevel** - Commit Frequency Classification

```rust
pub enum ActivityLevel {
    VeryLow,    // < 1 commit/week
    Low,        // 1-3 commits/week
    Moderate,   // 4-10 commits/week
    High,       // 11-20 commits/week
    VeryHigh,   // > 20 commits/week
}
```

**Methods:**

| Method                       | Purpose                     | Example                                              |
| ---------------------------- | --------------------------- | ---------------------------------------------------- |
| `from_weekly_commits(count)` | Calculate from commit count | `ActivityLevel::from_weekly_commits(8)` → `Moderate` |
| `description()`              | Human-readable text         | Returns `"Moderate - Regular updates"`               |
| `is_active()`                | Is activity >= Moderate?    | Returns `true` for Moderate+                         |

**Example Usage:**

```rust
let activity = ActivityLevel::from_weekly_commits(15);
if activity.is_active() {
    println!("Repository is actively maintained");
}
```

---

#### **RepositoryId** - Type-Safe ID

```rust
pub struct RepositoryId(pub i64);
```

**Methods:**

| Method    | Purpose                              |
| --------- | ------------------------------------ |
| `new(id)` | Create with validation (must be > 0) |
| `value()` | Get the ID value                     |

---

#### **CommitCount** - Commit Quantity

```rust
pub struct CommitCount(pub u32);
```

**Methods:**

| Method       | Purpose               |
| ------------ | --------------------- |
| `new(count)` | Create a commit count |
| `value()`    | Get the count         |
| `is_empty()` | Is count == 0?        |

---

### 3. **repository.rs** - Repository Business Entity

**Purpose:** Core business logic for repositories

#### **Repository Structure**

```rust
pub struct Repository {
    pub id: RepositoryId,
    pub name: String,
    pub path: PathBuf,
    pub git_dir: PathBuf,
    pub health_score: HealthScore,
    pub activity_level: ActivityLevel,
    pub default_branch: Option<String>,
    pub total_commits: CommitCount,
    pub unique_contributors: u32,
}
```

#### **Key Business Methods:**

| Method                                  | What It Does                                | Example                                        |
| --------------------------------------- | ------------------------------------------- | ---------------------------------------------- |
| `is_healthy()`                          | Health score >= 0.7?                        | `repo.is_healthy() → true`                     |
| `status()`                              | Returns `RepositoryStatus` enum             | `repo.status() → HealthyAndActive`             |
| `needs_maintenance()`                   | Bad health OR inactive?                     | `repo.needs_maintenance() → true`              |
| `calculate_risk_score()`                | Risk score 0.0-1.0                          | Based on health, activity, contributors        |
| `maintenance_priority()`                | `Critical`, `High`, `Medium`, `Low`, `None` | Determines action urgency                      |
| `is_dormant()`                          | No commits AND inactive?                    | `repo.is_dormant() → true`                     |
| `activity_description()`                | Human text with contributors                | `"High activity with 5 contributors"`          |
| `get_health_report()`                   | Full health analysis                        | Returns `HealthReport` struct                  |
| `validate_path()`                       | Path validation logic                       | Ensures `.git` in git_dir                      |
| `set_health_score(score)`               | Update health score                         | `repo.set_health_score(0.8)?`                  |
| `set_activity_level(level)`             | Update activity                             | `repo.set_activity_level(ActivityLevel::High)` |
| `update_metrics(commits, contributors)` | Update counts                               | `repo.update_metrics(100, 5)`                  |

#### **Example Usage:**

```rust
// Create repository
let mut repo = Repository::new(
    1,
    "my-repo".to_string(),
    PathBuf::from("/path/to/repo"),
    PathBuf::from("/path/to/repo/.git"),
)?;

// Update metrics (from database/analysis)
repo.set_health_score(0.85)?;
repo.set_activity_level(ActivityLevel::High);
repo.update_metrics(500, 10);

// Use business logic
if repo.is_healthy() && !repo.is_dormant() {
    println!("Repository is in good condition");
    println!("Maintenance priority: {:?}", repo.maintenance_priority());
}

// Get full report
let report = repo.get_health_report();
println!("Risk score: {}", report.risk_score);
```

---

#### **Repository Status Enum:**

```rust
pub enum RepositoryStatus {
    HealthyAndActive,      // Good! Keep as-is
    HealthyButInactive,    // Monitor - may need attention
    UnhealthyButActive,    // Needs care
    UnhealthyAndInactive,  // Highest priority
}
```

---

### 4. **commit.rs** - Commit Business Entity

**Purpose:** Business logic for analyzing commits

#### **Commit Structure**

```rust
pub struct Commit {
    pub id: CommitId,
    pub hash: CommitHash,
    pub author_name: String,
    pub author_email: String,
    pub subject: String,
    pub body: Option<String>,
    pub parent_count: u32,
    pub committed_at: String,
    pub is_significant: bool,
}
```

#### **Key Business Methods:**

| Method                     | Purpose                        | Returns                       |
| -------------------------- | ------------------------------ | ----------------------------- |
| `is_merge_commit()`        | Has 2+ parents?                | `bool`                        |
| `is_root_commit()`         | Has 0 parents?                 | `bool`                        |
| `is_regular_commit()`      | Has exactly 1 parent?          | `bool`                        |
| `commit_type()`            | Returns `CommitType` enum      | `Root`, `Regular`, or `Merge` |
| `message_size()`           | Total message length           | `usize`                       |
| `is_well_documented()`     | Has body + subject > 10 chars? | `bool`                        |
| `determine_significance()` | Mark if merge or documented    | Sets `is_significant`         |
| `short_message()`          | First 50 chars of subject      | `String`                      |
| `get_commit_info()`        | Simplified info for display    | `CommitInfo` struct           |
| `validate()`               | Check integrity                | `Result<()>`                  |
| `set_body(body)`           | Set commit description         | Void                          |

#### **Example Usage:**

```rust
// Create commit
let mut commit = Commit::new(
    1,
    "abc123def456".to_string(),
    "John Doe".to_string(),
    "john@example.com".to_string(),
    "John Doe".to_string(),
    "john@example.com".to_string(),
    "Fix: critical login bug".to_string(),
    1,  // 1 parent = regular commit
    "2024-01-15T10:30:00Z".to_string(),
)?;

// Analyze commit
match commit.commit_type() {
    CommitType::Root => println!("Initial commit"),
    CommitType::Regular => println!("Normal commit"),
    CommitType::Merge => println!("Merge commit"),
}

// Check significance
commit.determine_significance();
if commit.is_significant {
    println!("This is a significant commit");
}

// Get display info
let info = commit.get_commit_info();
println!("Author: {}, Subject: {}", info.author, info.subject);
```

---

#### **CommitType Enum:**

```rust
pub enum CommitType {
    Root,    // Initial commit (0 parents)
    Regular, // Normal (1 parent)
    Merge,   // Merge (2+ parents)
}
```

---

### 5. **branch.rs** - Branch Business Entity

**Purpose:** Business logic for branches

#### **Branch Structure**

```rust
pub struct Branch {
    pub id: BranchId,
    pub repo_id: i64,
    pub name: String,
    pub branch_type: BranchType,
    pub is_head: bool,
    pub is_default: bool,
    pub ahead_count: u32,
    pub behind_count: u32,
}
```

#### **Key Business Methods:**

| Method                            | Purpose                       | Example                                 |
| --------------------------------- | ----------------------------- | --------------------------------------- |
| `is_ahead()`                      | Has commits not in default?   | Returns `true` if ahead_count > 0       |
| `is_behind()`                     | Missing commits from default? | Returns `true` if behind_count > 0      |
| `is_in_sync()`                    | No divergence?                | Returns `true` if ahead=0 & behind=0    |
| `status()`                        | Returns `BranchStatus` enum   | `InSync`, `Ahead`, `Behind`, `Diverged` |
| `sync_message()`                  | Human-readable sync state     | `"Ahead by 5 commits"`                  |
| `should_merge()`                  | Ready to merge?               | `true` if ahead & not behind            |
| `is_stale()`                      | Behind by many commits?       | `true` if behind > 50                   |
| `importance()`                    | Returns `BranchImportance`    | `Critical`, `High`, `Medium`, `Low`     |
| `update_sync_info(ahead, behind)` | Update commit counts          | Void                                    |

#### **Example Usage:**

```rust
let mut branch = Branch::new(
    1,
    1,
    "feature/new-dashboard".to_string(),
    false,
    false,
)?;

// Simulate sync analysis
branch.update_sync_info(5, 0); // 5 commits ahead

if branch.should_merge() {
    println!("Ready to merge: {}", branch.sync_message());
    // Output: "Ready to merge: Ahead by 5 commits"
}

// Check importance
println!("Importance: {:?}", branch.importance());
```

---

#### **BranchType Enum:**

Automatically detected from branch name:

```rust
pub enum BranchType {
    Main,       // "main" or "master"
    Develop,    // "develop" or "development"
    Release,    // "release/*" or "release-*"
    Hotfix,     // "hotfix/*" or "hotfix-*"
    Feature,    // "feature/*" or "feature-*"
    Other,      // Everything else
}
```

---

#### **BranchStatus Enum:**

```rust
pub enum BranchStatus {
    InSync,   // In sync with default branch
    Ahead,    // Has commits not merged
    Behind,   // Missing commits from default
    Diverged, // Both ahead and behind
}
```

---

### 6. **contributor.rs** - Contributor Business Entity

**Purpose:** Business logic for contributors

#### **Contributor Structure**

```rust
pub struct Contributor {
    pub id: ContributorId,
    pub repo_id: i64,
    pub name: String,
    pub email: String,
    pub commit_count: u32,
    pub additions: u32,
    pub deletions: u32,
    pub active_days: u32,
    pub last_commit_at: Option<String>,
}
```

#### **Key Business Methods:**

| Method                           | Purpose                            | Returns                                                       |
| -------------------------------- | ---------------------------------- | ------------------------------------------------------------- |
| `impact_score()`                 | Weighted contributor importance    | `0.0-1.0`                                                     |
| `contributor_level()`            | Returns tier/level                 | `CoreMaintainer`, `Major`, `Regular`, `Occasional`, `Minimal` |
| `commits_per_day()`              | Average commits/day                | `f32`                                                         |
| `changes_per_commit()`           | Average additions+deletions/commit | `u32`                                                         |
| `is_active()`                    | Committed recently?                | `bool`                                                        |
| `get_role()`                     | Human role with activity           | `ActiveMaintainer`, `Contributor`, etc.                       |
| `contribution_summary()`         | Full text summary                  | `"50 commits, 5000 added, 2000 removed..."`                   |
| `update_metrics(add, del, days)` | Update counts                      | Void                                                          |
| `increment_commit_count()`       | Add 1 commit                       | Void                                                          |

#### **Example Usage:**

```rust
let mut contributor = Contributor::new(
    1,
    1,
    "Jane Smith".to_string(),
    "jane@example.com".to_string(),
)?;

// Update from analysis
contributor.commit_count = 150;
contributor.additions = 15000;
contributor.deletions = 8000;
contributor.active_days = 200;
contributor.set_last_commit("2024-01-15T10:00:00Z".to_string());

// Analyze contribution
println!("Impact: {:.2}", contributor.impact_score()); // e.g., 0.85
println!("Role: {}", contributor.get_role()); // "Active Maintainer"
println!("Commits/day: {}", contributor.commits_per_day()); // 0.75
println!("Summary: {}", contributor.contribution_summary());
```

---

#### **ContributorLevel Enum:**

```rust
pub enum ContributorLevel {
    Minimal,              // Very little contribution
    Occasional,           // Few commits
    RegularContributor,   // Consistent contribution
    MajorContributor,     // High impact
    CoreMaintainer,       // Main developer(s)
}
```

---

## 🔄 Workflow: From Persistence to Domain

### Example: Loading Repository from Database

```rust
// ❌ OLD (BAD): Commands → Database directly
pub fn get_repository(id: i64) {
    let persistence = models::Repository { id, name, ... }; // Database model
    Ok(persistence) // Directly return DB model
}

// ✅ NEW (GOOD): Database → Domain → Commands
pub fn get_repository(id: i64) {
    // 1. Fetch persistence model from database
    let persistence = db::repositories::get_by_id(id)?;

    // 2. Map to domain model
    let mut domain = domain::Repository::new(
        persistence.id,
        persistence.name,
        PathBuf::from(&persistence.path),
        PathBuf::from(&persistence.git_dir_path),
    )?;

    // 3. Set business state
    domain.set_health_score(calculate_health(&persistence))?;
    domain.set_activity_level(calculate_activity(&persistence));
    domain.update_metrics(persistence.total_commits, persistence.contributors);

    // 4. Return domain model to service/command
    Ok(domain)
}
```

---

## 🎯 Key Principles

1. **Domain models are pure business logic** - No database access
2. **Value Objects are immutable** - Cannot be changed after creation
3. **All methods are pure functions** - Same input always gives same output
4. **Domain errors are business errors** - Not database or infrastructure errors
5. **Validation happens in constructors** - Invalid states cannot exist
6. **Methods return domain types** - Not database types, not strings

---

## ✅ Testing

Each file includes comprehensive unit tests. Run with:

```bash
cargo test --lib domain::
```

Tests cover:

- Object creation and validation
- Business logic calculations
- Enum determinations
- Edge cases

---

## 📝 Migration Checklist

When refactoring existing code to use domain layer:

- [ ] Extract business logic from database queries
- [ ] Move calculations from commands to domain methods
- [ ] Replace string-based types with domain value objects
- [ ] Add domain error handling
- [ ] Create mappers from persistence → domain models
- [ ] Update services to use domain models
- [ ] Update commands to return domain models

---

## 🚀 Next Steps

1. **Create Domain Mapper Module** - Convert persistence ↔ domain
2. **Create Services Layer** - Orchestrate domain logic
3. **Update Commands** - Call services instead of database directly
4. **Add Security Layer** - Validate access before domain operations
5. **Add Infrastructure Layer** - Clean up git/fs code
