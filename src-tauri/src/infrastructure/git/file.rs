use std::collections::HashMap;

use crate::infrastructure::git::{
    change_type, CommitDiff, DiffHunk, DiffLine, DiffLineType, FileDiff,
};

pub fn get_file_diff(
    repo_path: &str,
    commit_hash: &str,
    file_path: &str,
) -> Result<FileDiff, String> {
    get_commit_diff(repo_path, commit_hash)?
        .files
        .into_iter()
        .find(|diff| diff.new_path == file_path || diff.old_path.as_deref() == Some(file_path))
        .ok_or_else(|| format!("File '{file_path}' was not changed by commit '{commit_hash}'"))
}

pub fn get_commit_diff(repo_path: &str, commit_hash: &str) -> Result<CommitDiff, String> {
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
    let mut diff = repo
        .diff_tree_to_tree(parent_tree.as_ref(), Some(&current_tree), None)
        .map_err(|error| format!("Failed to create commit diff: {error}"))?;
    diff.find_similar(None)
        .map_err(|error| format!("Failed to detect renamed files: {error}"))?;

    let mut files = Vec::with_capacity(diff.deltas().len());
    let mut file_indices = HashMap::new();

    for delta in diff.deltas() {
        let Some(new_path) =
            delta_path(&delta.new_file()).or_else(|| delta_path(&delta.old_file()))
        else {
            continue;
        };

        let key = delta_key(&delta);
        file_indices.insert(key, files.len());
        files.push(FileDiff {
            old_path: delta_path(&delta.old_file()),
            new_path,
            change_type: change_type(delta.status()),
            additions: 0,
            deletions: 0,
            hunks: Vec::new(),
        });
    }

    diff.print(git2::DiffFormat::Patch, |delta, hunk, line| {
        let Some(&file_index) = file_indices.get(&delta_key(&delta)) else {
            return true;
        };

        let Some(hunk) = hunk else {
            return true;
        };

        let Some(line_type) = diff_line_type(line.origin()) else {
            return true;
        };

        let file = &mut files[file_index];
        if !is_current_hunk(file.hunks.last(), &hunk) {
            file.hunks.push(DiffHunk {
                old_start: hunk.old_start() as i32,
                old_lines: hunk.old_lines() as i32,
                new_start: hunk.new_start() as i32,
                new_lines: hunk.new_lines() as i32,
                lines: Vec::new(),
            });
        }

        match line_type {
            DiffLineType::Added => file.additions += 1,
            DiffLineType::Removed => file.deletions += 1,
            DiffLineType::Context => {}
        }

        if let Some(current_hunk) = file.hunks.last_mut() {
            current_hunk.lines.push(DiffLine {
                line_type,
                old_line_number: line.old_lineno(),
                new_line_number: line.new_lineno(),
                content: String::from_utf8_lossy(line.content()).into_owned(),
            });
        }

        true
    })
    .map_err(|error| format!("Failed to build commit diff: {error}"))?;

    Ok(CommitDiff {
        commit_hash: commit.id().to_string(),
        files,
    })
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

fn diff_line_type(origin: char) -> Option<DiffLineType> {
    match origin {
        ' ' => Some(DiffLineType::Context),
        '+' => Some(DiffLineType::Added),
        '-' => Some(DiffLineType::Removed),
        _ => None,
    }
}

fn is_current_hunk(current: Option<&DiffHunk>, next: &git2::DiffHunk<'_>) -> bool {
    current.is_some_and(|hunk| {
        hunk.old_start == next.old_start() as i32
            && hunk.old_lines == next.old_lines() as i32
            && hunk.new_start == next.new_start() as i32
            && hunk.new_lines == next.new_lines() as i32
    })
}
