import { queryKeys } from "#/lib/query-keys";
import { getContributorsByRepoId, getRepoLanguagesStats, getRepositoryActivityDaily, getTopContributorsByRepoId } from "#/lib/tauri/analytics";
import { getBranchByRepoIdAndName, getBranchesByRepoId, getRepositories, getRepositoryInfoById } from "#/lib/tauri/repositories";
import { useQuery } from "@tanstack/react-query";

export function useRepositories() {
  return useQuery({
    queryKey: queryKeys.repositories,
    queryFn: getRepositories,
  });
}

export function useRepositoryById(repoId: number) {
  return useQuery({
    queryKey: queryKeys.repository(repoId),
    queryFn: () => getRepositoryInfoById(repoId),
  });
}

export function useBranchesByRepoId(repoId: number) {
  return useQuery({
    queryKey: queryKeys.branches(repoId),
    queryFn: () => getBranchesByRepoId(repoId),
  });
}

export function useBranchByName(repoId: number, branchName: string) {
  return useQuery({
    queryKey: queryKeys.branch(repoId, branchName),
    queryFn: () => getBranchByRepoIdAndName(repoId, branchName),
  });
}

export function useContributors(repoId: number) {
  return useQuery({
    queryKey: queryKeys.contributors(repoId),
    queryFn: () => getContributorsByRepoId(repoId),
  });
}

export function useTopContributors(repoId: number, limit: number) {
  return useQuery({
    queryKey: queryKeys.topContributors(repoId),
    queryFn: () => getTopContributorsByRepoId(repoId, limit),
  });
}

export function useRepositoryActivity(repoId: number, startDate: string | null, endDate: string | null) {
  return useQuery({
    queryKey: queryKeys.repositoryActivity(repoId),
    queryFn: () => getRepositoryActivityDaily(repoId, startDate, endDate),
  });
}

export function useRepositoryLanguagesStats(repoId: number) {
  return useQuery({
    queryKey: queryKeys.repositoryLanguagesStats(repoId),
    queryFn: () => getRepoLanguagesStats(repoId),
  });
}