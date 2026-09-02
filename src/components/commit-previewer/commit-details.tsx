import { Button } from '#/components/ui/button';
import { Copy, ExternalLink, FileText, GitBranch, GitCommitHorizontal } from 'lucide-react';
import type { ReactNode } from 'react';

import { CommitDiffViewer } from './commit-diff-viewer';
import { getBranchNames, relativeTime, shortHash } from './commit-list';
import type { CommitDiff, CommitGraphNode, CommitInlineDiff } from './types';
import { Link } from '@tanstack/react-router';

export function CommitDetails({
  diff,
  inlineDiff,
  commitInfo: _commitInfo,
}: {
  diff: CommitDiff;
  inlineDiff?: CommitInlineDiff;
  commitInfo: {
    hash: string;
    message: string;
    authorName: string;
    authorEmail: string;
    date: string;
  };
}) {
  return (
    <div className="flex flex-col gap-2">
      <div className="flex items-center gap-2">
        <h3 className="text-lg font-semibold">Commit Details</h3>
      </div>
      <div className="flex flex-col gap-1">
        <p className="text-sm text-muted-foreground">
          This section provides detailed information about the selected commit, including the commit
          message, author, date, and the changes made in the commit.
        </p>
      </div>
      <CommitDiffViewer diff={diff} inlineDiff={inlineDiff} />
    </div>
  );
}

export function CommitDetailsPanel({
  repoId,
  commit,
  onSelectParent,
}: {
  repoId: string;
  commit: CommitGraphNode | null;
  onSelectParent: (hash: string) => void;
}) {
  if (!commit) {
    return (
      <aside className="rounded-lg border bg-card p-5 text-sm text-muted-foreground">
        Select a commit to inspect its details.
      </aside>
    );
  }

  const branchNames = getBranchNames(commit);

  return (
    <aside className="sticky top-4 max-h-[calc(100vh-8rem)] overflow-y-auto rounded-lg border bg-card">
      <div className="border-b px-5 py-4">
        <div className="flex items-center gap-2 text-sm font-semibold">
          <GitCommitHorizontal className="h-4 w-4 text-muted-foreground" />
          Commit
        </div>
        <h2 className="mt-3 text-lg font-semibold leading-6">{commit.subject}</h2>
        <div className="mt-2 flex flex-wrap gap-2">
          {branchNames.map(branch => (
            <span
              key={branch}
              className="inline-flex items-center gap-1 rounded-md bg-blue-500/10 px-2 py-0.5 text-xs font-medium text-blue-600"
            >
              <GitBranch className="h-3 w-3" />
              {branch}
            </span>
          ))}
        </div>
      </div>

      <div className="space-y-5 p-5 text-sm">
        <DetailGroup label="Author">
          <div className="font-medium">{commit.author_name}</div>
          <div className="text-xs text-muted-foreground">{commit.author_email}</div>
        </DetailGroup>

        <DetailGroup label="Time">
          <div>{relativeTime(commit.committed_at)}</div>
          <div className="text-xs text-muted-foreground">
            {formatAbsoluteDate(commit.committed_at)}
          </div>
        </DetailGroup>

        <DetailGroup label="Full hash">
          <div className="flex items-center gap-2">
            <code className="min-w-0 flex-1 truncate rounded-md bg-muted px-2 py-1 font-mono text-xs">
              {commit.hash}
            </code>
            <Button
              variant="outline"
              size="icon-sm"
              aria-label="Copy full hash"
              onClick={() => void navigator.clipboard?.writeText(commit.hash)}
            >
              <Copy className="h-3.5 w-3.5" />
            </Button>
          </div>
        </DetailGroup>

        <DetailGroup label="Parent commits">
          {commit.parent_hashes.length > 0 ? (
            <div className="flex flex-wrap gap-2">
              {commit.parent_hashes.map(parent => (
                <button
                  key={parent}
                  type="button"
                  className="rounded-md border px-2 py-1 font-mono text-xs text-muted-foreground transition-colors hover:border-(--brand) hover:text-(--brand)"
                  onClick={() => onSelectParent(parent)}
                >
                  {shortHash(parent)}
                </button>
              ))}
            </div>
          ) : (
            <span className="text-muted-foreground">Root commit</span>
          )}
        </DetailGroup>

        <div className="grid grid-cols-3 gap-2">
          <StatBox label="Files" value={commit.total_changed_files_count} />
          <StatBox label="Added" value={`+${commit.additions}`} className="text-emerald-600" />
          <StatBox label="Deleted" value={`-${commit.deletions}`} className="text-red-600" />
        </div>

        <Link
          to="/repository/$id/diff/$hash"
          params={{
            id: repoId,
            hash: commit.hash,
          }}
          activeProps={{ className: 'text-black font-bold' }}
        >
          <Button className="w-full bg-(--brand) text-white hover:bg-(--brand-hover)">
            <FileText className="h-4 w-4" />
            Open Full Diff
            <ExternalLink className="ml-auto h-4 w-4" />
          </Button>
        </Link>
      </div>
    </aside>
  );
}

function DetailGroup({ label, children }: { label: string; children: ReactNode }) {
  return (
    <section>
      <div className="mb-1.5 text-xs font-medium uppercase text-muted-foreground">{label}</div>
      {children}
    </section>
  );
}

function StatBox({
  label,
  value,
  className,
}: {
  label: string;
  value: ReactNode;
  className?: string;
}) {
  return (
    <div className="rounded-md border bg-muted/30 p-3">
      <div className="text-xs text-muted-foreground">{label}</div>
      <div className={className ?? 'text-foreground'}>{value}</div>
    </div>
  );
}

function formatAbsoluteDate(value: string) {
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) return value;

  return new Intl.DateTimeFormat(undefined, {
    dateStyle: 'medium',
    timeStyle: 'short',
  }).format(date);
}
