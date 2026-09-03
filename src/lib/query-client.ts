import { QueryClient } from '@tanstack/react-query';
import { createSyncStoragePersister } from '@tanstack/query-sync-storage-persister';
import { persistQueryClient } from '@tanstack/react-query-persist-client';

export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: 1,
      staleTime: 30_000,
      // Keep cached data around across navigation (including full page loads)
      // so lists like the sidebar's recent/starred repos restore instantly.
      gcTime: 1000 * 60 * 60,
    },
  },
});

// Persist the query cache to localStorage so navigating between pages (which
// can trigger full page reloads here) restores data instantly and refetches
// silently in the background instead of flashing loading skeletons. Guarded so
// SSR / non-browser environments skip persistence.
if (typeof window !== 'undefined') {
  const persister = createSyncStoragePersister({
    storage: window.localStorage,
    key: 'gitradar-query-cache',
  });

  void persistQueryClient({
    queryClient,
    persister,
    maxAge: 1000 * 60 * 60 * 24,
    buster: 'gitradar-v1',
    dehydrateOptions: {
      shouldDehydrateQuery: query => {
        const key = query.queryKey as unknown[];
        // Persist the sidebar lists (recent/starred repos), the base
        // repositories list, and tracked roots so they restore instantly.
        if (key[0] === 'repositories') {
          return key[1] === 'recent' || key[1] === 'starred' || key.length === 1;
        }
        return key[0] === 'tracked-roots';
      },
    },
  });
}
