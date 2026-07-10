import { DiffHunkView } from "./diff-hunk-view";
import type { FileDiff } from "./types";

export function FileDiffCard({
  file,
}: {
  file: FileDiff;
}) {
  return (
    <div className="overflow-hidden rounded-lg border bg-card">
      <div className="flex items-center justify-between border-b bg-muted px-4 py-2">
        <div>
          <div className="font-medium">
            {file.new_path}
          </div>

          {file.old_path &&
            file.old_path !== file.new_path && (
              <div className="text-xs text-muted-foreground">
                renamed from {file.old_path}
              </div>
            )}
        </div>

        <div className="flex gap-4 text-sm">
          <span className="text-green-600">
            +{file.additions}
          </span>

          <span className="text-red-600">
            -{file.deletions}
          </span>

          <span>
            {file.change_type}
          </span>
        </div>
      </div>

      {file.hunks.map((hunk, index) => (
        <DiffHunkView
          key={index}
          hunk={hunk}
        />
      ))}
    </div>
  );
}