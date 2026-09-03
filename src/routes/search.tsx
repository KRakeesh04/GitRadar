import { useEffect, useRef, useState } from 'react';
import { createFileRoute, Link } from '@tanstack/react-router';
import { FileCode2, FolderGit2, GitBranch, GitCommit, Loader2, Search, User } from 'lucide-react';

import { SearchBar } from '#/components/searchbar';
import { Card, CardContent, CardHeader } from '#/components/ui/card';
import { useInfiniteGlobalSearch } from '#/hooks/useSearch';
import type { GlobalSearchResultKind } from '#/hooks/useSearch';

export const Route = createFileRoute('/search')({
  component: RouteComponent,
});

const entityIcons: Record<GlobalSearchResultKind, React.ReactNode> = {
  repository: <FolderGit2 className="h-4 w-4" />,
  commit: <GitCommit className="h-4 w-4" />,
  contributor: <User className="h-4 w-4" />,
  file: <FileCode2 className="h-4 w-4" />,
  branch: <GitBranch className="h-4 w-4" />,
};

const KIND_LABEL: Record<GlobalSearchResultKind, string> = {
  repository: 'Repositories',
  commit: 'Commits',
  contributor: 'Contributors',
  file: 'Files',
  branch: 'Branches',
};

function RouteComponent() {
  const [rawQuery, setRawQuery] = useState('');
  const [debounced, setDebounced] = useState('');
  const sentinelRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handle = setTimeout(() => setDebounced(rawQuery), 250);
    return () => clearTimeout(handle);
  }, [rawQuery]);

  const trimmed = debounced.trim();
  const {
    repositories,
    entities,
    isLoading,
    isLoadingMore,
    hasMoreRepos,
    hasMoreEntities,
    loadMore,
  } = useInfiniteGlobalSearch(trimmed, 20, 30);

  const groups = (
    [
      ['repository', repositories],
      ['commit', entities.filter(e => e.kind === 'commit')],
      ['contributor', entities.filter(e => e.kind === 'contributor')],
      ['file', entities.filter(e => e.kind === 'file')],
      ['branch', entities.filter(e => e.kind === 'branch')],
    ] as const
  )
    .map(([kind, items]) => ({ kind, items }))
    .filter(group => group.items.length > 0);

  const totalCount = repositories.length + entities.length;
  const hasMore = hasMoreRepos || hasMoreEntities;

  // Infinite scroll: load more when sentinel scrolls into view.
  useEffect(() => {
    if (!sentinelRef.current || !hasMore) return;
    const sentinel = sentinelRef.current;
    const observer = new IntersectionObserver(
      entries => {
        if (entries[0].isIntersecting) loadMore();
      },
      { rootMargin: '200px' }
    );
    observer.observe(sentinel);
    return () => observer.disconnect();
  }, [hasMore, loadMore, isLoading, isLoadingMore]);

  return (
    <div className="flex h-full w-full flex-col gap-3 overflow-y-auto px-[clamp(0.5rem,2vw,2.5rem)] py-5">
      <div className="flex flex-col gap-1">
        <span className="text-2xl font-medium">Search</span>
        <span className="text-sm text-muted-foreground">
          Find repositories, commits, contributors, files, and branches across your tracked
          projects. Type at least 3 characters.
        </span>
      </div>

      <div className="mt-2 max-w-md">
        <SearchBar
          placeholder="Search repos, commits, files..."
          className="w-full"
          mode="plain"
          value={rawQuery}
          onChange={e => setRawQuery(e.target.value)}
        />
      </div>

      {trimmed.length > 0 && trimmed.length < 3 ? (
        <div className="my-6 text-sm text-muted-foreground">
          Type at least 3 characters to search.
        </div>
      ) : trimmed.length === 0 ? (
        <div className="my-12 flex flex-col items-center justify-center rounded-xl border border-dashed border-border p-12 text-center">
          <Search className="h-10 w-10 text-muted-foreground" />
          <h3 className="mt-3 text-lg font-medium">Start typing to search</h3>
        </div>
      ) : isLoading ? (
        <div className="flex items-center gap-2 text-sm text-muted-foreground">
          <Loader2 className="h-4 w-4 animate-spin" />
          Searching&hellip;
        </div>
      ) : groups.length === 0 ? (
        <div className="my-6 text-sm text-muted-foreground">
          No results for &ldquo;{trimmed}&rdquo;.
        </div>
      ) : (
        <div className="my-4 flex flex-col gap-4">
          {groups.map(group => (
            <div key={group.kind}>
              <div className="mb-2 flex items-center gap-2 text-xs font-medium uppercase tracking-wide text-muted-foreground">
                {entityIcons[group.kind]}
                {KIND_LABEL[group.kind]}
              </div>
              <div className="flex flex-col gap-2">
                {group.items.map((hit, index) => (
                  <Link
                    key={`${hit.kind}-${hit.repoId}-${hit.title}-${index}`}
                    to="/repository/$id"
                    params={{ id: String(hit.repoId) }}
                    className="block"
                  >
                    <Card className="cursor-pointer transition-all hover:border-(--brand) hover:shadow-md">
                      <CardHeader className="p-4 pb-2">
                        <div className="flex items-center gap-2">
                          {entityIcons[hit.kind]}
                          <span className="text-sm font-medium text-foreground">{hit.title}</span>
                        </div>
                      </CardHeader>
                      <CardContent className="p-4 pt-1">
                        {hit.subtitle && (
                          <p className="line-clamp-2 font-mono text-xs text-muted-foreground">
                            {hit.subtitle}
                          </p>
                        )}
                        <div className="mt-2 flex items-center gap-2">
                          <span className="rounded-md bg-muted px-1.5 py-0.5 text-[11px] capitalize text-muted-foreground">
                            {KIND_LABEL[hit.kind]}
                          </span>
                          <span className="truncate text-xs text-muted-foreground">
                            {hit.repoName}
                          </span>
                        </div>
                      </CardContent>
                    </Card>
                  </Link>
                ))}
              </div>
            </div>
          ))}

          {hasMore && (
            <div
              ref={sentinelRef}
              className="flex items-center justify-center gap-2 py-4 text-sm text-muted-foreground"
            >
              {isLoadingMore ? (
                <>
                  <Loader2 className="h-4 w-4 animate-spin" />
                  Loading more…
                </>
              ) : (
                'Scroll for more'
              )}
            </div>
          )}
        </div>
      )}

      {totalCount > 0 && groups.length > 0 && (
        <div className="text-xs text-muted-foreground">
          {totalCount} result{totalCount === 1 ? '' : 's'}
        </div>
      )}
    </div>
  );
}
