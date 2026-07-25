use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;

use chrono::{DateTime, TimeZone, Utc};
use git2::{BranchType, DiffFormat, Repository, Sort};
use rusqlite::Connection;
use walkdir::WalkDir;

use crate::{
    domain::{
        repository::RepositoryCalculatedMetrics, ActivityLevel, DomainError, DomainResult,
        HealthScore,
    },
    infrastructure::{
        database::repositories::{
            analytics, branches, commits, contributors, file_stats, indexing_jobs, repositories,
            repository_files, repository_health, working_tree,
        },
        git,
    },
};

pub fn calculate_repository_metrics(
    conn: &Connection,
    repo_id: i64,
) -> DomainResult<RepositoryCalculatedMetrics> {
    let week_ago = (Utc::now() - chrono::Duration::days(7)).to_rfc3339();

    let metrics = repositories::get_repository_metrics(conn, repo_id, &week_ago).map_err(|e| {
        DomainError::InvalidRepository(format!("Failed to load repository metrics: {e}"))
    })?;

    let total_commits = u32::try_from(metrics.total_commits.max(0)).unwrap_or(u32::MAX);

    let weekly_commits = u32::try_from(metrics.weekly_commits.max(0)).unwrap_or(u32::MAX);

    let unique_contributors = u32::try_from(metrics.unique_contributors.max(0)).unwrap_or(u32::MAX);

    let recency_score = metrics
        .last_commit_at
        .as_deref()
        .and_then(|timestamp| DateTime::parse_from_rfc3339(timestamp).ok())
        .map(
            |timestamp| match (Utc::now() - timestamp.with_timezone(&Utc)).num_days() {
                ..=7 => 1.0,
                8..=30 => 0.8,
                31..=90 => 0.6,
                91..=180 => 0.4,
                181..=365 => 0.2,
                _ => 0.0,
            },
        )
        .unwrap_or(0.0);

    let history_score = (total_commits as f32 / 100.0).min(1.0);

    let contributor_score = (unique_contributors as f32 / 3.0).min(1.0);

    let health_score = (recency_score * 0.6) + (history_score * 0.2) + (contributor_score * 0.2);

    Ok(RepositoryCalculatedMetrics {
        total_commits,
        weekly_commits,
        unique_contributors,
        health_score: HealthScore::new(health_score)
            .unwrap_or_else(|_| HealthScore::new(0.0).unwrap()),
        activity_level: ActivityLevel::from_weekly_commits(weekly_commits),
    })
}

pub fn sync_repository(
    conn: &mut Connection,
    repo_id: i64,
    on_progress: &mut dyn FnMut(i64, i32, i32, i32, &str),
) -> DomainResult<i64> {
    let job_id = indexing_jobs::create_indexing_job(conn, repo_id, "sync").map_err(|e| {
        DomainError::InvalidRepository(format!("Failed to create indexing job: {e}"))
    })?;

    indexing_jobs::mark_indexing_job_started(conn, job_id).map_err(|e| {
        DomainError::InvalidRepository(format!("Failed to start indexing job: {e}"))
    })?;
    on_progress(job_id, 0, 0, 9, "running");

    repositories::update_repository_sync_state(
        conn,
        repo_id,
        Some(&Utc::now().to_rfc3339()),
        Some(&Utc::now().to_rfc3339()),
        Some("running"),
    )
    .map_err(|e| {
        DomainError::InvalidRepository(format!("Failed to mark repository running: {e}"))
    })?;

    let result = (|| -> DomainResult<()> {
        let repo_path = load_repository_path(conn, repo_id)?;
        let repo = Repository::open(&repo_path).map_err(|e| {
            DomainError::InvalidRepository(format!("Failed to open repository: {e}"))
        })?;
        let context = RepoSyncContext::new(&repo_path, repo)?;

        let tx = conn.transaction().map_err(|e| {
            DomainError::InvalidRepository(format!("Failed to open transaction: {e}"))
        })?;

        let total_steps = 9usize;
        let mut completed_steps = 0usize;

        sync_branches_with_context(&tx, repo_id, &context)?;
        completed_steps += 1;
        update_indexing_progress(&tx, job_id, completed_steps, total_steps)?;
        on_progress(
            job_id,
            11,
            completed_steps as i32,
            total_steps as i32,
            "running",
        );

        let snapshot = collect_history_snapshot(&context.repo)?;

        sync_commits_with_snapshot(&tx, repo_id, &snapshot)?;
        completed_steps += 1;
        update_indexing_progress(&tx, job_id, completed_steps, total_steps)?;
        on_progress(
            job_id,
            22,
            completed_steps as i32,
            total_steps as i32,
            "running",
        );

        sync_contributors_with_snapshot(&tx, repo_id, &snapshot)?;
        completed_steps += 1;
        update_indexing_progress(&tx, job_id, completed_steps, total_steps)?;
        on_progress(
            job_id,
            33,
            completed_steps as i32,
            total_steps as i32,
            "running",
        );

        sync_repository_files_with_context(&tx, repo_id, &context)?;
        completed_steps += 1;
        update_indexing_progress(&tx, job_id, completed_steps, total_steps)?;
        on_progress(
            job_id,
            44,
            completed_steps as i32,
            total_steps as i32,
            "running",
        );

        sync_commit_file_stats_with_snapshot(&tx, repo_id, &snapshot)?;
        completed_steps += 1;
        update_indexing_progress(&tx, job_id, completed_steps, total_steps)?;
        on_progress(
            job_id,
            56,
            completed_steps as i32,
            total_steps as i32,
            "running",
        );

        sync_file_hotspots_with_snapshot(&tx, repo_id, &snapshot)?;
        completed_steps += 1;
        update_indexing_progress(&tx, job_id, completed_steps, total_steps)?;
        on_progress(
            job_id,
            67,
            completed_steps as i32,
            total_steps as i32,
            "running",
        );

        sync_repo_activity_with_snapshot(&tx, repo_id, &snapshot)?;
        completed_steps += 1;
        update_indexing_progress(&tx, job_id, completed_steps, total_steps)?;
        on_progress(
            job_id,
            78,
            completed_steps as i32,
            total_steps as i32,
            "running",
        );

        sync_working_tree_status_with_context(&tx, repo_id, &context)?;
        completed_steps += 1;
        update_indexing_progress(&tx, job_id, completed_steps, total_steps)?;
        on_progress(
            job_id,
            89,
            completed_steps as i32,
            total_steps as i32,
            "running",
        );

        upsert_repository_health_snapshot(&tx, repo_id)?;
        completed_steps += 1;
        update_indexing_progress(&tx, job_id, completed_steps, total_steps)?;

        tx.commit().map_err(|e| {
            DomainError::InvalidRepository(format!("Failed to commit sync transaction: {e}"))
        })?;

        repositories::update_repository_sync_state(
            conn,
            repo_id,
            Some(&Utc::now().to_rfc3339()),
            Some(&Utc::now().to_rfc3339()),
            Some("completed"),
        )
        .map_err(|e| {
            DomainError::InvalidRepository(format!("Failed to mark repository complete: {e}"))
        })?;

        indexing_jobs::complete_indexing_job(conn, job_id).map_err(|e| {
            DomainError::InvalidRepository(format!("Failed to complete indexing job: {e}"))
        })?;

        let _ = indexing_jobs::cleanup_completed_indexing_jobs(conn, 30);

        on_progress(
            job_id,
            100,
            total_steps as i32,
            total_steps as i32,
            "completed",
        );

        Ok(())
    })();

    if let Err(error) = result {
        let _ = repositories::update_repository_sync_state(
            conn,
            repo_id,
            Some(&Utc::now().to_rfc3339()),
            None,
            Some("failed"),
        );
        let _ = indexing_jobs::fail_indexing_job(conn, job_id, &format!("{error}"));
        return Err(error);
    }

    Ok(job_id)
}

pub fn sync_branches(conn: &mut Connection, repo_id: i64) -> DomainResult<()> {
    let repo_path = load_repository_path(conn, repo_id)?;
    let repo = Repository::open(&repo_path)
        .map_err(|e| DomainError::InvalidRepository(format!("Failed to open repository: {e}")))?;
    let context = RepoSyncContext::new(&repo_path, repo)?;
    let tx = conn
        .transaction()
        .map_err(|e| DomainError::InvalidRepository(format!("Failed to open transaction: {e}")))?;

    sync_branches_with_context(&tx, repo_id, &context)?;
    tx.commit().map_err(|e| {
        DomainError::InvalidRepository(format!("Failed to commit transaction: {e}"))
    })?;
    Ok(())
}

pub fn sync_commits(conn: &mut Connection, repo_id: i64) -> DomainResult<()> {
    let repo_path = load_repository_path(conn, repo_id)?;
    let repo = Repository::open(&repo_path)
        .map_err(|e| DomainError::InvalidRepository(format!("Failed to open repository: {e}")))?;
    let context = RepoSyncContext::new(&repo_path, repo)?;
    let snapshot = collect_history_snapshot(&context.repo)?;
    let tx = conn
        .transaction()
        .map_err(|e| DomainError::InvalidRepository(format!("Failed to open transaction: {e}")))?;

    sync_commits_with_snapshot(&tx, repo_id, &snapshot)?;
    tx.commit().map_err(|e| {
        DomainError::InvalidRepository(format!("Failed to commit transaction: {e}"))
    })?;
    Ok(())
}

pub fn sync_contributors(conn: &mut Connection, repo_id: i64) -> DomainResult<()> {
    let repo_path = load_repository_path(conn, repo_id)?;
    let repo = Repository::open(&repo_path)
        .map_err(|e| DomainError::InvalidRepository(format!("Failed to open repository: {e}")))?;
    let context = RepoSyncContext::new(&repo_path, repo)?;
    let snapshot = collect_history_snapshot(&context.repo)?;
    let tx = conn
        .transaction()
        .map_err(|e| DomainError::InvalidRepository(format!("Failed to open transaction: {e}")))?;

    sync_contributors_with_snapshot(&tx, repo_id, &snapshot)?;
    tx.commit().map_err(|e| {
        DomainError::InvalidRepository(format!("Failed to commit transaction: {e}"))
    })?;
    Ok(())
}

pub fn sync_repository_files(conn: &mut Connection, repo_id: i64) -> DomainResult<()> {
    let repo_path = load_repository_path(conn, repo_id)?;
    let repo = Repository::open(&repo_path)
        .map_err(|e| DomainError::InvalidRepository(format!("Failed to open repository: {e}")))?;
    let context = RepoSyncContext::new(&repo_path, repo)?;
    let tx = conn
        .transaction()
        .map_err(|e| DomainError::InvalidRepository(format!("Failed to open transaction: {e}")))?;

    sync_repository_files_with_context(&tx, repo_id, &context)?;
    tx.commit().map_err(|e| {
        DomainError::InvalidRepository(format!("Failed to commit transaction: {e}"))
    })?;
    Ok(())
}

pub fn sync_commit_file_stats(conn: &mut Connection, repo_id: i64) -> DomainResult<()> {
    let repo_path = load_repository_path(conn, repo_id)?;
    let repo = Repository::open(&repo_path)
        .map_err(|e| DomainError::InvalidRepository(format!("Failed to open repository: {e}")))?;
    let context = RepoSyncContext::new(&repo_path, repo)?;
    let snapshot = collect_history_snapshot(&context.repo)?;
    let tx = conn
        .transaction()
        .map_err(|e| DomainError::InvalidRepository(format!("Failed to open transaction: {e}")))?;

    sync_commit_file_stats_with_snapshot(&tx, repo_id, &snapshot)?;
    tx.commit().map_err(|e| {
        DomainError::InvalidRepository(format!("Failed to commit transaction: {e}"))
    })?;
    Ok(())
}

pub fn sync_repo_activity(conn: &mut Connection, repo_id: i64) -> DomainResult<()> {
    let repo_path = load_repository_path(conn, repo_id)?;
    let repo = Repository::open(&repo_path)
        .map_err(|e| DomainError::InvalidRepository(format!("Failed to open repository: {e}")))?;
    let context = RepoSyncContext::new(&repo_path, repo)?;
    let snapshot = collect_history_snapshot(&context.repo)?;
    let tx = conn
        .transaction()
        .map_err(|e| DomainError::InvalidRepository(format!("Failed to open transaction: {e}")))?;

    sync_repo_activity_with_snapshot(&tx, repo_id, &snapshot)?;
    tx.commit().map_err(|e| {
        DomainError::InvalidRepository(format!("Failed to commit transaction: {e}"))
    })?;
    Ok(())
}

pub fn sync_working_tree_status(conn: &mut Connection, repo_id: i64) -> DomainResult<()> {
    let repo_path = load_repository_path(conn, repo_id)?;
    let repo = Repository::open(&repo_path)
        .map_err(|e| DomainError::InvalidRepository(format!("Failed to open repository: {e}")))?;
    let context = RepoSyncContext::new(&repo_path, repo)?;
    let tx = conn
        .transaction()
        .map_err(|e| DomainError::InvalidRepository(format!("Failed to open transaction: {e}")))?;

    sync_working_tree_status_with_context(&tx, repo_id, &context)?;
    tx.commit().map_err(|e| {
        DomainError::InvalidRepository(format!("Failed to commit transaction: {e}"))
    })?;
    Ok(())
}

pub fn sync_repository_health(conn: &mut Connection, repo_id: i64) -> DomainResult<()> {
    let tx = conn
        .transaction()
        .map_err(|e| DomainError::InvalidRepository(format!("Failed to open transaction: {e}")))?;
    upsert_repository_health_snapshot(&tx, repo_id)?;
    tx.commit().map_err(|e| {
        DomainError::InvalidRepository(format!("Failed to commit transaction: {e}"))
    })?;
    Ok(())
}

pub fn sync_file_hotspots(conn: &mut Connection, repo_id: i64) -> DomainResult<()> {
    let repo_path = load_repository_path(conn, repo_id)?;
    let repo = Repository::open(&repo_path)
        .map_err(|e| DomainError::InvalidRepository(format!("Failed to open repository: {e}")))?;
    let context = RepoSyncContext::new(&repo_path, repo)?;
    let snapshot = collect_history_snapshot(&context.repo)?;
    let tx = conn
        .transaction()
        .map_err(|e| DomainError::InvalidRepository(format!("Failed to open transaction: {e}")))?;

    sync_file_hotspots_with_snapshot(&tx, repo_id, &snapshot)?;
    tx.commit().map_err(|e| {
        DomainError::InvalidRepository(format!("Failed to commit transaction: {e}"))
    })?;
    Ok(())
}

fn upsert_repository_health_snapshot(conn: &Connection, repo_id: i64) -> DomainResult<()> {
    let metrics = calculate_repository_metrics(conn, repo_id)?;
    repository_health::upsert_repository_health(
        conn,
        repo_id,
        metrics.health_score.value() as f64,
        0,
        0,
        "healthy",
    )
    .map_err(|e| {
        DomainError::InvalidRepository(format!("Failed to upsert repository health: {e}"))
    })?;
    Ok(())
}

fn update_indexing_progress(
    conn: &Connection,
    job_id: i64,
    completed_steps: usize,
    total_steps: usize,
) -> DomainResult<()> {
    let progress = if total_steps == 0 {
        100
    } else {
        ((completed_steps * 100) / total_steps) as i32
    };

    indexing_jobs::update_indexing_job_progress(conn, job_id, progress, completed_steps as i32)
        .map_err(|e| {
            DomainError::InvalidRepository(format!("Failed to update indexing progress: {e}"))
        })
}

fn load_repository_path(conn: &Connection, repo_id: i64) -> DomainResult<String> {
    match repositories::get_repository_path(conn, repo_id) {
        Ok(Some(path)) => Ok(path),
        Ok(None) => Err(DomainError::InvalidRepository(
            "Repository Not Found".into(),
        )),
        Err(error) => Err(DomainError::InvalidRepository(format!(
            "Failed to load repo path: {error}"
        ))),
    }
}

struct RepoSyncContext {
    repo_path: String,
    repo: Repository,
    default_branch: Option<String>,
    head_branch: Option<String>,
    head_commit_hash: Option<String>,
    head_commit_at: Option<String>,
}

impl RepoSyncContext {
    fn new(repo_path: &str, repo: Repository) -> DomainResult<Self> {
        let default_branch = match repo.find_reference("HEAD") {
            Ok(head) => head
                .symbolic_target()
                .ok()
                .flatten()
                .map(normalize_branch_reference),
            Err(_) => None,
        };

        let head_branch = match repo.head() {
            Ok(head) => head.shorthand().ok().map(|branch| branch.to_string()),
            Err(_) => None,
        };

        let (head_commit_hash, head_commit_at) =
            match repo.head().and_then(|head| head.peel_to_commit()) {
                Ok(commit) => {
                    let committed_at = rfc3339_from_seconds(commit.time().seconds());
                    (Some(commit.id().to_string()), Some(committed_at))
                }
                Err(_) => (None, None),
            };

        Ok(Self {
            repo_path: repo_path.to_string(),
            repo,
            default_branch,
            head_branch,
            head_commit_hash,
            head_commit_at,
        })
    }
}

#[derive(Clone)]
struct CommitRecord {
    hash: String,
    author_name: String,
    author_email: String,
    committer_name: String,
    committer_email: String,
    subject: String,
    body: String,
    parent_count: i64,
    committed_at: String,
    parent_hashes: Vec<String>,
}

#[derive(Clone)]
struct CommitFileStatRecord {
    commit_hash: String,
    file_path: String,
    change_type: String,
    additions: i32,
    deletions: i32,
}

#[derive(Clone)]
struct ContributorAggregate {
    author_name: String,
    author_email: Option<String>,
    commit_count: i32,
    additions: i32,
    deletions: i32,
    active_days: HashSet<String>,
    last_commit_at: Option<String>,
}

#[derive(Clone)]
struct ActivityAggregate {
    commit_count: i32,
    additions: i32,
    deletions: i32,
    files_changed: i32,
}

#[derive(Clone)]
struct FileHotspotAggregate {
    touch_count: i32,
    churn_score: f64,
    last_touched_at: Option<String>,
}

struct RepoHistorySnapshot {
    commits: Vec<CommitRecord>,
    file_stats: Vec<CommitFileStatRecord>,
    contributors: HashMap<String, ContributorAggregate>,
    activity_daily: BTreeMap<String, ActivityAggregate>,
    file_hotspots: HashMap<String, FileHotspotAggregate>,
}

fn collect_history_snapshot(repo: &Repository) -> DomainResult<RepoHistorySnapshot> {
    let mut revwalk = repo
        .revwalk()
        .map_err(|e| DomainError::InvalidRepository(format!("Failed to create revwalk: {e}")))?;
    revwalk.push_head().map_err(|e| {
        DomainError::InvalidRepository(format!("Failed to push HEAD to revwalk: {e}"))
    })?;
    revwalk
        .set_sorting(Sort::TOPOLOGICAL | Sort::TIME)
        .map_err(|e| DomainError::InvalidRepository(format!("Failed to sort revwalk: {e}")))?;

    let mut commits = Vec::new();
    let mut file_stats = Vec::new();
    let mut contributors_map: HashMap<String, ContributorAggregate> = HashMap::new();
    let mut activity_daily: BTreeMap<String, ActivityAggregate> = BTreeMap::new();
    let mut file_hotspots: HashMap<String, FileHotspotAggregate> = HashMap::new();

    for oid_result in revwalk {
        let oid = oid_result.map_err(|e| {
            DomainError::InvalidRepository(format!("Failed to read commit id: {e}"))
        })?;
        let commit = repo.find_commit(oid).map_err(|e| {
            DomainError::InvalidRepository(format!("Failed to load commit {oid}: {e}"))
        })?;

        let author_name = commit.author().name().unwrap_or("Unknown").to_string();
        let author_email = match commit.author().email() {
            Ok(value) => Some(value.to_string()),
            _ => None,
        };
        let committer_name = commit.committer().name().unwrap_or("Unknown").to_string();
        let committer_email = match commit.committer().email() {
            Ok(value) => Some(value.to_string()),
            _ => None,
        };
        let committed_at = rfc3339_from_seconds(commit.time().seconds());
        let parent_hashes = commit
            .parent_ids()
            .map(|parent_oid| parent_oid.to_string())
            .collect::<Vec<_>>();
        let file_changes = collect_commit_file_stats(repo, &commit)?;
        let additions = file_changes
            .iter()
            .map(|entry| entry.additions)
            .sum::<i32>();
        let deletions = file_changes
            .iter()
            .map(|entry| entry.deletions)
            .sum::<i32>();

        commits.push(CommitRecord {
            hash: commit.id().to_string(),
            author_name: author_name.clone(),
            author_email: author_email.clone().unwrap_or_default(),
            committer_name,
            committer_email: committer_email.clone().unwrap_or_default(),
            subject: match commit.summary() {
                Ok(value) => value.unwrap_or("No subject").to_string(),
                _ => "No subject".to_string(),
            },
            body: match commit.message() {
                Ok(value) => value.to_string(),
                _ => String::new(),
            },
            parent_count: commit.parent_count() as i64,
            committed_at: committed_at.clone(),
            parent_hashes,
        });

        let contributor_key = author_email
            .clone()
            .unwrap_or_else(|| format!("name:{author_name}"));
        let contributor =
            contributors_map
                .entry(contributor_key)
                .or_insert_with(|| ContributorAggregate {
                    author_name: author_name.clone(),
                    author_email: author_email.clone(),
                    commit_count: 0,
                    additions: 0,
                    deletions: 0,
                    active_days: HashSet::new(),
                    last_commit_at: None,
                });
        contributor.commit_count += 1;
        contributor.additions += additions;
        contributor.deletions += deletions;
        contributor
            .active_days
            .insert(committed_at_date(&committed_at));
        contributor.last_commit_at = Some(committed_at.clone());

        let day = committed_at_date(&committed_at);
        let activity = activity_daily.entry(day).or_insert(ActivityAggregate {
            commit_count: 0,
            additions: 0,
            deletions: 0,
            files_changed: 0,
        });
        activity.commit_count += 1;
        activity.additions += additions;
        activity.deletions += deletions;
        activity.files_changed += file_changes.len() as i32;

        for file_change in file_changes {
            let total_changes = file_change.additions + file_change.deletions;
            file_stats.push(file_change.clone());

            let hotspot = file_hotspots
                .entry(file_change.file_path.clone())
                .or_insert_with(|| FileHotspotAggregate {
                    touch_count: 0,
                    churn_score: 0.0,
                    last_touched_at: None,
                });
            hotspot.touch_count += 1;
            hotspot.churn_score += total_changes as f64;
            hotspot.last_touched_at = Some(committed_at.clone());
        }
    }

    Ok(RepoHistorySnapshot {
        commits,
        file_stats,
        contributors: contributors_map,
        activity_daily,
        file_hotspots,
    })
}

fn collect_commit_file_stats(
    repo: &Repository,
    commit: &git2::Commit<'_>,
) -> DomainResult<Vec<CommitFileStatRecord>> {
    let current_tree = commit
        .tree()
        .map_err(|e| DomainError::InvalidRepository(format!("Failed to read commit tree: {e}")))?;

    let parent_tree = if commit.parent_count() == 0 {
        None
    } else {
        Some(
            commit
                .parent(0)
                .and_then(|parent| parent.tree())
                .map_err(|e| {
                    DomainError::InvalidRepository(format!("Failed to read parent tree: {e}"))
                })?,
        )
    };

    let mut diff = repo
        .diff_tree_to_tree(parent_tree.as_ref(), Some(&current_tree), None)
        .map_err(|e| DomainError::InvalidRepository(format!("Failed to create diff: {e}")))?;
    diff.find_similar(None).map_err(|e| {
        DomainError::InvalidRepository(format!("Failed to detect renamed files: {e}"))
    })?;

    let mut file_stats = Vec::with_capacity(diff.deltas().len());
    let mut file_indices = HashMap::new();

    for delta in diff.deltas() {
        let Some(file_path) = delta
            .new_file()
            .path()
            .or(delta.old_file().path())
            .map(|path| path.to_string_lossy().into_owned())
        else {
            continue;
        };

        file_indices.insert(delta_key(&delta), file_stats.len());
        file_stats.push(CommitFileStatRecord {
            commit_hash: commit.id().to_string(),
            file_path,
            change_type: git::change_type(delta.status()).as_str().to_string(),
            additions: 0,
            deletions: 0,
        });
    }

    diff.print(DiffFormat::Patch, |delta, hunk, line| {
        let Some(file_index) = file_indices.get(&delta_key(&delta)).copied() else {
            return true;
        };
        let Some(hunk) = hunk else {
            return true;
        };
        let Some(line_type) = diff_line_type(line.origin()) else {
            return true;
        };

        let _ = hunk;

        match line_type {
            git::DiffLineType::Added => file_stats[file_index].additions += 1,
            git::DiffLineType::Removed => file_stats[file_index].deletions += 1,
            git::DiffLineType::Context => {}
        }

        true
    })
    .map_err(|e| DomainError::InvalidRepository(format!("Failed to build diff stats: {e}")))?;

    Ok(file_stats)
}

fn sync_branches_with_context(
    conn: &Connection,
    repo_id: i64,
    context: &RepoSyncContext,
) -> DomainResult<usize> {
    let mut branches_seen = HashSet::new();
    let default_branch = context.default_branch.as_deref();
    let branch_iter = context
        .repo
        .branches(Some(BranchType::Local))
        .map_err(|e| DomainError::InvalidBranch(format!("Failed to enumerate branches: {e}")))?;

    for branch_result in branch_iter {
        let (branch, _) = branch_result
            .map_err(|e| DomainError::InvalidBranch(format!("Failed to read branch: {e}")))?;
        let branch_name = branch
            .name()
            .map_err(|e| DomainError::InvalidBranch(format!("Failed to read branch name: {e}")))?
            .ok_or_else(|| DomainError::InvalidBranch("Branch name missing".into()))?
            .to_string();

        branches_seen.insert(branch_name.clone());
        let is_default = default_branch == Some(branch_name.as_str());
        let last_commit = last_commit_info_by_branch_from_repo(&context.repo, &branch_name)?;
        let ahead_behind_from_default = if is_default {
            find_ahead_behind_given_vs_default_from_repo(
                &context.repo,
                default_branch.unwrap_or(""),
                &branch_name,
            )
            .unwrap_or((0, 0))
        } else {
            (0, 0)
        };
        let ahead_behind_from_remote =
            find_ahead_behind_local_vs_remote_from_repo(&context.repo, &branch_name)
                .unwrap_or((0, 0));

        branches::upsert_branch(
            conn,
            repo_id,
            &branch_name,
            branch.is_head(),
            is_default,
            Some(last_commit.hash.as_str()),
            Some(last_commit.committed_at.as_str()),
            ahead_behind_from_default.0,
            ahead_behind_from_default.1,
            ahead_behind_from_remote.0,
            ahead_behind_from_remote.1,
        )
        .map_err(|e| DomainError::InvalidBranch(format!("Failed to upsert branch: {e}")))?;
    }

    if let Some(existing) = branches::get_all_branches(conn, repo_id)
        .map_err(|e| DomainError::InvalidBranch(format!("Failed to load existing branches: {e}")))?
    {
        for branch in existing {
            if !branches_seen.contains(&branch.name) {
                let _ = conn.execute(
                    "DELETE FROM branches WHERE repo_id = ?1 AND name = ?2",
                    rusqlite::params![repo_id, branch.name],
                );
            }
        }
    }

    Ok(branches_seen.len())
}

fn sync_commits_with_snapshot(
    conn: &Connection,
    repo_id: i64,
    snapshot: &RepoHistorySnapshot,
) -> DomainResult<usize> {
    for commit in &snapshot.commits {
        commits::upsert_commit(
            conn,
            repo_id,
            &commit.hash,
            &commit.author_name,
            &commit.author_email,
            &commit.committer_name,
            &commit.committer_email,
            &commit.subject,
            &commit.body,
            commit.parent_count,
            &commit.committed_at,
            &commit.parent_hashes,
        )
        .map_err(|e| DomainError::InvalidRepository(format!("Failed to upsert commit: {e}")))?;
    }

    Ok(snapshot.commits.len())
}

fn sync_contributors_with_snapshot(
    conn: &Connection,
    repo_id: i64,
    snapshot: &RepoHistorySnapshot,
) -> DomainResult<usize> {
    conn.execute(
        "DELETE FROM contributors WHERE repo_id = ?1",
        rusqlite::params![repo_id],
    )
    .map_err(|e| DomainError::InvalidRepository(format!("Failed to clear contributors: {e}")))?;

    for contributor in snapshot.contributors.values() {
        contributors::upsert_contributor(
            conn,
            repo_id,
            &contributor.author_name,
            contributor.author_email.as_deref(),
            contributor.commit_count,
            contributor.additions,
            contributor.deletions,
            i32::try_from(contributor.active_days.len()).unwrap_or(i32::MAX),
            contributor.last_commit_at.as_deref(),
        )
        .map_err(|e| {
            DomainError::InvalidRepository(format!("Failed to upsert contributor: {e}"))
        })?;
    }

    Ok(snapshot.contributors.len())
}

fn sync_repository_files_with_context(
    conn: &Connection,
    repo_id: i64,
    context: &RepoSyncContext,
) -> DomainResult<usize> {
    conn.execute(
        "DELETE FROM repository_files WHERE repo_id = ?1",
        rusqlite::params![repo_id],
    )
    .map_err(|e| {
        DomainError::InvalidRepository(format!("Failed to clear repository files: {e}"))
    })?;

    let discovered_files = collect_repository_files(&context.repo_path)?;
    for file in &discovered_files {
        repository_files::upsert_repository_file(
            conn,
            repo_id,
            &file.file_path,
            &file.file_name,
            file.extension.as_deref(),
            file.size_bytes,
            file.is_binary,
            file.last_modified_at.as_deref(),
        )
        .map_err(|e| {
            DomainError::InvalidRepository(format!("Failed to upsert repository file: {e}"))
        })?;
    }

    Ok(discovered_files.len())
}

fn sync_commit_file_stats_with_snapshot(
    conn: &Connection,
    repo_id: i64,
    snapshot: &RepoHistorySnapshot,
) -> DomainResult<usize> {
    for stat in &snapshot.file_stats {
        file_stats::upsert_commit_file_stat(
            conn,
            repo_id,
            &stat.commit_hash,
            &stat.file_path,
            &stat.change_type,
            stat.additions,
            stat.deletions,
        )
        .map_err(|e| DomainError::InvalidRepository(format!("Failed to upsert file stat: {e}")))?;
    }

    Ok(snapshot.file_stats.len())
}

fn sync_repo_activity_with_snapshot(
    conn: &Connection,
    repo_id: i64,
    snapshot: &RepoHistorySnapshot,
) -> DomainResult<usize> {
    conn.execute(
        "DELETE FROM repo_activity_daily WHERE repo_id = ?1",
        rusqlite::params![repo_id],
    )
    .map_err(|e| DomainError::InvalidRepository(format!("Failed to clear repo activity: {e}")))?;

    for (activity_date, activity) in &snapshot.activity_daily {
        analytics::insert_repo_activity_daily(
            conn,
            repo_id,
            activity_date,
            activity.commit_count,
            activity.additions,
            activity.deletions,
            activity.files_changed,
        )
        .map_err(|e| {
            DomainError::InvalidRepository(format!("Failed to upsert repo activity: {e}"))
        })?;
    }

    Ok(snapshot.activity_daily.len())
}

fn sync_working_tree_status_with_context(
    conn: &Connection,
    repo_id: i64,
    context: &RepoSyncContext,
) -> DomainResult<()> {
    let status = git::status::get_working_tree_statuses(&context.repo_path).map_err(|e| {
        DomainError::InvalidRepository(format!("Failed to load working tree status: {e}"))
    })?;

    working_tree::insert_working_tree(
        conn,
        repo_id,
        i32::try_from(status.modified.len()).unwrap_or(i32::MAX),
        0,
        i32::try_from(status.added.len()).unwrap_or(i32::MAX),
        i32::try_from(status.deleted.len()).unwrap_or(i32::MAX),
        i32::try_from(status.renamed.len()).unwrap_or(i32::MAX),
    )
    .map_err(|e| {
        DomainError::InvalidRepository(format!("Failed to insert working tree status: {e}"))
    })?;

    Ok(())
}

fn sync_file_hotspots_with_snapshot(
    conn: &Connection,
    repo_id: i64,
    snapshot: &RepoHistorySnapshot,
) -> DomainResult<usize> {
    conn.execute(
        "DELETE FROM file_hotspots WHERE repo_id = ?1",
        rusqlite::params![repo_id],
    )
    .map_err(|e| DomainError::InvalidRepository(format!("Failed to clear file hotspots: {e}")))?;

    for (file_path, hotspot) in &snapshot.file_hotspots {
        let churn_score = hotspot.churn_score;
        let hotspot_score = (hotspot.touch_count as f64 * 0.5) + (churn_score * 0.5);
        file_stats::upsert_file_hotspot(
            conn,
            repo_id,
            file_path,
            hotspot.touch_count,
            churn_score,
            hotspot_score,
            hotspot.last_touched_at.as_deref().unwrap_or(""),
        )
        .map_err(|e| {
            DomainError::InvalidRepository(format!("Failed to upsert file hotspot: {e}"))
        })?;
    }

    Ok(snapshot.file_hotspots.len())
}

fn collect_repository_files(repo_path: &str) -> DomainResult<Vec<DiscoveredRepositoryFile>> {
    let mut files = Vec::new();

    for entry in WalkDir::new(repo_path)
        .into_iter()
        .filter_entry(|entry| !should_skip_path(entry.path()))
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        let relative = match path.strip_prefix(repo_path) {
            Ok(relative) => relative,
            Err(_) => continue,
        };

        let file_name = path
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or_default()
            .to_string();
        let extension = path
            .extension()
            .and_then(|value| value.to_str())
            .map(|value| value.to_string());
        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(_) => continue,
        };
        let size_bytes = i64::try_from(metadata.len()).unwrap_or(i64::MAX);
        let last_modified_at = metadata
            .modified()
            .ok()
            .map(|modified| DateTime::<Utc>::from(modified).to_rfc3339());

        files.push(DiscoveredRepositoryFile {
            file_path: relative.to_string_lossy().replace('\\', "/"),
            file_name,
            extension,
            size_bytes: Some(size_bytes),
            is_binary: is_binary_extension(path),
            last_modified_at,
        });
    }

    Ok(files)
}

fn should_skip_path(path: &Path) -> bool {
    path.components().any(|component| {
        matches!(
            component.as_os_str().to_str(),
            Some(".git") | Some("target") | Some("node_modules") | Some("dist") | Some("build")
        )
    })
}

fn is_binary_extension(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|value| value.to_str()),
        Some("png")
            | Some("jpg")
            | Some("jpeg")
            | Some("webp")
            | Some("gif")
            | Some("bmp")
            | Some("ico")
            | Some("pdf")
            | Some("zip")
            | Some("gz")
            | Some("jar")
            | Some("class")
    )
}

fn rfc3339_from_seconds(seconds: i64) -> String {
    Utc.timestamp_opt(seconds, 0)
        .single()
        .unwrap_or_else(Utc::now)
        .to_rfc3339()
}

fn committed_at_date(timestamp: &str) -> String {
    DateTime::parse_from_rfc3339(timestamp)
        .map(|value| value.date_naive().to_string())
        .unwrap_or_else(|_| Utc::now().date_naive().to_string())
}

fn normalize_branch_reference(reference: &str) -> String {
    reference
        .strip_prefix("refs/heads/")
        .unwrap_or(reference)
        .to_string()
}

fn last_commit_info_by_branch_from_repo(
    repo: &Repository,
    branch_name: &str,
) -> DomainResult<git::LastCommit> {
    let branch = repo
        .find_branch(branch_name, BranchType::Local)
        .or_else(|_| repo.find_branch(branch_name, BranchType::Remote))
        .map_err(|e| {
            DomainError::InvalidBranch(format!("Failed to find branch '{branch_name}': {e}"))
        })?;

    let reference = branch.into_reference();
    let commit_hash = reference.target().ok_or_else(|| {
        DomainError::InvalidBranch(format!(
            "Branch '{branch_name}' does not point to a valid commit"
        ))
    })?;

    let commit = repo.find_commit(commit_hash).map_err(|e| {
        DomainError::InvalidBranch(format!(
            "Failed to load commit for branch '{branch_name}': {e}"
        ))
    })?;

    Ok(git::LastCommit {
        hash: commit.id().to_string(),
        committed_at: rfc3339_from_seconds(commit.time().seconds()),
    })
}

fn find_ahead_behind_local_vs_remote_from_repo(
    repo: &Repository,
    branch: &str,
) -> Result<(i32, i32), String> {
    let remote_ref = repo
        .find_branch(branch, BranchType::Remote)
        .map_err(|error| error.to_string())?
        .into_reference();
    let local_ref = repo
        .find_branch(branch, BranchType::Local)
        .map(|branch| branch.into_reference())
        .unwrap_or_else(|_| remote_ref.clone());

    let (ahead, behind) = repo
        .graph_ahead_behind(
            local_ref
                .target()
                .ok_or_else(|| format!("Branch '{branch}' has no target"))?,
            remote_ref
                .target()
                .ok_or_else(|| format!("Remote branch '{branch}' has no target"))?,
        )
        .map_err(|e| format!("Failed to calculate ahead/behind: {e}"))?;

    Ok((ahead as i32, behind as i32))
}

fn find_ahead_behind_given_vs_default_from_repo(
    repo: &Repository,
    default_branch: &str,
    given_branch: &str,
) -> Result<(i32, i32), String> {
    if default_branch == given_branch {
        return Ok((0, 0));
    }

    let given_ref = repo
        .find_branch(given_branch, BranchType::Local)
        .map_err(|error| error.to_string())?
        .into_reference();
    let default_ref = repo
        .find_branch(default_branch, BranchType::Local)
        .map_err(|error| error.to_string())?
        .into_reference();

    let (ahead, behind) = repo
        .graph_ahead_behind(
            given_ref.target().ok_or_else(|| {
                format!("Branch '{given_branch}' does not point to a valid commit")
            })?,
            default_ref.target().ok_or_else(|| {
                format!("Branch '{default_branch}' does not point to a valid commit")
            })?,
        )
        .map_err(|e| format!("Failed to calculate ahead/behind: {e}"))?;

    Ok((ahead as i32, behind as i32))
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

fn diff_line_type(origin: char) -> Option<git::DiffLineType> {
    match origin {
        ' ' => Some(git::DiffLineType::Context),
        '+' => Some(git::DiffLineType::Added),
        '-' => Some(git::DiffLineType::Removed),
        _ => None,
    }
}

#[derive(Clone)]
struct DiscoveredRepositoryFile {
    file_path: String,
    file_name: String,
    extension: Option<String>,
    size_bytes: Option<i64>,
    is_binary: bool,
    last_modified_at: Option<String>,
}
