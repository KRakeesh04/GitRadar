import { queryKeys } from "#/lib/query-keys";
import { getFileDiff, getFileDiffHistory, getFileHotspots, getFilesByExtension, getFileStatsByPath, getFileStats, getRepoFiles, getRepoFilesByPath } from "#/lib/tauri/files";
import { useQuery } from "@tanstack/react-query";

export function useRepoFiles(repoId: number) {
  return useQuery({
    queryKey: queryKeys.repositoryFiles(repoId),
    queryFn: () => getRepoFiles(repoId),
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