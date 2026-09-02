import { tauri } from "./tauri";

export interface RepositoryInfo {
  id: number;
  rootIds: number[];
  rootId: number | null;
  isEnabled: boolean;
  createdAt: string;
  updatedAt: string;
  name: string;
  path: string;
  gitDir: string;
  healthScore: number;
  activityLevel: string;
  defaultBranch: string | null;
  headBranch: string | null;
  remoteUrl: string | null;
  isDirty: boolean;
  totalCommits: number;
  uniqueContributors: number;
}

export type Repository = RepositoryInfo;

interface RepositoryInfoResponse {
  id: number;
  root_ids?: number[];
  root_id?: number | null;
  is_enabled: boolean;
  created_at: string;
  updated_at: string;
  name: string;
  path: string;
  git_dir: string;
  health_score: number;
  activity_level: string;
  default_branch: string | null;
  head_branch: string | null;
  remote_url: string | null;
  is_dirty: boolean;
  total_commits: number;
  unique_contributors: number;
}

export interface PaginatedRepositoriesResponse {
  items: RepositoryInfoResponse[];
  next_cursor: number | null;
  has_more: boolean;
  total_count: number;
}

export interface PaginatedRepositories {
  items: RepositoryInfo[];
  nextCursor: number | null;
  hasMore: boolean;
  totalCount: number;
}

function toRepositoryInfo(repository: RepositoryInfoResponse): RepositoryInfo {
  return {
    id: repository.id,
    rootIds: repository.root_ids ?? (repository.root_id != null ? [repository.root_id] : []),
    rootId: repository.root_id ?? (repository.root_ids?.[0] ?? null),
    isEnabled: repository.is_enabled,
    createdAt: repository.created_at,
    updatedAt: repository.updated_at,
    name: repository.name,
    path: repository.path,
    gitDir: repository.git_dir,
    healthScore: repository.health_score,
    activityLevel: repository.activity_level,
    defaultBranch: repository.default_branch,
    headBranch: repository.head_branch,
    remoteUrl: repository.remote_url,
    isDirty: repository.is_dirty,
    totalCommits: repository.total_commits,
    uniqueContributors: repository.unique_contributors,
  };
}

export interface Branch {
  id: number;
  repoId: number;
  name: string;
  branchType: string;
  isHead: boolean;
  isDefault: boolean;
  lastCommitHash: string | null;
  aheadCountFromRemote: number;
  behindCountFromRemote: number;
  aheadCountFromDefault: number;
  behindCountFromDefault: number;
  status: string;
  shouldMerge: boolean;
  isStale: boolean;
  importance: string;
}

interface BranchResponse {
  id: number;
  repo_id: number;
  name: string;
  branch_type: string;
  is_head: boolean;
  is_default: boolean;
  last_commit_hash: string | null;
  ahead_count_from_remote: number;
  behind_count_from_remote: number;
  ahead_count_from_default: number;
  behind_count_from_default: number;
  status: string;
  should_merge: boolean;
  is_stale: boolean;
  importance: string;
}

function toBranch(branch: BranchResponse): Branch {
  return {
    id: branch.id,
    repoId: branch.repo_id,
    name: branch.name,
    branchType: branch.branch_type,
    isHead: branch.is_head,
    isDefault: branch.is_default,
    lastCommitHash: branch.last_commit_hash,
    aheadCountFromRemote: branch.ahead_count_from_remote,
    behindCountFromRemote: branch.behind_count_from_remote,
    aheadCountFromDefault: branch.ahead_count_from_default,
    behindCountFromDefault: branch.behind_count_from_default,
    status: branch.status,
    shouldMerge: branch.should_merge,
    isStale: branch.is_stale,
    importance: branch.importance,
  };
}

export async function getRepositories(): Promise<RepositoryInfo[]> {
  const repositories = await tauri<RepositoryInfoResponse[]>('get_all_repositories');
  return repositories.map(toRepositoryInfo);
}

export async function getRepositoriesByRootId(rootId: number): Promise<RepositoryInfo[]> {
  const repositories = await tauri<RepositoryInfoResponse[]>('get_repositories_by_root_id', { rootId });
  return repositories.map(toRepositoryInfo);
}

export async function getPaginatedRepositories(params: {
  search?: string;
  filter?: string;
  limit?: number;
  cursor?: number | null;
}): Promise<PaginatedRepositories> {
  const response = await tauri<PaginatedRepositoriesResponse>('get_paginated_repositories', {
    search: params.search || null,
    filter: params.filter || null,
    limit: params.limit || 20,
    cursor: params.cursor || null,
  });

  return {
    items: response.items.map(toRepositoryInfo),
    nextCursor: response.next_cursor,
    hasMore: response.has_more,
    totalCount: response.total_count,
  };
}

export function setRepositoryEnabled(repoId: number, enabled: boolean): Promise<boolean> {
  return tauri<boolean>('set_repository_enabled', { repoId, enabled });
}

export async function getRepositoryInfoById(id: number): Promise<RepositoryInfo | null> {
  const repositoryInfo = await tauri<RepositoryInfoResponse | null>('get_repository_info', { repoId: id });
  return repositoryInfo ? toRepositoryInfo(repositoryInfo) : null;
}

export async function getBranchesByRepoId(repoId: number): Promise<Branch[]> {
  const branches = await tauri<BranchResponse[]>('get_repository_branches', { repoId });
  return branches.map(toBranch);
}

export async function getBranchByRepoIdAndName(repoId: number, branchName: string): Promise<Branch | null> {
  const branch = await tauri<BranchResponse | null>('get_branch_info', { repoId, name: branchName });
  return branch ? toBranch(branch) : null;
}
