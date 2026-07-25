import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import {
  addTrackedRoot,
  deleteTrackedRoot,
  getTrackedRoots,
  rescanTrackedRoots,
  setTrackedRootEnabled,
} from '#/lib/tauri/tracked-roots';
import type { TrackedRoot } from '#/lib/tauri/tracked-roots';
import { queryKeys } from '#/lib/query-keys';

export function useTrackedRoots() {
  return useQuery({
    queryKey: queryKeys.trackedRoots,
    queryFn: getTrackedRoots,
  });
}

export function useAddTrackedRoot() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: addTrackedRoot,
    onSuccess: () => queryClient.invalidateQueries({ queryKey: queryKeys.trackedRoots }),
  });
}

export function useRescanTrackedRoots() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: rescanTrackedRoots,
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: queryKeys.trackedRoots });
      void queryClient.invalidateQueries({ queryKey: queryKeys.repositories });
    },
  });
}

export function useToggleTrackedRoot() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ path, enabled }: Pick<TrackedRoot, 'path' | 'enabled'>) =>
      setTrackedRootEnabled(path, enabled),
    onMutate: async ({ path, enabled }) => {
      await queryClient.cancelQueries({ queryKey: queryKeys.trackedRoots });
      const previous = queryClient.getQueryData<TrackedRoot[]>(queryKeys.trackedRoots);

      queryClient.setQueryData<TrackedRoot[]>(queryKeys.trackedRoots, roots =>
        roots?.map(root => (root.path === path ? { ...root, enabled } : root))
      );

      return { previous };
    },
    onError: (_error, _variables, context) => {
      if (context?.previous) {
        queryClient.setQueryData(queryKeys.trackedRoots, context.previous);
      }
    },
    onSettled: () => queryClient.invalidateQueries({ queryKey: queryKeys.trackedRoots }),
  });
}

export function useDeleteTrackedRoot() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: deleteTrackedRoot,
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: queryKeys.trackedRoots });
      void queryClient.invalidateQueries({ queryKey: queryKeys.repositories });
    },
  });
}
