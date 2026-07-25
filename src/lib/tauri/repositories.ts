import { tauri } from "./tauri";

export interface Repository {
  id: number;
  rootId: number;
  name: string;
  path: string;
  headBranch: string | null;
  isDirty: boolean;
  updatedAt: string;
}

interface RepositoryResponse {
  id: number;
  root_id: number;
  name: string;
  path: string;
  head_branch: string | null;
  is_dirty: boolean;
  updated_at: string;
}

export interface RepositoryInfo {
  id: number;
  rootId: number;
  updatedAt: string;
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

interface RepositoryInfoResponse {
  id: number;
  root_id: number;
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

function toRepository(repository: RepositoryResponse): Repository {
  return {
    id: repository.id,
    rootId: repository.root_id,
    name: repository.name,
    path: repository.path,
    headBranch: repository.head_branch,
    isDirty: repository.is_dirty,
    updatedAt: repository.updated_at,
  };
}

function toRepositoryInfo(repository: RepositoryInfoResponse): RepositoryInfo {
  return {
    id: repository.id,
    rootId: repository.root_id,
    updatedAt: repository.updated_at,
    name: repository.name,
    path: repository.path,
    git_dir: repository.git_dir,
    health_score: repository.health_score,
    activity_level: repository.activity_level,
    default_branch: repository.default_branch,
    head_branch: repository.head_branch,
    remote_url: repository.remote_url,
    is_dirty: repository.is_dirty,
    total_commits: repository.total_commits,
    unique_contributors: repository.unique_contributors,
  };
}

export interface Branch {
  id: number,
  repoId: number,
  name: string,
  branchType: string,
  isHead: boolean,
  isDefault: boolean,
  lastCommitHash: string | null,
  aheadCountFromRemote: number,
  behindCountFromRemote: number,
  aheadCountFromDefault: number,
  behindCountFromDefault: number,
  status: string,
  shouldMerge: boolean,
  isStale: boolean,
  importance: string,
}

interface BranchResponse {
  id: number,
  repo_id: number,
  name: string,
  branch_type: string,
  is_head: boolean,
  is_default: boolean,
  last_commit_hash: string | null,
  ahead_count_from_remote: number,
  behind_count_from_remote: number,
  ahead_count_from_default: number,
  behind_count_from_default: number,
  status: string,
  should_merge: boolean,
  is_stale: boolean,
  importance: string,
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

export async function getRepositories(): Promise<Repository[]> {
  const repositories = await tauri<RepositoryResponse[]>('get_all_repositories');
  return repositories.map(toRepository);
}

export async function getRepositoryInfoById(id: number): Promise<RepositoryInfo | null> {
  const repositoryInfo = await tauri<RepositoryInfoResponse | null>('get_repository_info', { repo_id: id });
  return repositoryInfo ? toRepositoryInfo(repositoryInfo) : null;
}

export async function getBranchesByRepoId(repoId: number): Promise<Branch[]> {
  const branches = await tauri<BranchResponse[]>('get_repository_branches', { repo_id: repoId });
  return branches.map(toBranch);
}

export async function getBranchByRepoIdAndName(repoId: number, branchName: string): Promise<Branch | null> {
  const branch = await tauri<BranchResponse | null>('get_branch_info', { repo_id: repoId, name: branchName });
  return branch ? toBranch(branch) : null;
}
