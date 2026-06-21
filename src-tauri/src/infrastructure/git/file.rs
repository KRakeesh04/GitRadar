use std::collections::HashMap;

use crate::infrastructure::git::{ChangeType, FilePatch};

pub fn get_file_patch(
    repo_path: &str,
    commit_hash: &str,
    file_path: &str,
) -> Result<FilePatch, String> {
    get_commit_patch(repo_path, commit_hash)?
        .into_iter()
        .find(|patch| patch.file_path == file_path || patch.old_path.as_deref() == Some(file_path))
        .ok_or_else(|| format!("File '{file_path}' was not changed by commit '{commit_hash}'"))
}

pub fn get_commit_patch(repo_path: &str, commit_hash: &str) -> Result<Vec<FilePatch>, String> {
    let repo = git2::Repository::open(repo_path)
        .map_err(|error| format!("Failed to open repository: {error}"))?;
    let oid = git2::Oid::from_str(commit_hash)
        .map_err(|error| format!("Invalid commit hash '{commit_hash}': {error}"))?;
    let commit = repo
        .find_commit(oid)
        .map_err(|error| format!("Failed to find commit '{commit_hash}': {error}"))?;

    let current_tree = commit
        .tree()
        .map_err(|error| format!("Failed to read commit tree: {error}"))?;
    let parent_tree = if commit.parent_count() == 0 {
        None
    } else {
        Some(
            commit
                .parent(0)
                .and_then(|parent| parent.tree())
                .map_err(|error| format!("Failed to read parent tree: {error}"))?,
        )
    };
    let diff = repo
        .diff_tree_to_tree(parent_tree.as_ref(), Some(&current_tree), None)
        .map_err(|error| format!("Failed to create commit diff: {error}"))?;

    let mut patches = Vec::with_capacity(diff.deltas().len());
    let mut patch_indices = HashMap::new();

    for delta in diff.deltas() {
        let file_path = delta_path(&delta.new_file()).or_else(|| delta_path(&delta.old_file()));
        let Some(file_path) = file_path else {
            continue;
        };

        let old_path = delta_path(&delta.old_file());
        let key = delta_key(&delta);
        patch_indices.insert(key, patches.len());
        patches.push(FilePatch {
            file_path,
            old_path,
            change_type: change_type(delta.status()),
            patch: String::new(),
        });
    }

    diff.print(git2::DiffFormat::Patch, |delta, _hunk, line| {
        if let Some(&index) = patch_indices.get(&delta_key(&delta)) {
            patches[index]
                .patch
                .push_str(&String::from_utf8_lossy(line.content()));
        }
        true
    })
    .map_err(|error| format!("Failed to render commit patch: {error}"))?;

    Ok(patches)
}

fn delta_path(file: &git2::DiffFile<'_>) -> Option<String> {
    file.path().map(|path| path.to_string_lossy().into_owned())
}

fn delta_key(delta: &git2::DiffDelta<'_>) -> String {
    format!(
        "{:?}\0{}\0{}",
        delta.status(),
        delta_path(&delta.old_file()).unwrap_or_default(),
        delta_path(&delta.new_file()).unwrap_or_default()
    )
}

fn change_type(status: git2::Delta) -> ChangeType {
    match status {
        git2::Delta::Added => ChangeType::Added,
        git2::Delta::Deleted => ChangeType::Deleted,
        git2::Delta::Renamed | git2::Delta::Copied => ChangeType::Renamed,
        git2::Delta::Modified | git2::Delta::Typechange => ChangeType::Modified,
        _ => ChangeType::Modified,
    }
}
