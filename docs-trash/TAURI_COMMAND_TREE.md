services/

├── analytics_service.rs
│ ├── get_repository_activity()
│ │ Return persisted daily repository activity.
│ ├── get_contributors()
│ │ Return persisted repository contributors.
│ ├── get_top_contributors()
│ │ Return persisted contributors ordered by commit count.
│ └── get_contributor_by_email()
│ Return one persisted contributor by email.
├── branch_service.rs
│ ├── get_branch_info_by_name()
│ │ Return branch information by repository and name.
│ └── get_repository_branches()
│ Return all branches for a repository.
├── commit_service.rs
│ ├── get_commits()
│ │ Return a paginated repository commit list.
│ └── get_commit_by_hash()
│ Return one repository commit by hash.
├── file_service.rs
│ ├── get_repository_files()
│ │ Return persisted files for a repository.
│ ├── get_repository_file_by_path()
│ │ Return one persisted repository file by path.
│ ├── get_files_by_extension()
│ │ Return persisted repository files by extension.
│ ├── get_file_stats()
│ │ Return persisted file-change statistics.
│ ├── get_file_stats_by_path()
│ │ Return persisted file-change statistics for one path.
│ └── get_file_hotspots()
│ Return persisted file hotspots.
├── repository_discovery_service.rs
│ └── discover_repositories()
│ Discover repositories from enabled tracked roots.
├── repository_query_service.rs
│ ├── get_repository_info_by_id()
│ │ Return a repository domain model by id.
│ └── get_all_repositories()
│ Return a paginated repository list.
├── sync_service.rs
│ └── calculate_repository_metrics()
│ Calculate repository metrics.
└── tracked_root_service.rs
├── add_tracked_root_path()
│ Add a tracked root path.
├── get_all_tracked_root_paths()
│ Return all tracked root paths.
├── enable_or_disable_track_root_path()
│ Change a tracked root enabled state.
└── delete_tracked_root_path()
Delete a tracked root path.

commands/

├── repository_commands.rs
│ ├── get_repository_info()
│ ├── get_all_repositories()
│ └── discover_repositories()
│
├── tracked_root_commands.rs
│ ├── add_tracked_root_path()
│ ├── get_all_tracked_root_paths()
│ ├── set_tracked_root_enabled()
│ └── delete_tracked_root_path()
│
├── branch_commands.rs
│ ├── get_repository_branches()
│ └── get_branch_info()
│
├── commit_commands.rs
│ ├── get_commits()
│ └── get_commit_by_hash()
│
├── file_commands.rs
│ ├── get_repository_files()
│ ├── get_repository_file_by_path()
│ ├── get_files_by_extension()
│ ├── get_file_stats()
│ ├── get_file_stats_by_path()
│ └── get_file_hotspots()
│
├── analytics_commands.rs
│ ├── get_repository_activity()
│ ├── get_contributors()
│ ├── get_top_contributors()
│ └── get_contributor_by_email()
│
└── sync_commands.rs
└── calculate_repository_metrics()
