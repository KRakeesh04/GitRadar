import { tauri } from "./tauri";

export interface Commit {
  id: number,
  hash: String,
  shortHash: String,
  authorName: String,
  authorEmail: String,
  committerName: String,
  committerEmail: String,
  subject: String,
  body: String | null,
  parentCount: number,
  committedAt: String,
  isSignificant: boolean,
  isMergeCommit: boolean,
  isRootCommit: boolean,
}

interface CommitResponse {
  id: number,
  hash: String,
  short_hash: String,
  author_name: String,
  author_email: String,
  committer_name: String,
  committer_email: String,
  subject: String,
  body: String | null,
  parent_count: number,
  committed_at: String,
  is_significant: boolean,
  is_merge_commit: boolean,
  is_root_commit: boolean,
}

export interface CommitGraphNode {
  hash: String,
  branchName: string | null,
  branchNames: string[],
  authorName: String,
  authorEmail: String,
  subject: String,
  committedAt: String,
  additions: number,
  deletions: number,
  totalChangedFilesCount: number,
  parentHashes: string[],
}

interface CommitGraphNodeResponse {
  hash: String,
  branch_name: string | null,
  branch_names: string[],
  author_name: String,
  author_email: String,
  subject: String,
  committed_at: String,
  additions: number,
  deletions: number,
  total_changed_files_count: number,
  parent_hashes: string[],
}

function toCommit(commit: CommitResponse): Commit {
  return {
    id: commit.id,
    hash: commit.hash,
    shortHash: commit.short_hash,
    authorName: commit.author_name,
    authorEmail: commit.author_email,
    committerName: commit.committer_name,
    committerEmail: commit.committer_email,
    subject: commit.subject,
    body: commit.body,
    parentCount: commit.parent_count,
    committedAt: commit.committed_at,
    isSignificant: commit.is_significant,
    isMergeCommit: commit.is_merge_commit,
    isRootCommit: commit.is_root_commit,
  };
}

function toCommitGraphNode(node: CommitGraphNodeResponse): CommitGraphNode {
  return {
    hash: node.hash,
    branchName: node.branch_name,
    branchNames: node.branch_names,
    authorName: node.author_name,
    authorEmail: node.author_email,
    subject: node.subject,
    committedAt: node.committed_at,
    additions: node.additions,
    deletions: node.deletions,
    totalChangedFilesCount: node.total_changed_files_count,
    parentHashes: node.parent_hashes,
  };
}

export async function getCommitByHash(repo_id: number, hash: string): Promise<Commit> {
  const commitResponse = await tauri<CommitResponse>('get_commit_by_hash', { repo_id, hash });
  return toCommit(commitResponse);
}

export async function getCommitsByRepoId(repoId: number, limit: number, offset: number): Promise<Commit[]> {
  const commitsResponse = await tauri<CommitResponse[]>('get_commits', { repo_id: repoId, count: limit, offset });
  return commitsResponse.map(toCommit);
}

export async function getCommitGraphByRepoId(repoId: number, limit: number, offset: number): Promise<CommitGraphNode[]> {
  const commitGraphResponse = await tauri<CommitGraphNodeResponse[]>('get_commit_graph', { repo_id: repoId, count: limit, offset });
  return commitGraphResponse.map(toCommitGraphNode);
}

export async function getCommitDiffByHash(repoId: number, hash: string): Promise<any> {
  const diff = await tauri<any>('get_commit_diff', { repo_id: repoId, commit_hash: hash });
  return diff;
}

export async function getCommitInlineDiff(repoId: number, hash: string): Promise<any> {
  const inlineDiff = await tauri<any>('get_commit_inline_diff', { repo_id: repoId, commit_hash: hash });
  return inlineDiff;
}