import { useEffect, useMemo, useState } from 'react';
import { createFileRoute, Link, Outlet, useRouterState } from '@tanstack/react-router';
import {
  ArrowLeft,
  ArrowRight,
  Clock,
  Filter,
  GitBranch,
  GitCommit,
  Power,
  Star,
  Users,
} from 'lucide-react';

import { SearchBar } from '#/components/searchbar';
import { Button } from '#/components/ui/button';
import { Card, CardContent, CardHeader } from '#/components/ui/card';
import { Separator } from '#/components/ui/separator';
import { Skeleton } from '#/components/ui/skeleton';
import {
  usePaginatedRepositories,
  useRepositories,
  useSearchedRepositories,
} from '#/hooks/useRepositories';
import type { RepositoryInfo } from '#/lib/tauri/repositories';

export const Route = createFileRoute('/repository')({
  component: RouteComponent,
});

enum RepositoryFilter {
  All = 'All',
  Clean = 'Clean',
  Dirty = 'Modified',
  Enabled = 'Enabled',
  Disabled = 'Disabled',
}

const filterOptions = [
  { label: RepositoryFilter.All, value: RepositoryFilter.All, apiKey: undefined },
  { label: RepositoryFilter.Clean, value: RepositoryFilter.Clean, apiKey: 'clean' },
  { label: RepositoryFilter.Dirty, value: RepositoryFilter.Dirty, apiKey: 'modified' },
  { label: RepositoryFilter.Enabled, value: RepositoryFilter.Enabled, apiKey: 'enabled' },
  { label: RepositoryFilter.Disabled, value: RepositoryFilter.Disabled, apiKey: 'disabled' },
];

function RouteComponent() {
  const [filter, setFilter] = useState<RepositoryFilter>(RepositoryFilter.All);
  const [searchQuery, setSearchQuery] = useState('');
  const [cursorHistory, setCursorHistory] = useState<(number | null)[]>([null]);
  const [pageIndex, setPageIndex] = useState(0);

  const currentCursor = cursorHistory[pageIndex] ?? null;
  const currentFilterOption = filterOptions.find(opt => opt.value === filter);

  const pathname = useRouterState({ select: state => state.location.pathname });

  // Debounce the raw input so we don't fire an FTS query on every keystroke.
  const [debouncedSearch, setDebouncedSearch] = useState('');
  useEffect(() => {
    const handle = setTimeout(() => setDebouncedSearch(searchQuery), 250);
    return () => clearTimeout(handle);
  }, [searchQuery]);

  // Total summary counts
  const allReposQuery = useRepositories();

  const trimmedSearch = debouncedSearch.trim();
  const isSearching = trimmedSearch.length > 0;

  const paginatedQuery = usePaginatedRepositories({
    search: isSearching ? trimmedSearch : '',
    filter: currentFilterOption?.apiKey,
    limit: 12,
    cursor: currentCursor,
    enabled: !isSearching,
  });

  const searchedQuery = useSearchedRepositories({
    query: trimmedSearch,
    filter: currentFilterOption?.apiKey,
    limit: 12,
    cursor: currentCursor,
  });

  const activeQuery = isSearching ? searchedQuery : paginatedQuery;

  if (pathname.replace(/\/$/, '') !== '/repository') {
    return <Outlet />;
  }

  const allRepos = useMemo(() => {
    const seen = new Map<number, RepositoryInfo>();
    for (const repo of allReposQuery.data ?? []) seen.set(repo.id, repo);
    return [...seen.values()];
  }, [allReposQuery.data]);
  const cleanCount = allRepos.filter(repo => !repo.isDirty).length;
  const modifiedCount = allRepos.filter(repo => repo.isDirty).length;
  const disabledCount = allRepos.filter(repo => !repo.isEnabled).length;

  const handleNextPage = () => {
    const nextCursor = activeQuery.data?.nextCursor;
    const hasMore = activeQuery.data?.hasMore;
    if (nextCursor == null || !hasMore) return;
    setPageIndex(prevIndex => {
      setCursorHistory(prevHistory =>
        prevIndex + 1 < prevHistory.length ? prevHistory : [...prevHistory, nextCursor]
      );
      return prevIndex + 1;
    });
  };

  const handlePrevPage = () => {
    if (pageIndex > 0) {
      setPageIndex(pageIndex - 1);
    }
  };

  const handleFilterChange = (newFilter: RepositoryFilter) => {
    setFilter(newFilter);
    setCursorHistory([null]);
    setPageIndex(0);
  };

  const handleSearchChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setSearchQuery(e.target.value);
    setCursorHistory([null]);
    setPageIndex(0);
  };

  const repoItems = activeQuery.data?.items ?? [];

  return (
    <div className="flex h-full w-full flex-col gap-3 overflow-y-auto px-[clamp(0.5rem,2vw,2.5rem)] py-5">
      <div className="flex flex-col gap-1">
        <span className="text-2xl font-medium">Repositories</span>
        <span className="text-sm text-muted-foreground">
          {allRepos.length} repositories tracked · {modifiedCount} with uncommitted changes ·{' '}
          {cleanCount} clean · {disabledCount} disabled
        </span>
      </div>

      <div className="mt-3 flex flex-wrap items-center gap-3">
        <SearchBar
          placeholder="Search repositories by name or path..."
          className="min-w-60 max-w-md"
          mode="plain"
          value={searchQuery}
          onChange={handleSearchChange}
        />
        <div className="flex items-center gap-2">
          <Filter className="h-4 w-4 text-muted-foreground" />
          <div className="flex flex-wrap gap-1">
            {filterOptions.map(option => (
              <Button
                key={option.value}
                variant={filter === option.value ? 'default' : 'outline'}
                size="sm"
                className={`cursor-pointer ${
                  filter === option.value
                    ? 'bg-(--brand) text-white hover:bg-(--brand-hover)'
                    : 'text-muted-foreground'
                }`}
                onClick={() => handleFilterChange(option.value)}
              >
                {option.label}
              </Button>
            ))}
          </div>
        </div>
      </div>

      {activeQuery.isLoading ? (
        <div className="my-4 grid grid-cols-1 gap-5 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
          {[1, 2, 3, 4, 5, 6].map(item => (
            <Skeleton key={item} className="h-44 w-full rounded-xl" />
          ))}
        </div>
      ) : repoItems.length === 0 ? (
        <div className="my-12 flex flex-col items-center justify-center rounded-xl border border-dashed border-border p-12 text-center">
          <GitBranch className="h-10 w-10 text-muted-foreground" />
          <h3 className="mt-3 text-lg font-medium">No repositories found</h3>
          <p className="mt-1 text-sm text-muted-foreground">
            {searchQuery
              ? `No repositories match "${searchQuery}" with the selected filter.`
              : 'Add tracked folders in Root Paths to discover repositories.'}
          </p>
        </div>
      ) : (
        <div className="my-4 grid grid-cols-1 gap-5 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
          {repoItems.map(repo => {
            const isRepoDisabled = !repo.isEnabled;

            return (
              <Link
                key={repo.id}
                to="/repository/$id"
                params={{ id: String(repo.id) }}
                className="group block"
              >
                <Card
                  className={`h-full transition-all duration-200 hover:border-(--brand) hover:shadow-md ${
                    isRepoDisabled ? 'border-dashed bg-muted/30 opacity-75 grayscale-30' : 'bg-card'
                  }`}
                >
                  <CardHeader className="p-4 pb-2">
                    <div className="min-w-0">
                      <div className="flex items-center gap-1.5">
                        <span
                          className={`truncate text-base font-semibold ${
                            isRepoDisabled
                              ? 'text-muted-foreground line-through decoration-muted-foreground/50'
                              : 'text-foreground'
                          }`}
                          title={repo.name}
                        >
                          {repo.name}
                        </span>
                        {repo.isStarred && (
                          <Star className="h-3.5 w-3.5 shrink-0 fill-amber-400 text-amber-400" />
                        )}
                      </div>
                      <p
                        className="truncate font-mono text-xs text-muted-foreground"
                        title={repo.path}
                      >
                        {repo.path}
                      </p>
                    </div>

                    <div className="mt-2 flex flex-wrap items-center gap-1.5 text-xs">
                      <span
                        className={`inline-flex items-center gap-1 rounded-full px-2 py-0.5 font-medium ${
                          isRepoDisabled
                            ? 'bg-zinc-200 text-zinc-700 dark:bg-zinc-800 dark:text-zinc-300'
                            : !repo.isDirty
                              ? 'bg-emerald-500/10 text-emerald-600 dark:text-emerald-400'
                              : 'bg-amber-500/10 text-amber-600 dark:text-amber-400'
                        }`}
                      >
                        <span
                          className={`h-1.5 w-1.5 rounded-full ${
                            isRepoDisabled
                              ? 'bg-zinc-400'
                              : !repo.isDirty
                                ? 'bg-emerald-500'
                                : 'bg-amber-500'
                          }`}
                        />
                        {isRepoDisabled ? 'Disabled' : !repo.isDirty ? 'Clean' : 'Modified'}
                      </span>

                      {repo.healthScore > 0 ? (
                        <span className="rounded-md border border-border px-1.5 py-0.5 text-muted-foreground">
                          Health: {(repo.healthScore * 100).toFixed(0)}%
                        </span>
                      ) : null}
                    </div>
                  </CardHeader>

                  <CardContent className="p-4 pt-2">
                    <div className="flex items-center justify-between text-xs text-muted-foreground">
                      <span className="flex items-center gap-1 truncate">
                        <GitBranch className="h-3.5 w-3.5" />
                        {repo.headBranch ?? 'Detached'}
                      </span>
                      <span className="flex items-center gap-1 text-muted-foreground">
                        <Clock className="h-3.5 w-3.5" />
                        {repo.updatedAt ? new Date(repo.updatedAt).toLocaleDateString() : 'N/A'}
                      </span>
                    </div>

                    <Separator className="my-2.5" />

                    <div className="flex items-center justify-between text-xs text-muted-foreground">
                      <span className="flex items-center gap-1" title="Commits">
                        <GitCommit className="h-3.5 w-3.5" />
                        {repo.totalCommits}
                      </span>
                      <span className="flex items-center gap-1" title="Contributors">
                        <Users className="h-3.5 w-3.5" />
                        {repo.uniqueContributors}
                      </span>
                      <span className="flex items-center gap-1" title="Status">
                        <Power
                          className={`h-3 w-3 ${repo.isEnabled ? 'text-emerald-500' : 'text-zinc-400'}`}
                        />
                        {repo.isEnabled ? 'Active' : 'Paused'}
                      </span>
                    </div>
                  </CardContent>
                </Card>
              </Link>
            );
          })}
        </div>
      )}

      {/* Pagination Controls */}
      <div className="mt-auto flex items-center justify-between border-t border-border pt-4 pb-2">
        <div className="text-xs text-muted-foreground">
          Showing page {pageIndex + 1} · Total {activeQuery.data?.totalCount ?? allRepos.length}{' '}
          results
        </div>
        <div className="flex items-center gap-2">
          <Button
            variant="outline"
            size="sm"
            onClick={handlePrevPage}
            disabled={pageIndex === 0 || activeQuery.isLoading}
            className="cursor-pointer"
          >
            <ArrowLeft className="mr-1 h-3.5 w-3.5" /> Previous
          </Button>
          <Button
            variant="outline"
            size="sm"
            onClick={handleNextPage}
            disabled={!activeQuery.data?.hasMore || activeQuery.isLoading}
            className="cursor-pointer"
          >
            Next <ArrowRight className="ml-1 h-3.5 w-3.5" />
          </Button>
        </div>
      </div>
    </div>
  );
}
