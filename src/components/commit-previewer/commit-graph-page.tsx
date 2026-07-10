import { AlertCircle } from 'lucide-react';
import { useMemo } from 'react';

import { Spinner } from '#/components/ui/spinner';

import { CommitDetailsPanel } from './commit-details';
import {
  useCommitGraphInfinite,
  useCommitGraphLayout,
  useLastRowObserver,
} from './commit-graph-hooks';
import { CommitGraphList } from './commit-list';

const PAGE_SIZE = 50;

export function CommitGraphPage({ repoId }: { repoId: string }) {
  const {
    commits,
    selectedHash,
    selectedCommit,
    setSelectedHash,
    loadNextPage,
    hasMore,
    isLoading,
    error,
  } = useCommitGraphInfinite(repoId, PAGE_SIZE);

  const layout = useCommitGraphLayout(commits);
  const commitHashes = useMemo(() => new Set(commits.map(commit => commit.hash)), [commits]);

  const lastRowRef = useLastRowObserver({
    isLoading,
    hasMore,
    onLoadMore: loadNextPage,
  });

  return (
    <div className="grid min-h-0 gap-4 xl:grid-cols-[minmax(0,1fr)_360px]">
      <div className="min-w-0 space-y-3">
        {error ? (
          <div className="flex items-center gap-2 rounded-lg border border-destructive/30 bg-destructive/10 px-4 py-3 text-sm text-destructive">
            <AlertCircle className="h-4 w-4" />
            {error}
          </div>
        ) : null}

        <CommitGraphList
          commits={commits}
          layout={layout}
          selectedHash={selectedHash}
          onSelect={setSelectedHash}
          lastRowRef={lastRowRef}
        />

        <div className="flex min-h-10 items-center justify-center text-sm text-muted-foreground">
          {isLoading ? (
            <span className="inline-flex items-center gap-2">
              <Spinner className="h-4 w-4" />
              Loading commits
            </span>
          ) : hasMore ? (
            'Scroll to load more commits'
          ) : commits.length > 0 ? (
            'End of commit history'
          ) : (
            'No commits found'
          )}
        </div>
      </div>

      <CommitDetailsPanel
        repoId={repoId}
        commit={selectedCommit}
        onSelectParent={hash => {
          if (commitHashes.has(hash)) setSelectedHash(hash);
        }}
      />
    </div>
  );
}
