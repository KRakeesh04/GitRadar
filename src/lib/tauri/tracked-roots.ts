import { tauri } from '#/lib/tauri/tauri';

export interface TrackedRoot {
  id: number;
  path: string;
  enabled: boolean;
  updatedAt: string;
}

export interface Repository {
  id: number;
  rootId: number;
  name: string;
  path: string;
  headBranch: string | null;
  isDirty: boolean;
  updatedAt: string;
}

interface TrackedRootResponse {
  id: number;
  path: string;
  is_enabled: boolean;
  updated_at: string;
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

function toTrackedRoot(root: TrackedRootResponse): TrackedRoot {
  return {
    id: root.id,
    path: root.path,
    enabled: root.is_enabled,
    updatedAt: root.updated_at,
  };
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

export async function getTrackedRoots(): Promise<TrackedRoot[]> {
  const roots = await tauri<TrackedRootResponse[]>('get_all_tracked_root_paths');
  return roots.map(toTrackedRoot);
}

export async function getRepositories(): Promise<Repository[]> {
  const repositories = await tauri<RepositoryResponse[]>('get_all_repositories');
  return repositories.map(toRepository);
}

export function rescanTrackedRoots(): Promise<void> {
  return tauri<void>('discover_repositories');
}

export function addTrackedRoot(path: string): Promise<number> {
  return tauri<number>('add_tracked_root_path', { path });
}

export function setTrackedRootEnabled(path: string, enabled: boolean): Promise<boolean> {
  return tauri<boolean>('set_tracked_root_enabled', { path, enabled });
}

export function deleteTrackedRoot(id: number): Promise<boolean> {
  return tauri<boolean>('delete_tracked_root_path', { rootId: id });
}
