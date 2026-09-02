import { queryKeys } from "#/lib/query-keys";
import { getFileDiff, getFileDiffHistory, getFileHotspots, getFilesByExtension, getFileStatsByPath, getFileStats, getRepoFiles, getRepoFilesByPath, getRepositoryFileContent, getRepositoryFileTree } from "#/lib/tauri/files";
import { getBranchesByRepoId } from "#/lib/tauri/repositories";
import { useQuery } from "@tanstack/react-query";

export function useRepoFiles(repoId: number) {
  return useQuery({
    queryKey: queryKeys.repositoryFiles(repoId),
    queryFn: () => getRepoFiles(repoId),
  });
}

export function useRepoFileTree(repoId: number) {
  return useQuery({
    queryKey: queryKeys.repositoryFileTree(repoId),
    queryFn: () => getRepositoryFileTree(repoId),
  });
}

export function useRepoBranches(repoId: number) {
  return useQuery({
    queryKey: queryKeys.branches(repoId),
    queryFn: () => getBranchesByRepoId(repoId),
  });
}

export function useRepoFileContent(repoId: number, filePath: string | null) {
  return useQuery({
    queryKey: queryKeys.repositoryFileContent(repoId, filePath ?? ''),
    queryFn: () => getRepositoryFileContent(repoId, filePath ?? ''),
    enabled: Boolean(filePath),
  });
}

export function useRepoFile(repoId: number, filePath: string) {
  return useQuery({
    queryKey: queryKeys.repositoryFilesByPath(repoId, filePath),
    queryFn: () => getRepoFilesByPath(repoId, filePath),
  });
}

export function useRepoFileDiff(repoId: number, filePath: string, commitHash: string) {
  return useQuery({
    queryKey: queryKeys.repositoryFileDiff(repoId, filePath, commitHash),
    queryFn: () => getFileDiff(repoId, filePath, commitHash),
  });
}

export function useRepoFileDiffHistory(repoId: number, filePath: string, limit: number, offset: number) {
  return useQuery({
    queryKey: queryKeys.repositoryFileDiffHistory(repoId, filePath, limit, offset),
    queryFn: () => getFileDiffHistory(repoId, filePath, limit, offset),
  });
}

export function useRepoFilesByExtension(repoId: number, extension: string) {
  return useQuery({
    queryKey: queryKeys.repositoryFilesByExtension(repoId, extension),
    queryFn: () => getFilesByExtension(repoId, extension),
  });
}

export function useRepoFileHotspots(repoId: number) {
  return useQuery({
    queryKey: queryKeys.repositoryFileHotspots(repoId),
    queryFn: () => getFileHotspots(repoId),
  });
}

export function useRepoFileStats(repoId: number) {
  return useQuery({
    queryKey: queryKeys.repositoryFileStats(repoId),
    queryFn: () => getFileStats(repoId),
  });
}

export function useRepoFileStatsByPath(repoId: number, filePath: string) {
  return useQuery({
    queryKey: queryKeys.repositoryFileStatsByPath(repoId, filePath),
    queryFn: () => getFileStatsByPath(repoId, filePath),
  });
}
