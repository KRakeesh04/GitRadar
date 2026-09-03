import { useEffect, useMemo, useRef, useState } from 'react';
import { useNavigate } from '@tanstack/react-router';
import { Search, GitCommit, User, FileCode2, GitBranch, FolderGit2, Loader2 } from 'lucide-react';

import { cn } from '#/lib/utils';
import { isQueryLongEnough, useInfiniteGlobalSearch } from '#/hooks/useSearch';
import type { GlobalSearchResult } from '#/hooks/useSearch';
import { Input } from './ui/input';

const KIND_ORDER: GlobalSearchResult['kind'][] = [
  'repository',
  'commit',
  'contributor',
  'file',
  'branch',
];

const KIND_LABEL: Record<GlobalSearchResult['kind'], string> = {
  repository: 'Repositories',
  commit: 'Commits',
  contributor: 'Contributors',
  file: 'Files',
  branch: 'Branches',
};

const KIND_ICON: Record<GlobalSearchResult['kind'], React.ReactNode> = {
  repository: <FolderGit2 className="h-4 w-4" />,
  commit: <GitCommit className="h-4 w-4" />,
  contributor: <User className="h-4 w-4" />,
  file: <FileCode2 className="h-4 w-4" />,
  branch: <GitBranch className="h-4 w-4" />,
};

export function SearchBar({
  placeholder,
  className,
  mode = 'suggest',
  value,
  onChange,
  onSelect,
  repoLimit = 5,
  entityLimit = 12,
  autoFocus = false,
}: {
  placeholder: string;
  className?: string;
  mode?: 'suggest' | 'plain';
  value?: string;
  onChange?: (e: React.ChangeEvent<HTMLInputElement>) => void;
  onSelect?: (result: GlobalSearchResult) => void;
  repoLimit?: number;
  entityLimit?: number;
  autoFocus?: boolean;
}) {
  const navigate = useNavigate();
  const [query, setQuery] = useState(typeof value === 'string' ? value : '');
  const [open, setOpen] = useState(false);
  const [highlightedIndex, setHighlightedIndex] = useState(-1);
  const inputRef = useRef<HTMLInputElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const scrollRef = useRef<HTMLDivElement>(null);
  const sentinelRef = useRef<HTMLDivElement>(null);

  const isControlled = value !== undefined && onChange !== undefined;
  const currentQuery = isControlled ? value : query;

  const isSuggest = mode === 'suggest';
  const queryLongEnough = isQueryLongEnough(currentQuery);

  const {
    repositories,
    entities,
    isLoading,
    isLoadingMore,
    hasMoreRepos,
    hasMoreEntities,
    loadMore,
  } = useInfiniteGlobalSearch(isSuggest ? currentQuery : '', repoLimit, entityLimit);

  const groups = useMemo(() => {
    return KIND_ORDER.map(kind => ({
      kind,
      label: KIND_LABEL[kind],
      icon: KIND_ICON[kind],
      items: kind === 'repository' ? repositories : entities.filter(e => e.kind === kind),
    })).filter(group => group.items.length > 0);
  }, [repositories, entities]);

  const flatItems = useMemo(() => groups.flatMap(group => group.items), [groups]);

  const showDropdown = isSuggest && open && queryLongEnough && currentQuery.trim().length > 0;

  // Reset highlight when query/results change.
  useEffect(() => {
    setHighlightedIndex(-1);
  }, [currentQuery, isLoading]);

  // Close on outside click.
  useEffect(() => {
    if (!showDropdown) return;
    const handler = (event: MouseEvent) => {
      if (containerRef.current && !containerRef.current.contains(event.target as Node)) {
        setOpen(false);
      }
    };
    document.addEventListener('mousedown', handler);
    return () => document.removeEventListener('mousedown', handler);
  }, [showDropdown]);

  // Infinite scroll: load more when the sentinel becomes visible.
  useEffect(() => {
    if (!showDropdown || !sentinelRef.current) return;
    const sentinel = sentinelRef.current;
    const observer = new IntersectionObserver(
      entries => {
        if (entries[0].isIntersecting) loadMore();
      },
      { root: scrollRef.current, rootMargin: '80px' }
    );
    observer.observe(sentinel);
    return () => observer.disconnect();
  }, [showDropdown, loadMore, isLoading, isLoadingMore]);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (isControlled) {
      onChange(e);
    } else {
      setQuery(e.target.value);
    }
    setOpen(true);
    setHighlightedIndex(-1);
  };

  const handleSelect = (result: GlobalSearchResult) => {
    if (onSelect) {
      onSelect(result);
    } else {
      void navigate({ to: '/repository/$id', params: { id: String(result.repoId) } });
    }
    setOpen(false);
    inputRef.current?.blur();
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (!showDropdown || flatItems.length === 0) return;

    if (e.key === 'ArrowDown') {
      e.preventDefault();
      setHighlightedIndex(prev => (prev + 1) % flatItems.length);
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      setHighlightedIndex(prev => (prev <= 0 ? flatItems.length - 1 : prev - 1));
    } else if (e.key === 'Enter') {
      e.preventDefault();
      const item = highlightedIndex >= 0 ? flatItems[highlightedIndex] : flatItems[0];
      handleSelect(item);
    } else if (e.key === 'Escape') {
      setOpen(false);
    }
  };

  return (
    <div ref={containerRef} className={cn('relative max-w-lg flex-1 mr-auto', className)}>
      <Search className="pointer-events-none absolute left-3 top-1/2 h-5 w-5 -translate-y-1/2 text-muted-foreground" />
      <Input
        ref={inputRef}
        type="text"
        placeholder={placeholder}
        value={currentQuery}
        autoFocus={autoFocus}
        onChange={handleChange}
        onKeyDown={handleKeyDown}
        onFocus={() => setOpen(true)}
        className="pl-10 pr-10 bg-background text-foreground placeholder:text-muted-foreground border border-input focus-visible:outline-none focus-visible:ring-2 w-full rounded-md text-sm shadow-sm transition-all focus-visible:ring-(--brand)"
      />
      {isSuggest && isLoading && queryLongEnough && (
        <Loader2 className="pointer-events-none absolute right-3 top-1/2 h-4 w-4 -translate-y-1/2 animate-spin text-muted-foreground" />
      )}

      {showDropdown && (
        <div className="absolute z-50 mt-2 w-full overflow-hidden rounded-lg border border-border bg-card shadow-xl dark:bg-card">
          {flatItems.length === 0 ? (
            <div className="px-4 py-6 text-center text-sm text-muted-foreground">
              {isLoading ? 'Searching…' : `No results for "${currentQuery}".`}
            </div>
          ) : (
            <div ref={scrollRef} className="max-h-[min(60vh,24rem)] overflow-y-auto">
              {groups.map((group, groupIndex) => {
                const offset = groups
                  .slice(0, groupIndex)
                  .reduce((sum, g) => sum + g.items.length, 0);
                return (
                  <div key={group.kind}>
                    <div className="sticky top-0 flex items-center gap-2 border-b border-border/60 bg-muted/50 px-3 py-1.5 text-[11px] font-medium uppercase tracking-wide text-muted-foreground">
                      {group.icon}
                      {group.label}
                    </div>
                    {group.items.map((item, localIndex) => {
                      const flatIndex = offset + localIndex;
                      const isHighlighted = flatIndex === highlightedIndex;
                      return (
                        <button
                          key={`${item.kind}-${item.repoId}-${item.title}`}
                          type="button"
                          onMouseEnter={() => setHighlightedIndex(flatIndex)}
                          onClick={() => handleSelect(item)}
                          className={cn(
                            'flex w-full flex-col items-start gap-0.5 px-3 py-2 text-left text-sm transition-colors',
                            isHighlighted ? 'bg-muted' : ''
                          )}
                        >
                          <span className="flex w-full items-center gap-2">
                            <span className="text-muted-foreground">{group.icon}</span>
                            <span className="truncate font-medium text-foreground">
                              {item.title}
                            </span>
                          </span>
                          {item.subtitle && (
                            <span className="ml-6 line-clamp-1 font-mono text-xs text-muted-foreground">
                              {item.subtitle}
                            </span>
                          )}
                        </button>
                      );
                    })}
                  </div>
                );
              })}
              {(hasMoreRepos || hasMoreEntities) && (
                <div
                  ref={sentinelRef}
                  className="flex items-center justify-center gap-2 px-3 py-3 text-xs text-muted-foreground"
                >
                  {isLoadingMore && <Loader2 className="h-4 w-4 animate-spin" />}
                  {isLoadingMore ? 'Loading more…' : ''}
                </div>
              )}
            </div>
          )}
        </div>
      )}
    </div>
  );
}
