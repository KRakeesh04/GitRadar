import { DiffLineView } from "./diff-line-view";
import type { DiffHunk } from "./types";

export function DiffHunkView({
  hunk,
}: {
  hunk: DiffHunk;
}) {
  return (
    <div>
      <div className="border-b bg-muted/40 px-4 py-1 font-mono text-xs text-muted-foreground">
        @@ -{hunk.old_start},{hunk.old_lines}
        {" "}
        +{hunk.new_start},{hunk.new_lines}
        {" "}
        @@
      </div>

      {hunk.lines.map((line, index) => (
        <DiffLineView
          key={index}
          line={line}
        />
      ))}
    </div>
  );
}