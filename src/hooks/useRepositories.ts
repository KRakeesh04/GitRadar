import { queryKeys } from '#/lib/query-keys';
import type { RepositoryInfo } from '#/lib/tauri/repositories';
import {
  getContributorsByRepoId,
  getRepoLanguagesStats,
  getRepositoryActivityDaily,
  getTopContributorsByRepoId,
} from '#/lib/tauri/analytics';
import {
  getBranchByRepoIdAndName,
  getBranchesByRepoId,
  getPaginatedRepositories,
  getRecentRepositories,
  getRepositories,
  getRepositoriesByRootId,
  getRepositoryInfoById,
  getStarredRepositories,
  setRepositoryEnabled,
  setRepositoryStarred,
} from '#/lib/tauri/repositories';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';

export function useRepositories() {
  return useQuery({
    queryKey: queryKeys.repositories,
    queryFn: getRepositories,
  });
}

export function useRepositoriesByRootId(rootId: number) {
  return useQuery({
    queryKey: queryKeys.repositoriesByRoot(rootId),
    queryFn: () => getRepositoriesByRootId(rootId),
  });
}

export function usePaginatedRepositories(params: {
  search?: string;
  filter?: string;
  limit?: number;
  cursor?: number | null;
}) {
  return useQuery({
    queryKey: queryKeys.paginatedRepositories(params),
    queryFn: () => getPaginatedRepositories(params),
  });
}

export function useToggleRepositoryEnabled() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ repoId, enabled }: { repoId: number; enabled: boolean }) =>
      setRepositoryEnabled(repoId, enabled),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.repositories });
    },
  });
}

export function useToggleRepositoryStarred() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ repoId, isStarred }: { repoId: number; isStarred: boolean }) =>
      setRepositoryStarred(repoId, isStarred),
    onMutate: async ({ repoId, isStarred }) => {
      const starredPrefix = ['repositories', 'starred'];
      const recentPrefix = ['repositories', 'recent'];

      // Cancel any in-flight queries that read starred state
      await queryClient.cancelQueries({ queryKey: queryKeys.repository(repoId) });
      await queryClient.cancelQueries({ queryKey: queryKeys.repositories });
      await queryClient.cancelQueries({ queryKey: starredPrefix });
      await queryClient.cancelQueries({ queryKey: recentPrefix });

      // Snapshot previous data for rollback
      const prevRepo = queryClient.getQueryData<RepositoryInfo>(queryKeys.repository(repoId));
      const prevList = queryClient.getQueryData<RepositoryInfo[]>(queryKeys.repositories);
      const prevStarred = queryClient.getQueriesData<RepositoryInfo[]>({ queryKey: starredPrefix });
      const prevRecent = queryClient.getQueriesData<RepositoryInfo[]>({ queryKey: recentPrefix });

      // Optimistically update the single repository query
      queryClient.setQueryData<RepositoryInfo>(queryKeys.repository(repoId), old =>
        old ? { ...old, isStarred, starredAt: isStarred ? new Date().toISOString() : null } : old
      );

      // Optimistically update list snapshots in place (covers all limits/offsets via prefix)
      const patch = (repos: RepositoryInfo[] | undefined): RepositoryInfo[] | undefined =>
        repos?.map(repo =>
          repo.id === repoId
            ? { ...repo, isStarred, starredAt: isStarred ? new Date().toISOString() : null }
            : repo
        );
      if (prevList)
        queryClient.setQueryData<RepositoryInfo[]>(queryKeys.repositories, patch(prevList));
      queryClient.setQueriesData<RepositoryInfo[]>({ queryKey: starredPrefix }, patch);
      queryClient.setQueriesData<RepositoryInfo[]>({ queryKey: recentPrefix }, patch);

      return { prevRepo, prevList, prevStarred, prevRecent };
    },
    onError: (_err, _vars, context) => {
      if (!context) return;
      const { prevRepo, prevList, prevStarred, prevRecent } = context;
      if (prevRepo !== undefined)
        queryClient.setQueryData<RepositoryInfo>(queryKeys.repository(_vars.repoId), prevRepo);
      if (prevList) queryClient.setQueryData<RepositoryInfo[]>(queryKeys.repositories, prevList);
      prevStarred.forEach(([key, data]) => queryClient.setQueryData<RepositoryInfo[]>(key, data));
      prevRecent.forEach(([key, data]) => queryClient.setQueryData<RepositoryInfo[]>(key, data));
    },
    onSettled: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.repositories });
      queryClient.invalidateQueries({ queryKey: ['repositories', 'starred'] });
      queryClient.invalidateQueries({ queryKey: ['repositories', 'recent'] });
    },
  });
}

export function useStarredRepositories(limit = 11, offset = 0) {
  return useQuery({
    queryKey: queryKeys.starredRepositories(limit, offset),
    queryFn: () => getStarredRepositories(limit, offset),
  });
}

export function useRecentRepositories(limit = 11, offset = 0) {
  return useQuery({
    queryKey: queryKeys.recentRepositories(limit, offset),
    queryFn: () => getRecentRepositories(limit, offset),
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

export function useRepositoryActivity(
  repoId: number,
  startDate: string | null,
  endDate: string | null
) {
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
