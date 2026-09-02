import { tauri } from "./tauri";

export interface RepositoryFile {
  id: number,
  repoId: number,
  path: string,
  name: string,
  extension: string | null,
  sizeBytes: number | null,
  isBinary: boolean,
  lastModifiedAt: string | null,
}

export interface RepositoryFileTreeNode {
  name: string;
  path: string;
  is_directory: boolean;
  size_or_file_count: number;
  children: RepositoryFileTreeNode[];
}

export interface RepositoryFileContent {
  mimeType: string;
  content: string;
  isBinary: boolean;
}

interface RepositoryFileContentResponse {
  mime_type: string;
  data: number[];
}

interface RepositoryFileResponse {
  id: number,
  repo_id: number,
  path: string,
  name: string,
  extension: string | null,
  size_bytes: number | null,
  is_binary: boolean,
  last_modified_at: string | null,
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
  const diff = await tauri<any>('get_file_diff', { repoId, filePath, commitHash });
  return diff;
}

export async function getFileDiffHistory(repoId: number, filePath: string, limit: number, offset: number): Promise<any[]> {
  const diffHistory = await tauri<any[]>('get_file_diff_history', { repoId, filePath, commitCount: limit, commitOffset: offset });
  return diffHistory;
}

export async function getRepoFiles(repoId: number): Promise<RepositoryFile[]> {
  const filesResponse = await tauri<RepositoryFileResponse[]>('get_repository_files', { repoId });
  return filesResponse.map(toRepositoryFile);
}

export async function getRepoFilesByPath(repoId: number, path: string): Promise<RepositoryFile[]> {
  const fileResponse = await tauri<RepositoryFileResponse>('get_repository_file_by_path', { repoId, filePath: path });
  return [toRepositoryFile(fileResponse)];
}

export async function getFilesByExtension(repoId: number, extension: string): Promise<RepositoryFile[]> {
  const extensions = await tauri<RepositoryFileResponse[]>('get_files_by_extension', { repoId, extension });
  return extensions.map(toRepositoryFile);
}

export async function getFileStats(repoId: number): Promise<CommitFileStat[]> {
  const fileStatsResponse = await tauri<CommitFileStatResponse[]>('get_file_stats', { repoId });
  return fileStatsResponse.map(toCommitFileStat);
}

export async function getFileStatsByPath(repoId: number, filePath: string): Promise<CommitFileStat[]> {
  const fileStatsResponse = await tauri<CommitFileStatResponse[]>('get_file_stats_by_path', { repoId, filePath });
  return fileStatsResponse.map(toCommitFileStat);
}

export async function getFileHotspots(repoId: number): Promise<FileHotspot[]> {
  const hotspotsResponse = await tauri<FileHotspotResponse[]>('get_file_hotspots', { repoId });
  return hotspotsResponse.map(toFileHotspot);
}

export async function getRepositoryFileTree(repoId: number): Promise<RepositoryFileTreeNode[]> {
  return tauri<RepositoryFileTreeNode[]>('get_repository_file_tree', { repoId });
}

export async function getRepositoryFileContent(
  repoId: number,
  filePath: string,
): Promise<RepositoryFileContent> {
  const response = await tauri<RepositoryFileContentResponse>('get_repository_file_content', {
    repoId,
    filePath,
  });
  const content = new TextDecoder().decode(new Uint8Array(response.data));
  const isBinary = !response.mime_type.startsWith('text/') &&
    !['application/json', 'application/javascript', 'application/xml'].includes(response.mime_type);

  return {
    mimeType: response.mime_type,
    content: isBinary ? '[Binary file cannot be previewed]' : content,
    isBinary,
  };
}
