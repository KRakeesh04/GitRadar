import { tauri } from "./tauri";

export interface Commit {
  id: number,
  hash: string,
  shortHash: string,
  authorName: string,
  authorEmail: string,
  committerName: string,
  committerEmail: string,
  subject: string,
  body: string | null,
  parentCount: number,
  committedAt: string,
  isSignificant: boolean,
  isMergeCommit: boolean,
  isRootCommit: boolean,
}

interface CommitResponse {
  id: number,
  hash: string,
  short_hash: string,
  author_name: string,
  author_email: string,
  committer_name: string,
  committer_email: string,
  subject: string,
  body: string | null,
  parent_count: number,
  committed_at: string,
  is_significant: boolean,
  is_merge_commit: boolean,
  is_root_commit: boolean,
}

export interface CommitGraphNode {
  hash: string,
  branchName: string | null,
  branchNames: string[],
  authorName: string,
  authorEmail: string,
  subject: string,
  committedAt: string,
  additions: number,
  deletions: number,
  totalChangedFilesCount: number,
  parentHashes: string[],
}

interface CommitGraphNodeResponse {
  hash: string,
  branch_name: string | null,
  branch_names: string[],
  author_name: string,
  author_email: string,
  subject: string,
  committed_at: string,
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

export async function getCommitByHash(repoId: number, hash: string): Promise<Commit> {
  const commitResponse = await tauri<CommitResponse>('get_commit_by_hash', { repoId, hash });
  return toCommit(commitResponse);
}

export async function getCommitsByRepoId(repoId: number, limit: number, offset: number): Promise<Commit[]> {
  const commitsResponse = await tauri<CommitResponse[]>('get_commits', { repoId, count: limit, offset });
  return commitsResponse.map(toCommit);
}

export async function getCommitGraphByRepoId(repoId: number, limit: number, offset: number): Promise<CommitGraphNode[]> {
  const commitGraphResponse = await tauri<CommitGraphNodeResponse[]>('get_commit_graph', { repoId, count: limit, offset });
  return commitGraphResponse.map(toCommitGraphNode);
}

export async function getCommitDiffByHash(repoId: number, hash: string): Promise<any> {
  const diff = await tauri<any>('get_commit_diff', { repoId, commitHash: hash });
  return diff;
}

export async function getCommitInlineDiff(repoId: number, hash: string): Promise<any> {
  const inlineDiff = await tauri<any>('get_commit_inline_diff', { repoId, commitHash: hash });
  return inlineDiff;
}
