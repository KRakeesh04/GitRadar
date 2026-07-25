import { tauri } from "./tauri";

export interface TrackedRoot {
  id: number;
  path: string;
  enabled: boolean;
  updatedAt: string;
}

interface TrackedRootResponse {
  id: number;
  path: string;
  is_enabled: boolean;
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

export async function getTrackedRoots(): Promise<TrackedRoot[]> {
  const roots = await tauri<TrackedRootResponse[]>('get_all_tracked_root_paths');
  return roots.map(toTrackedRoot);
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
