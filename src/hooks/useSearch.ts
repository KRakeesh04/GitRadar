import { useCallback, useEffect, useRef, useState } from 'react';
import { searchEverything } from '#/lib/tauri/search';
import type { SearchHit } from '#/lib/tauri/search';
import { searchRepositories } from '#/lib/tauri/repositories';
import type { RepositoryInfo } from '#/lib/tauri/repositories';

export const MIN_SEARCH_LENGTH = 3;

export function isQueryLongEnough(query: string): boolean {
  return query.trim().length >= MIN_SEARCH_LENGTH;
}

export type GlobalSearchResultKind = 'repository' | 'commit' | 'contributor' | 'file' | 'branch';

export interface GlobalSearchResult {
  kind: GlobalSearchResultKind;
  repoId: number;
  repoName: string;
  title: string;
  subtitle?: string;
}

function repoResults(repos: RepositoryInfo[]): GlobalSearchResult[] {
  return repos.map(repo => ({
    kind: 'repository' as const,
    repoId: repo.id,
    repoName: repo.name,
    title: repo.name,
    subtitle: repo.path,
  }));
}

function entityResults(hits: SearchHit[]): GlobalSearchResult[] {
  return hits.map(hit => {
    const kind = (
      ['commit', 'contributor', 'file', 'branch'].includes(hit.entityType) ? hit.entityType : 'file'
    ) as GlobalSearchResultKind;
    return {
      kind,
      repoId: hit.repoId,
      repoName: hit.repoName,
      title: hit.title,
      subtitle: hit.body || hit.repoName,
    };
  });
}

export interface InfiniteGlobalSearch {
  repositories: GlobalSearchResult[];
  entities: GlobalSearchResult[];
  isLoading: boolean; // initial page in flight
  isLoadingMore: boolean; // subsequent page in flight
  hasMoreRepos: boolean;
  hasMoreEntities: boolean;
  loadMore: () => void;
}

/**
 * Combines repository-identity matches (repo_search FTS) with cross-entity
 * matches (search_index FTS) into a single grouped result set, with automatic
 * infinite-scroll pagination. Repos paginate by cursor; entities by offset.
 * Requires at least MIN_SEARCH_LENGTH characters before querying.
 */
export function useInfiniteGlobalSearch(
  query: string,
  repoPageSize = 5,
  entityPageSize = 12
): InfiniteGlobalSearch {
  const trimmed = query.trim();
  const [debouncedQuery, setDebouncedQuery] = useState(trimmed);
  useEffect(() => {
    const handle = setTimeout(() => setDebouncedQuery(trimmed), 250);
    return () => clearTimeout(handle);
  }, [trimmed]);

  const activeQuery = debouncedQuery.trim();
  const activeEnabled = isQueryLongEnough(activeQuery);

  const [repositories, setRepositories] = useState<GlobalSearchResult[]>([]);
  const [entities, setEntities] = useState<GlobalSearchResult[]>([]);
  const [reposCursor, setReposCursor] = useState<number | null>(null);
  const [entityOffset, setEntityOffset] = useState(0);
  const [hasMoreRepos, setHasMoreRepos] = useState(false);
  const [hasMoreEntities, setHasMoreEntities] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [isLoadingMore, setIsLoadingMore] = useState(false);

  const inFlightRef = useRef(false);
  const queryRef = useRef(activeQuery);

  // Reset + initial fetch whenever the (debounced) query or page sizes change.
  useEffect(() => {
    queryRef.current = activeQuery;
    setRepositories([]);
    setEntities([]);
    setReposCursor(null);
    setEntityOffset(0);
    setHasMoreRepos(false);
    setHasMoreEntities(false);

    if (!activeEnabled) {
      setIsLoading(false);
      return;
    }

    let cancelled = false;
    setIsLoading(true);

    Promise.all([
      searchRepositories({ query: activeQuery, limit: repoPageSize }),
      searchEverything(activeQuery, entityPageSize, 0),
    ])
      .then(([repoPage, entityPage]) => {
        if (cancelled) return;
        setRepositories(repoResults(repoPage.items));
        setReposCursor(repoPage.nextCursor);
        setHasMoreRepos(repoPage.hasMore);
        setEntities(entityResults(entityPage.items));
        setEntityOffset(entityPage.items.length);
        setHasMoreEntities(entityPage.totalCount > entityPage.items.length);
      })
      .catch(() => {
        if (!cancelled) setHasMoreRepos(false);
      })
      .finally(() => {
        if (!cancelled) setIsLoading(false);
      });

    return () => {
      cancelled = true;
    };
  }, [activeQuery, activeEnabled, repoPageSize, entityPageSize]);

  const loadMore = useCallback(() => {
    if (!activeEnabled || isLoading || isLoadingMore || inFlightRef.current) return;
    if (queryRef.current !== activeQuery) return;
    if (!hasMoreRepos && !hasMoreEntities) return;

    inFlightRef.current = true;
    setIsLoadingMore(true);

    const tasks: Promise<unknown>[] = [];
    const repoPromise = hasMoreRepos
      ? searchRepositories({ query: activeQuery, limit: repoPageSize, cursor: reposCursor })
      : Promise.resolve(null);
    const entityPromise = hasMoreEntities
      ? searchEverything(activeQuery, entityPageSize, entityOffset)
      : Promise.resolve(null);

    tasks.push(
      repoPromise.then(repoPage => {
        if (repoPage) {
          setRepositories(prev => {
            const seen = new Set(prev.map(r => `${r.kind}:${r.repoId}:${r.title}`));
            const next = repoResults(repoPage.items).filter(
              r => !seen.has(`${r.kind}:${r.repoId}:${r.title}`)
            );
            return [...prev, ...next];
          });
          setReposCursor(repoPage.nextCursor);
          setHasMoreRepos(repoPage.hasMore);
        }
      }),
      entityPromise.then(entityPage => {
        if (entityPage) {
          setEntities(prev => {
            const seen = new Set(prev.map(r => `${r.kind}:${r.repoId}:${r.title}`));
            const next = entityResults(entityPage.items).filter(
              r => !seen.has(`${r.kind}:${r.repoId}:${r.title}`)
            );
            return [...prev, ...next];
          });
          setEntityOffset(prev => prev + entityPage.items.length);
          setHasMoreEntities(entityPage.totalCount > entityOffset + entityPage.items.length);
        }
      })
    );

    Promise.allSettled(tasks).finally(() => {
      inFlightRef.current = false;
      setIsLoadingMore(false);
    });
  }, [
    activeEnabled,
    isLoading,
    isLoadingMore,
    hasMoreRepos,
    hasMoreEntities,
    activeQuery,
    reposCursor,
    entityOffset,
    repoPageSize,
    entityPageSize,
  ]);

  return {
    repositories,
    entities,
    isLoading,
    isLoadingMore,
    hasMoreRepos,
    hasMoreEntities,
    loadMore,
  };
}
