use std::collections::HashMap;
use std::path::Path;

use crate::infrastructure::git::{
    change_type, ChangeType, CommitDiff, CommitInlineDiff, DiffHunk, DiffLine, DiffLineType,
    FileDiff, InlineDiffLine, InlineFileDiff,
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

pub fn get_commit_inline_diff(
    repo_path: &str,
    commit_hash: &str,
) -> Result<CommitInlineDiff, String> {
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
        files.push(InlineFileBuilder {
            old_path: delta_path(&delta.old_file()),
            new_path,
            change_type: change_type(delta.status()),
            hunk_lines: Vec::new(),
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

        files[file_index].hunk_lines.push(HunkLine {
            old_start: hunk.old_start(),
            old_lines: hunk.old_lines(),
            new_start: hunk.new_start(),
            new_lines: hunk.new_lines(),
            line: InlineDiffLine {
                old_line_number: line.old_lineno(),
                new_line_number: line.new_lineno(),
                content: strip_line_ending(String::from_utf8_lossy(line.content()).into_owned()),
                line_type,
            },
        });

        true
    })
    .map_err(|error| format!("Failed to build commit inline diff: {error}"))?;

    let mut inline_files = Vec::with_capacity(files.len());
    for file in files {
        let base_lines = match file.change_type {
            ChangeType::Deleted => {
                read_tree_file_lines(&repo, parent_tree.as_ref(), &file.new_path)?
            }
            _ => read_tree_file_lines(&repo, Some(&current_tree), &file.new_path)?,
        };

        inline_files.push(file.into_inline_file_diff(base_lines));
    }

    Ok(CommitInlineDiff {
        commit_hash: commit.id().to_string(),
        files: inline_files,
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

#[derive(Debug)]
struct InlineFileBuilder {
    old_path: Option<String>,
    new_path: String,
    change_type: ChangeType,
    hunk_lines: Vec<HunkLine>,
}

impl InlineFileBuilder {
    fn into_inline_file_diff(self, base_lines: Vec<String>) -> InlineFileDiff {
        let lines = match self.change_type {
            ChangeType::Added => base_lines
                .into_iter()
                .enumerate()
                .map(|(index, content)| InlineDiffLine {
                    old_line_number: None,
                    new_line_number: Some((index + 1) as u32),
                    content,
                    line_type: DiffLineType::Added,
                })
                .collect(),
            ChangeType::Deleted => base_lines
                .into_iter()
                .enumerate()
                .map(|(index, content)| InlineDiffLine {
                    old_line_number: Some((index + 1) as u32),
                    new_line_number: None,
                    content,
                    line_type: DiffLineType::Removed,
                })
                .collect(),
            ChangeType::Modified | ChangeType::Renamed | ChangeType::Copied => {
                merge_hunks_with_new_file(base_lines, self.hunk_lines)
            }
        };

        InlineFileDiff {
            old_path: self.old_path,
            new_path: self.new_path,
            change_type: self.change_type,
            lines,
        }
    }
}

#[derive(Debug)]
struct HunkLine {
    old_start: u32,
    old_lines: u32,
    new_start: u32,
    new_lines: u32,
    line: InlineDiffLine,
}

fn merge_hunks_with_new_file(
    base_lines: Vec<String>,
    hunk_lines: Vec<HunkLine>,
) -> Vec<InlineDiffLine> {
    let mut lines = Vec::with_capacity(base_lines.len() + hunk_lines.len());
    let mut old_cursor = 1_u32;
    let mut new_cursor = 1_u32;
    let mut current_hunk: Option<(u32, u32, u32, u32)> = None;

    for hunk_line in hunk_lines {
        let hunk_key = (
            hunk_line.old_start,
            hunk_line.old_lines,
            hunk_line.new_start,
            hunk_line.new_lines,
        );

        if current_hunk != Some(hunk_key) {
            append_context_lines(
                &mut lines,
                &base_lines,
                old_cursor,
                new_cursor,
                hunk_line.new_start,
            );
            old_cursor = hunk_line.old_start.saturating_add(hunk_line.old_lines);
            new_cursor = hunk_line.new_start.saturating_add(hunk_line.new_lines);
            current_hunk = Some(hunk_key);
        }

        lines.push(hunk_line.line);
    }

    append_context_lines(
        &mut lines,
        &base_lines,
        old_cursor,
        new_cursor,
        (base_lines.len() + 1) as u32,
    );

    lines
}

fn append_context_lines(
    lines: &mut Vec<InlineDiffLine>,
    base_lines: &[String],
    old_start_line: u32,
    new_start_line: u32,
    new_end_line: u32,
) {
    for new_line_number in new_start_line..new_end_line {
        if new_line_number == 0 {
            continue;
        }

        let Some(content) = base_lines.get((new_line_number - 1) as usize) else {
            continue;
        };
        let old_line_number = old_start_line + (new_line_number - new_start_line);

        lines.push(InlineDiffLine {
            old_line_number: Some(old_line_number),
            new_line_number: Some(new_line_number),
            content: content.clone(),
            line_type: DiffLineType::Context,
        });
    }
}

fn read_tree_file_lines(
    repo: &git2::Repository,
    tree: Option<&git2::Tree<'_>>,
    path: &str,
) -> Result<Vec<String>, String> {
    let Some(tree) = tree else {
        return Ok(Vec::new());
    };

    let entry = tree
        .get_path(Path::new(path))
        .map_err(|error| format!("Failed to find '{path}' in commit tree: {error}"))?;
    let blob = entry
        .to_object(repo)
        .and_then(|object| object.peel_to_blob())
        .map_err(|error| format!("Failed to read blob for '{path}': {error}"))?;

    Ok(split_blob_lines(blob.content()))
}

fn split_blob_lines(content: &[u8]) -> Vec<String> {
    String::from_utf8_lossy(content)
        .lines()
        .map(ToOwned::to_owned)
        .collect()
}

fn strip_line_ending(mut content: String) -> String {
    if content.ends_with('\n') {
        content.pop();
        if content.ends_with('\r') {
            content.pop();
        }
    }

    content
}
