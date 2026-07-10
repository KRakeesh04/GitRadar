import { DiffLineView } from './diff-line-view';
import type { InlineFileDiff } from './types';

export function InlineFileDiffCard({ file }: { file: InlineFileDiff }) {
  const additions = file.lines.filter(line => line.line_type === 'Added').length;
  const deletions = file.lines.filter(line => line.line_type === 'Removed').length;

  return (
    <div className="overflow-hidden rounded-lg border bg-card">
      <div className="flex items-center justify-between gap-4 border-b bg-muted px-4 py-2">
        <div className="min-w-0">
          <div className="truncate font-medium">{file.new_path}</div>

          {file.old_path && file.old_path !== file.new_path && (
            <div className="truncate text-xs text-muted-foreground">
              renamed from {file.old_path}
            </div>
          )}
        </div>

        <div className="flex shrink-0 gap-4 text-sm">
          <span className="text-green-600">+{additions}</span>

          <span className="text-red-600">-{deletions}</span>

          <span>{file.change_type}</span>
        </div>
      </div>

      <div>
        {file.lines.map((line, index) => (
          <DiffLineView key={index} line={line} />
        ))}
      </div>
    </div>
  );
}
