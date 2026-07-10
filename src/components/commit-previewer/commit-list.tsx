import { GitBranch, GitCommitHorizontal } from 'lucide-react';
import { forwardRef, memo } from 'react';

import { cn } from '#/lib/utils';

import { COMMIT_ROW_HEIGHT, GRAPH_WIDTH } from './commit-graph-hooks';
import { CommitGraphSvg } from './commit-graph';
import type { CommitGraphLayout } from './commit-graph-hooks';
import type { CommitGraphNode } from './types';

export function CommitGraphList({
  commits,
  layout,
  selectedHash,
  onSelect,
  lastRowRef,
}: {
  commits: CommitGraphNode[];
  layout: CommitGraphLayout;
  selectedHash: string | null;
  onSelect: (hash: string) => void;
  lastRowRef: (node: HTMLDivElement | null) => void;
}) {
  return (
    <div className="relative min-w-0 overflow-hidden rounded-lg border bg-card">
      <div className="border-b px-4 py-3">
        <h2 className="text-sm font-semibold">Commit Graph</h2>
      </div>

      <div className="relative">
        <CommitGraphSvg layout={layout} selectedHash={selectedHash} />

        {commits.map((commit, index) => (
          <CommitRow
            key={commit.hash}
            ref={index === commits.length - 1 ? lastRowRef : undefined}
            commit={commit}
            selected={commit.hash === selectedHash}
            onSelect={onSelect}
          />
        ))}
      </div>
    </div>
  );
}

export const CommitRow = memo(
  forwardRef<
    HTMLDivElement,
    {
      commit: CommitGraphNode;
      selected: boolean;
      onSelect: (hash: string) => void;
    }
  >(function CommitRow({ commit, selected, onSelect }, ref) {
    const branchNames = getBranchNames(commit);

    return (
      <div
        ref={ref}
        role="button"
        tabIndex={0}
        className={cn(
          'group relative z-10 grid cursor-pointer grid-cols-[112px_minmax(0,1fr)] items-center border-b px-3 transition-colors last:border-b-0 hover:bg-muted/60',
          selected && 'bg-(--brand-low) hover:bg-(--brand-low)'
        )}
        style={{ minHeight: COMMIT_ROW_HEIGHT }}
        onClick={() => onSelect(commit.hash)}
        onKeyDown={event => {
          if (event.key === 'Enter' || event.key === ' ') {
            event.preventDefault();
            onSelect(commit.hash);
          }
        }}
      >
        <div className="h-full" style={{ width: GRAPH_WIDTH }} />

        <div className="min-w-0 py-3">
          <div className="flex min-w-0 items-center gap-2">
            <GitCommitHorizontal className="h-4 w-4 shrink-0 text-muted-foreground" />
            <span className="truncate font-medium text-foreground">{commit.subject}</span>
            {branchNames[0] ? (
              <span className="hidden shrink-0 items-center gap-1 rounded-md bg-blue-500/10 px-2 py-0.5 text-xs font-medium text-blue-600 sm:inline-flex">
                <GitBranch className="h-3 w-3" />
                {branchNames[0]}
              </span>
            ) : null}
            <span className="ml-auto shrink-0 font-mono text-xs text-muted-foreground">
              {shortHash(commit.hash)}
            </span>
          </div>

          <div className="mt-1 flex flex-wrap items-center gap-x-3 gap-y-1 text-xs text-muted-foreground">
            <span className="truncate">{commit.author_name}</span>
            <span>{relativeTime(commit.committed_at)}</span>
            <span className="text-emerald-600">+{commit.additions}</span>
            <span className="text-red-600">-{commit.deletions}</span>
            <span>{commit.total_changed_files_count} files</span>
          </div>
        </div>
      </div>
    );
  })
);

export function getBranchNames(commit: CommitGraphNode) {
  return commit.branch_names?.length
    ? commit.branch_names
    : commit.branch_name
      ? [commit.branch_name]
      : [];
}

export function shortHash(hash: string) {
  return hash.slice(0, 7);
}

export function relativeTime(value: string) {
  const timestamp = new Date(value).getTime();
  if (Number.isNaN(timestamp)) return value;

  const seconds = Math.max(1, Math.floor((Date.now() - timestamp) / 1000));
  const units: Array<[Intl.RelativeTimeFormatUnit, number]> = [
    ['year', 60 * 60 * 24 * 365],
    ['month', 60 * 60 * 24 * 30],
    ['week', 60 * 60 * 24 * 7],
    ['day', 60 * 60 * 24],
    ['hour', 60 * 60],
    ['minute', 60],
  ];

  const formatter = new Intl.RelativeTimeFormat(undefined, { numeric: 'auto' });
  for (const [unit, unitSeconds] of units) {
    if (seconds >= unitSeconds) {
      return formatter.format(-Math.floor(seconds / unitSeconds), unit);
    }
  }

  return formatter.format(-seconds, 'second');
}
