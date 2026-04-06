# GitRadar 🚀

**GitRadar** is a Linux-first desktop application that provides deep analytics and insights for your local Git repositories — similar to GitHub insights, but fully offline and focused on your local development workflow.

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
* Node.js (optional fallback)
* Rust (latest stable)
* Tauri CLI

---

### Install dependencies

```bash
bun install
```

---

### Run development app

```bash
bun run tauri dev
```

---

### Build for production

```bash
bun run tauri build
```

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
│   │   │   └── helpers.ts
│   │   │
│   │   ├── types/
│   │   │   ├── repository.ts
│   │   │   ├── commit.ts
│   │   │   ├── file.ts
│   │   │   └── analytics.ts
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
│   ├── dev.sh
│   ├── build.sh
│   └── package.sh
│
├── .gitignore
├── package.json
├── bun.lockb
├── tsconfig.json
├── vite.config.ts
└── README.md

```