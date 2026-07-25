import { listen } from "@tauri-apps/api/event";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { queryKeys } from "#/lib/query-keys";
import {
  getIndexingJobs,
  getLatestIndexingJob,
  syncRepositories,
  startRepositorySync,
  type IndexingJob,
  type SyncProgressEvent,
} from "#/lib/tauri/sync";
import { useEffect } from "react";

export function useLatestIndexingJob(repoId: number) {
  return useQuery({
    queryKey: queryKeys.latestIndexingJob(repoId),
    queryFn: () => getLatestIndexingJob(repoId),
    refetchInterval: (query) => {
      const status = query.state.data?.status;
      return status === "pending" || status === "running" ? 1_000 : false;
    },
  });
}

export function useIndexingJobs(repoId: number, limit = 20) {
  return useQuery({
    queryKey: queryKeys.indexingJobs(repoId),
    queryFn: () => getIndexingJobs(repoId, limit),
    refetchInterval: 1_000,
  });
}

export function useStartRepositorySync(repoId: number) {
  const queryClient = useQueryClient();
  const latestKey = queryKeys.latestIndexingJob(repoId);

  useEffect(() => {
    let unlisten: (() => void) | undefined;
    let active = true;

    void listen<SyncProgressEvent>("sync:progress", (event) => {
      if (!active || event.payload.repo_id !== repoId) return;
      queryClient.setQueryData<IndexingJob | null>(latestKey, (previous) => ({
        id: event.payload.job_id,
        repoId,
        jobType: previous?.jobType ?? "sync",
        status: event.payload.status,
        progress: event.payload.progress,
        totalItems: event.payload.total_items,
        processedItems: event.payload.processed_items,
        errorMessage: null,
        startedAt: previous?.startedAt ?? new Date().toISOString(),
        completedAt: event.payload.status === "completed" ? new Date().toISOString() : null,
        createdAt: previous?.createdAt ?? new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      }));
    }).then((cleanup) => {
      if (active) unlisten = cleanup;
      else cleanup();
    });

    return () => {
      active = false;
      unlisten?.();
    };
  }, [latestKey, queryClient, repoId]);

  return useMutation({
    mutationFn: () => startRepositorySync(repoId),
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: queryKeys.latestIndexingJob(repoId) });
      void queryClient.invalidateQueries({ queryKey: queryKeys.indexingJobs(repoId) });
    },
    onSettled: () => {
      void queryClient.invalidateQueries({ queryKey: queryKeys.repositories });
      void queryClient.invalidateQueries({ queryKey: queryKeys.repository(repoId) });
    },
  });
}

export function useSyncRepositories() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (repoIds: number[]) => syncRepositories(repoIds),
    onSettled: () => {
      void queryClient.invalidateQueries({ queryKey: queryKeys.repositories });
      void queryClient.invalidateQueries({ queryKey: queryKeys.trackedRoots });
    },
  });
}
