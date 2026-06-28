use std::collections::HashMap;

use rusqlite::Connection;

use crate::{
    domain::{
        file::FileTreeNode, CommitFileStat, DomainError, DomainResult, FileHotspot, LanguageStat,
        LanguageStats, RepositoryFile,
    },
    infrastructure::database::{
        models::file_change::{
            CommitFileStat as DatabaseCommitFileStat, FileHotspot as DatabaseFileHotspot,
            RepositoryFile as DatabaseRepositoryFile,
        },
        repositories::{file_stats, repository_files},
    },
};

pub fn get_repository_files(conn: &Connection, repo_id: i64) -> DomainResult<Vec<RepositoryFile>> {
    repository_files::get_repository_files(conn, repo_id)
        .map_err(|error| file_database_error("load repository files", error))
        .map(|files| files.into_iter().map(map_repository_file).collect())
}

pub fn get_repository_file_by_path(
    conn: &Connection,
    repo_id: i64,
    file_path: &str,
) -> DomainResult<RepositoryFile> {
    repository_files::get_repository_file_by_path(conn, repo_id, file_path)
        .map_err(|error| file_database_error("load repository file", error))?
        .map(map_repository_file)
        .ok_or_else(|| DomainError::InvalidRepository("Repository file not found".into()))
}

pub fn get_files_by_extension(
    conn: &Connection,
    repo_id: i64,
    extension: &str,
) -> DomainResult<Vec<RepositoryFile>> {
    repository_files::get_files_by_extension(conn, repo_id, extension)
        .map_err(|error| file_database_error("load repository files by extension", error))
        .map(|files| files.into_iter().map(map_repository_file).collect())
}

pub fn get_repo_languages_stats(conn: &Connection, repo_id: i64) -> DomainResult<LanguageStats> {
    let extensions_and_sizes = repository_files::get_files_extensions_and_file_sizes(conn, repo_id)
        .map_err(|error| file_database_error("load files extensions and sizes", error))?;

    let mut total_size: u64 = 0;
    let mut language_sizes: HashMap<String, u64> = HashMap::new();

    for (extension, size) in extensions_and_sizes {
        let language = match map_extension_to_language(&extension) {
            Some(lang) => lang,
            None => continue, // Skip files with unrecognized extensions
        };

        total_size += size as u64;

        *language_sizes.entry(language.to_string()).or_insert(0) += size as u64;
    }

    if total_size == 0 {
        return Ok(LanguageStats {
            total_bytes: 0,
            languages: Vec::new(),
        });
    }

    let result = language_sizes
        .into_iter()
        .map(|(language, bytes)| LanguageStat { language, bytes })
        .collect();

    Ok(LanguageStats {
        total_bytes: total_size,
        languages: result,
    })
}

pub fn get_file_stats(conn: &Connection, repo_id: i64) -> DomainResult<Vec<CommitFileStat>> {
    file_stats::get_file_stats(conn, repo_id)
        .map_err(|error| file_database_error("load file statistics", error))
        .map(|stats| stats.into_iter().map(map_file_stat).collect())
}

pub fn get_file_stats_by_path(
    conn: &Connection,
    repo_id: i64,
    file_path: &str,
) -> DomainResult<Vec<CommitFileStat>> {
    file_stats::get_file_stats_by_path(conn, repo_id, file_path)
        .map_err(|error| file_database_error("load file statistics by path", error))
        .map(|stats| stats.into_iter().map(map_file_stat).collect())
}

pub fn get_file_hotspots(conn: &Connection, repo_id: i64) -> DomainResult<Vec<FileHotspot>> {
    file_stats::get_file_hotspots(conn, repo_id)
        .map_err(|error| file_database_error("load file hotspots", error))
        .map(|hotspots| hotspots.into_iter().map(map_file_hotspot).collect())
}

pub fn get_repository_file_tree(
    conn: &Connection,
    repo_id: i64,
) -> DomainResult<Vec<FileTreeNode>> {
    let files = repository_files::get_repository_files(conn, repo_id)
        .map_err(|e| file_database_error("load repository files", e))?;

    let mut tree = Vec::new();

    for file in files {
        let size = file.size_bytes.unwrap_or(0) as u64;
        insert_into_tree(&mut tree, &file.file_path, size);
    }

    sort_tree(&mut tree);

    Ok(tree)
}

fn insert_into_tree(tree: &mut Vec<FileTreeNode>, full_path: &str, size: u64) {
    let parts: Vec<&str> = full_path.split('/').collect();
    insert_parts(tree, &parts, String::new(), size);
}

fn insert_parts(tree: &mut Vec<FileTreeNode>, parts: &[&str], current_path: String, size: u64) {
    if parts.is_empty() {
        return;
    }

    let name = parts[0];

    let path = if current_path.is_empty() {
        name.to_string()
    } else {
        format!("{}/{}", current_path, name)
    };

    let is_directory = parts.len() > 1;

    let index = tree
        .iter()
        .position(|n| n.name == name && n.is_directory == is_directory);

    let node = if let Some(i) = index {
        &mut tree[i]
    } else {
        tree.push(FileTreeNode {
            name: name.to_string(),
            path: path.clone(),
            size_or_file_count: if is_directory { 0 } else { size },
            is_directory,
            children: Vec::new(),
        });

        tree.last_mut().unwrap()
    };

    if node.is_directory {
        node.size_or_file_count += 1;
    }

    insert_parts(&mut node.children, &parts[1..], path, size);
}

fn sort_tree(tree: &mut Vec<FileTreeNode>) {
    tree.sort_by(|a, b| match (a.is_directory, b.is_directory) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.cmp(&b.name),
    });

    for node in tree {
        sort_tree(&mut node.children);
    }
}

fn map_repository_file(file: DatabaseRepositoryFile) -> RepositoryFile {
    RepositoryFile {
        id: file.id,
        repo_id: file.repo_id,
        path: file.file_path,
        name: file.file_name,
        extension: file.extension,
        size_bytes: file.size_bytes,
        is_binary: file.is_binary,
        last_modified_at: file.last_modified_at,
    }
}

fn map_file_stat(stat: DatabaseCommitFileStat) -> CommitFileStat {
    CommitFileStat {
        id: stat.id,
        repo_id: stat.repo_id,
        commit_hash: stat.commit_hash,
        file_path: stat.file_path,
        change_type: stat.change_type,
        additions: stat.additions,
        deletions: stat.deletions,
        total_changes: stat.total_changes,
    }
}

fn map_file_hotspot(hotspot: DatabaseFileHotspot) -> FileHotspot {
    FileHotspot {
        id: hotspot.id,
        repo_id: hotspot.repo_id,
        file_path: hotspot.file_path,
        touch_count: hotspot.touch_count,
        churn_score: hotspot.churn_score,
        hotspot_score: hotspot.hotspot_score,
        last_touched_at: hotspot.last_touched_at,
        updated_at: hotspot.updated_at,
    }
}

fn file_database_error(action: &str, error: rusqlite::Error) -> DomainError {
    DomainError::InvalidRepository(format!("Failed to {action}: {error}"))
}

fn map_extension_to_language(extension: &str) -> Option<&'static str> {
    match extension {
        // Rust
        "rs" => Some("Rust"),

        // JavaScript / TypeScript
        "js" | "jsx" => Some("JavaScript"),
        "ts" | "tsx" => Some("TypeScript"),

        // Python
        "py" => Some("Python"),

        // JVM
        "java" => Some("Java"),
        "kt" | "kts" => Some("Kotlin"),
        "scala" => Some("Scala"),
        "clj" | "cljs" => Some("Clojure"),

        // C Family
        "c" => Some("C"),
        "cpp" | "cxx" | "cc" => Some("C++"),
        "h" | "hpp" => Some("C/C++ Header"),
        "cs" => Some("C#"),

        // Systems
        "go" => Some("Go"),
        "zig" => Some("Zig"),

        // Apple
        "swift" => Some("Swift"),
        "m" => Some("Objective-C"),
        "mm" => Some("Objective-C++"),

        // Web
        "html" | "htm" => Some("HTML"),
        "css" => Some("CSS"),
        "php" => Some("PHP"),

        // Scripting
        "rb" => Some("Ruby"),
        "lua" => Some("Lua"),
        "pl" => Some("Perl"),
        "r" => Some("R"),

        // Functional
        "ex" | "exs" => Some("Elixir"),
        "erl" | "hrl" => Some("Erlang"),
        "fs" | "fsi" | "fsx" => Some("F#"),

        // Database
        "sql" => Some("SQL"),

        // Shell
        "sh" | "bash" | "zsh" | "fish" => Some("Shell"),
        "ps1" => Some("PowerShell"),
        "bat" | "cmd" => Some("Batch"),

        // Build tools
        "makefile" => Some("Makefile"),
        "justfile" => Some("Just"),
        "Dockerfile" => Some("Dockerfile"),

        _ => None,
    }
}
