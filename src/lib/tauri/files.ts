import { tauri } from "./tauri";

export interface RepositoryFile {
  id: number,
  repoId: number,
  path: String,
  name: String,
  extension: String | null,
  sizeBytes: number | null,
  isBinary: boolean,
  lastModifiedAt: String | null,
}

interface RepositoryFileResponse {
  id: number,
  repo_id: number,
  path: String,
  name: String,
  extension: String | null,
  size_bytes: number | null,
  is_binary: boolean,
  last_modified_at: String | null,
}

function toRepositoryFile(response: RepositoryFileResponse): RepositoryFile {
  return {
    id: response.id,
    repoId: response.repo_id,
    path: response.path,
    name: response.name,
    extension: response.extension,
    sizeBytes: response.size_bytes,
    isBinary: response.is_binary,
    lastModifiedAt: response.last_modified_at,
  };
}

export interface CommitFileStat {
  id: number,
  repoId: number,
  commitHash: string,
  filePath: string,
  changeType: string,
  additions: number,
  deletions: number,
  totalChanges: number,
}

interface CommitFileStatResponse {
  id: number,
  repo_id: number,
  commit_hash: string,
  file_path: string,
  change_type: string,
  additions: number,
  deletions: number,
  total_changes: number,
}

function toCommitFileStat(response: CommitFileStatResponse): CommitFileStat {
  return {
    id: response.id,
    repoId: response.repo_id,
    commitHash: response.commit_hash,
    filePath: response.file_path,
    changeType: response.change_type,
    additions: response.additions,
    deletions: response.deletions,
    totalChanges: response.total_changes,
  };
}

export interface FileHotspot {
  id: number,
  repoId: number,
  filePath: string,
  touchCount: number,
  churnScore: number,
  hotspotScore: number,
  lastTouchedAt: string | null,
  updatedAt: string,
}

interface FileHotspotResponse {
  id: number,
  repo_id: number,
  file_path: string,
  touch_count: number,
  churn_score: number,
  hotspot_score: number,
  last_touched_at: string | null,
  updated_at: string,
}

function toFileHotspot(response: FileHotspotResponse): FileHotspot {
  return {
    id: response.id,
    repoId: response.repo_id,
    filePath: response.file_path,
    touchCount: response.touch_count,
    churnScore: response.churn_score,
    hotspotScore: response.hotspot_score,
    lastTouchedAt: response.last_touched_at,
    updatedAt: response.updated_at,
  };
}

export async function getFileDiff(repoId: number, filePath: string, commitHash: string): Promise<any> {
  const diff = await tauri<any>('get_file_diff', { repo_id: repoId, file_path: filePath, commit_hash: commitHash });
  return diff;
}

export async function getFileDiffHistory(repoId: number, filePath: string, limit: number, offset: number): Promise<any[]> {
  const diffHistory = await tauri<any[]>('get_file_diff_history', { repo_id: repoId, file_path: filePath, commit_count: limit, commit_offset: offset });
  return diffHistory;
}

export async function getRepoFiles(repoId: number): Promise<RepositoryFile[]> {
  const filesResponse = await tauri<RepositoryFileResponse[]>('get_repository_files', { repo_id: repoId });
  return filesResponse.map(toRepositoryFile);
}

export async function getRepoFilesByPath(repoId: number, path: string): Promise<RepositoryFile[]> {
  const filesResponse = await tauri<RepositoryFileResponse[]>('get_repository_file_by_path', { repo_id: repoId, path });
  return filesResponse.map(toRepositoryFile);
}

export async function getFilesByExtension(repoId: number, extension: string): Promise<RepositoryFile[]> {
  const extensions = await tauri<RepositoryFileResponse[]>('get_files_by_extension', { repo_id: repoId, extension });
  return extensions.map(toRepositoryFile);
}

export async function getFileStats(repoId: number): Promise<CommitFileStat[]> {
  const fileStatsResponse = await tauri<CommitFileStatResponse[]>('get_file_stats', { repo_id: repoId });
  return fileStatsResponse.map(toCommitFileStat);
}

export async function getFileStatsByPath(repoId: number, filePath: string): Promise<CommitFileStat[]> {
  const fileStatsResponse = await tauri<CommitFileStatResponse[]>('get_file_stats_by_path', { repo_id: repoId, file_path: filePath });
  return fileStatsResponse.map(toCommitFileStat);
}

export async function getFileHotspots(repoId: number): Promise<FileHotspot[]> {
  const hotspotsResponse = await tauri<FileHotspotResponse[]>('get_file_hotspots', { repo_id: repoId });
  return hotspotsResponse.map(toFileHotspot);
}
