# GitRadar 🚀

**GitRadar** is a desktop application that provides deep analytics and insights for your local Git repositories — similar to GitHub insights, but fully offline and focused on your local development workflow.

---

## ✨ Features (Planned)

* 📁 Recursive discovery of local Git repositories
* 📊 Commit analytics (daily/weekly trends)
* 🔥 File hotspots & churn analysis
* 🌿 Branch overview & activity tracking
* 🧾 Working directory insights (modified, staged, untracked)
* 👥 Contributor statistics (for multi-author repos)
* ⚡ Real-time updates via filesystem watching
* 🧠 Repository health indicators
* 🔍 Search across repositories, commits, and files

---

## 🏗️ Tech Stack

### Frontend

* TanStack (Router, Query)
* React + TypeScript
* Vite
* Recharts (for analytics visualization)

### Backend

* Tauri (Rust)
* libgit2 (Git operations)
* SQLite (local data storage)

### Runtime & Tooling

* Bun (package manager & runtime)

---

## 📂 Project Structure

```
gitradar/
├── src/                # Frontend (React + TanStack)
├── src-tauri/          # Rust backend (Tauri)
├── public/             # Static assets
├── docs/               # Documentation
├── scripts/            # Dev & build scripts
└── README.md
```

---

## 🚀 Getting Started

### Prerequisites

* Bun
* Rust (latest stable)
* Just (task runner - optional but recommended)
* Tauri CLI (auto-installed with setup)

---

### Quick Setup (Recommended)

Run the setup script to configure everything automatically:

```bash
./scripts/setup.sh
```

This script will:
- Check for required dependencies (Bun, Rust)
- Install Node.js dependencies
- Install Tauri CLI
- Verify system requirements

---

### Development Commands

#### Using Just (Recommended)

```bash
# Setup development environment
just setup

# Start development server with hot reload
just dev

# Run all tests (TypeScript + Rust)
just test

# Clean build artifacts
just clean

# Build for production
just build

# Package the application
just package

# Format code
just fmt

# Run linter
just lint

# Watch for changes and auto-rebuild
just watch

# Show all available commands
just info
```

#### Using Scripts

```bash
# Start development server with hot reload
./scripts/dev.sh
# or
bun run tauri dev

# Run all tests (TypeScript + Rust)
./scripts/test.sh

# Clean build artifacts
./scripts/clean.sh

# Build for production
./scripts/build.sh
# or
bun run tauri build

# Package the application
./scripts/package.sh
```

---

### Manual Setup

If you prefer to set up manually:

1. **Install dependencies**
   ```bash
   bun install
   ```

2. **Start development**
   ```bash
   bun run tauri dev
   ```

---

### Code Quality & Formatting

GitRadar uses automated tools to maintain consistent code quality and formatting across both TypeScript and Rust codebases.

#### Using Just (Recommended)

```bash
# Format all code (TypeScript + Rust)
just fmt

# Check formatting without fixing
just fmt-check

# Run all linters
just lint

# Fix auto-fixable linting issues
just lint-fix

# Run type checks for both TS and Rust
just type-check

# Complete code quality check
just check
```

#### Using NPM Scripts

```bash
# TypeScript/React
bun run format          # Format TS/React code
bun run format:check    # Check formatting
bun run lint           # Run ESLint
bun run lint:fix       # Fix linting issues
bun run type-check     # TypeScript type check

# Rust
bun run rust:fmt       # Format Rust code
bun run rust:lint      # Run Clippy
bun run rust:check     # Rust type check

# All checks
bun run check:all      # Run all checks
```

#### Configuration Files

- **`.prettierrc`** - TypeScript/React formatting rules
- **`.eslintrc.json`** - TypeScript/React linting rules  
- **`src-tauri/rustfmt.toml`** - Rust formatting configuration
- **`.prettierignore`** & **`.eslintignore`** - Excluded files

The formatting ensures:
- Consistent indentation (2 spaces for TS, 4 for Rust)
- Proper line length (100 chars max)
- Consistent quote usage and trailing commas
- Automatic import organization
- Type safety enforcement

---

## 🧠 Core Concepts

GitRadar works by:

1. Discovering Git repositories in user-selected directories
2. Indexing commit history and file changes
3. Storing structured analytics in a local SQLite database
4. Watching filesystem changes for incremental updates
5. Rendering insights through an interactive UI

---

## 🗺️ Roadmap

### Phase 1 (MVP)

* Repo discovery
* Basic repo dashboard
* Commit history view
* Working directory status

### Phase 2

* Commit analytics
* File churn tracking
* Hotspot detection
* Search functionality

### Phase 3

* Real-time updates
* Advanced insights
* Health scoring
* Export reports

---

## 🧪 Development Philosophy

* Local-first
* Fast & lightweight
* Incremental indexing (no heavy rescans)
* Developer-centric insights

---

## 📜 License

MIT License

---

## 🤝 Contributing

This is currently a solo project. But open to contributions in the future.

---

## 💡 Inspiration

GitRadar is inspired by GitHub Insights, but designed to work **locally, privately, and efficiently**.

---

## Project Structure

```
gitradar/
│
├── src/                              # Frontend (React + TanStack)
│   ├── app/
│   │   ├── router/                   # TanStack Router setup
│   │   │   ├── index.tsx
│   │   │   ├── routeTree.gen.ts
│   │   │   └── routes/
│   │   │       ├── __root.tsx
│   │   │       ├── dashboard.tsx
│   │   │       ├── repositories.tsx
│   │   │       ├── repository.$id.tsx
│   │   │       ├── commits.tsx
│   │   │       ├── files.tsx
│   │   │       └── settings.tsx
│   │   │
│   │   ├── providers/                # App providers (Query, Theme, etc.)
│   │   │   ├── QueryProvider.tsx
│   │   │   └── ThemeProvider.tsx
│   │   │
│   │   ├── layout/
│   │   │   ├── AppLayout.tsx
│   │   │   ├── Sidebar.tsx
│   │   │   ├── Header.tsx
│   │   │   └── Content.tsx
│   │   │
│   │   └── App.tsx
│   │
│   ├── features/                     # Feature-based modular structure
│   │   ├── dashboard/
│   │   │   ├── components/
│   │   │   │   ├── RepoOverviewCard.tsx
│   │   │   │   ├── ActivityChart.tsx
│   │   │   │   └── HealthScore.tsx
│   │   │   ├── hooks/
│   │   │   │   └── useDashboardData.ts
│   │   │   └── api/
│   │   │       └── dashboardApi.ts
│   │   │
│   │   ├── repositories/
│   │   │   ├── components/
│   │   │   │   ├── RepoCard.tsx
│   │   │   │   ├── RepoList.tsx
│   │   │   │   └── RepoFilters.tsx
│   │   │   ├── hooks/
│   │   │   │   └── useRepositories.ts
│   │   │   └── api/
│   │   │       └── repoApi.ts
│   │   │
│   │   ├── repository-details/
│   │   │   ├── components/
│   │   │   │   ├── RepoHeader.tsx
│   │   │   │   ├── BranchList.tsx
│   │   │   │   ├── WorkingTreePanel.tsx
│   │   │   │   └── RepoStats.tsx
│   │   │   ├── hooks/
│   │   │   │   └── useRepoDetails.ts
│   │   │   └── api/
│   │   │       └── repoDetailsApi.ts
│   │   │
│   │   ├── commits/
│   │   │   ├── components/
│   │   │   │   ├── CommitList.tsx
│   │   │   │   ├── CommitItem.tsx
│   │   │   │   └── CommitGraph.tsx
│   │   │   ├── hooks/
│   │   │   │   └── useCommits.ts
│   │   │   └── api/
│   │   │       └── commitsApi.ts
│   │   │
│   │   ├── files/
│   │   │   ├── components/
│   │   │   │   ├── FileHotspots.tsx
│   │   │   │   ├── FileChurnChart.tsx
│   │   │   │   └── FileList.tsx
│   │   │   ├── hooks/
│   │   │   │   └── useFiles.ts
│   │   │   └── api/
│   │   │       └── filesApi.ts
│   │   │
│   │   ├── analytics/
│   │   │   ├── components/
│   │   │   │   ├── ChurnChart.tsx
│   │   │   │   ├── CommitFrequency.tsx
│   │   │   │   └── ContributorStats.tsx
│   │   │   ├── hooks/
│   │   │   │   └── useAnalytics.ts
│   │   │   └── api/
│   │   │       └── analyticsApi.ts
│   │   │
│   │   └── settings/
│   │       ├── components/
│   │       │   └── SettingsPanel.tsx
│   │       ├── hooks/
│   │       │   └── useSettings.ts
│   │       └── api/
│   │           └── settingsApi.ts
│   │
│   ├── shared/                       # Shared utilities/components
│   │   ├── components/
│   │   │   ├── ui/
│   │   │   │   ├── Button.tsx
│   │   │   │   ├── Card.tsx
│   │   │   │   ├── Badge.tsx
│   │   │   │   └── Spinner.tsx
│   │   │   └── common/
│   │   │       ├── SearchBar.tsx
│   │   │       ├── EmptyState.tsx
│   │   │       └── ErrorBoundary.tsx
│   │   │
│   │   ├── hooks/
│   │   ├── utils/
│   │   │   ├── format.ts
│   │   │   ├── date.ts
│   │   │   ├── constants.ts
│   │   │   ├── type.ts
│   │   │   └── helpers.ts
│   │   │
│   │   └── lib/
│   │       ├── queryClient.ts
│   │       └── tauriClient.ts       # bridge to Rust commands
│   │
│   ├── styles/
│   │   ├── globals.css
│   │   └── theme.css
│   │
│   ├── main.tsx
│   └── vite-env.d.ts
│
├── src-tauri/                       # Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── state.rs                # global app state
│   │   |
│   │   ├── commands/              # Tauri command handlers
│   │   │   ├── mod.rs
│   │   │   ├── repos.rs
│   │   │   ├── commits.rs
│   │   │   ├── files.rs
│   │   │   ├── analytics.rs
│   │   │   └── settings.rs
│   │   |
│   │   ├── core/                  # core logic
│   │   │   ├── mod.rs
│   │   │   ├── repo_discovery.rs
│   │   │   ├── git_service.rs
│   │   │   ├── diff_service.rs
│   │   │   ├── watcher.rs
│   │   │   ├── scheduler.rs
│   │   │   └── permissions.rs
│   │   |
│   │   ├── analytics/
│   │   │   ├── mod.rs
│   │   │   ├── churn.rs
│   │   │   ├── commit_frequency.rs
│   │   │   ├── hotspots.rs
│   │   │   ├── contributors.rs
│   │   │   └── health.rs
│   │   |
│   │   ├── db/
│   │   │   ├── mod.rs
│   │   │   ├── connection.rs
│   │   │   ├── migrations.rs
│   │   │   ├── repositories.rs
│   │   │   ├── commits.rs
│   │   │   ├── file_stats.rs
│   │   │   ├── snapshots.rs
│   │   │   └── settings.rs
│   │   |
│   │   ├── models/
│   │   │   ├── mod.rs
│   │   │   ├── repository.rs
│   │   │   ├── commit.rs
│   │   │   ├── file_change.rs
│   │   │   ├── branch.rs
│   │   │   └── analytics.rs
│   │   |
│   │   ├── jobs/                  # background jobs
│   │   │   ├── mod.rs
│   │   │   ├── initial_scan.rs
│   │   │   ├── incremental_index.rs
│   │   │   └── refresh_repo.rs
│   │   |
│   │   ├── utils/
│   │   │   ├── mod.rs
│   │   │   ├── paths.rs
│   │   │   ├── hashing.rs
│   │   │   ├── time.rs
│   │   │   └── errors.rs
│   │   |
│   │   └── config/
│   │       ├── mod.rs
│   │       └── app_config.rs
│   │
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── build.rs
│   ├── icons/
│   └── capabilities/
│
├── public/
│   ├── favicon.ico
│   └── logo.svg
│
├── docs/                           # project documentation
│   ├── architecture.md
│   ├── database-schema.md
│   ├── analytics-metrics.md
│   ├── api-contract.md
│   └── roadmap.md
│
├── scripts/
│   ├── setup.sh      # Initial environment setup
│   ├── dev.sh        # Start development server
│   ├── build.sh      # Build for production
│   ├── test.sh       # Run all tests
│   ├── clean.sh      # Clean build artifacts
│   └── package.sh    # Package the application
│
├── justfile           # Just task runner commands
│
├── .gitignore
├── package.json
├── bun.lockb
├── tsconfig.json
├── vite.config.ts
└── README.md

```