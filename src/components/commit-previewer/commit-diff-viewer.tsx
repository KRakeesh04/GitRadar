import { Columns2, FileText } from 'lucide-react';
import { useState } from 'react';

import { Button } from '#/components/ui/button';

import { FileDiffCard } from './file-diff-card';
import { InlineFileDiffCard } from './inline-file-diff-card';
import type { CommitDiff, CommitInlineDiff } from './types';

interface Props {
  diff: CommitDiff;
  inlineDiff?: CommitInlineDiff;
}

type DiffMode = 'patch' | 'inline';

export function CommitDiffViewer({ diff, inlineDiff }: Props) {
  const [mode, setMode] = useState<DiffMode>('inline');
  const activeMode = inlineDiff ? mode : 'patch';

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-end">
        <div className="inline-flex rounded-md border bg-card p-1">
          <Button
            type="button"
            variant={activeMode === 'inline' ? 'secondary' : 'ghost'}
            size="sm"
            disabled={!inlineDiff}
            onClick={() => setMode('inline')}
          >
            <Columns2 className="h-4 w-4" />
            Inline
          </Button>

          <Button
            type="button"
            variant={activeMode === 'patch' ? 'secondary' : 'ghost'}
            size="sm"
            onClick={() => setMode('patch')}
          >
            <FileText className="h-4 w-4" />
            Patch
          </Button>
        </div>
      </div>

      <div className="space-y-6">
        {activeMode === 'inline'
          ? inlineDiff?.files.map(file => (
            <InlineFileDiffCard key={`${file.old_path}-${file.new_path}`} file={file} />
          ))
          : diff.files.map(file => (
            <FileDiffCard key={`${file.old_path}-${file.new_path}`} file={file} />
          ))}
      </div>
    </div>
  );
}