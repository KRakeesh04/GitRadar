import { queryKeys } from "#/lib/query-keys";
import { getCommitByHash, getCommitDiffByHash, getCommitGraphByRepoId, getCommitInlineDiff, getCommitsByRepoId } from "#/lib/tauri/commits";
import { useQuery } from "@tanstack/react-query";

export function useCommitByHash(repoId: number, commitHash: string) {
  return useQuery({
    queryKey: queryKeys.commit(repoId, commitHash),
    queryFn: () => getCommitByHash(repoId, commitHash),
  });
}

export function useCommits(repoId: number, limit: number, offset: number) {
  return useQuery({
    queryKey: queryKeys.commits(repoId, limit, offset),
    queryFn: () => getCommitsByRepoId(repoId, limit, offset),
  });
}

export function useCommitGraph(repoId: number, limit: number, offset: number) {
  return useQuery({
    queryKey: queryKeys.commitsGraph(repoId, limit, offset),
    queryFn: () => getCommitGraphByRepoId(repoId, limit, offset),
  });
}

export function useCommitDiff(repoId: number, hash: string) {
  return useQuery({
    queryKey: queryKeys.commitDiff(repoId, hash),
    queryFn: () => getCommitDiffByHash(repoId, hash),
  });
}

export function useCommitDiffInline(repoId: number, hash: string) {
  return useQuery({
    queryKey: queryKeys.commitDiffInline(repoId, hash),
    queryFn: () => getCommitInlineDiff(repoId, hash),
  });
}