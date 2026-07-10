import clsx from "clsx";
import type { DiffLine } from "./types";

export function DiffLineView({
  line,
}: {
  line: DiffLine;
}) {
  const background = {
    Context: "",
    Added: "bg-green-500/10",
    Removed: "bg-red-500/10",
  }[line.line_type];

  const symbol = {
    Context: " ",
    Added: "+",
    Removed: "-",
  }[line.line_type];

  return (
    <div
      className={clsx(
        "grid grid-cols-[60px_60px_20px_1fr]",
        "font-mono text-sm",
        background
      )}
    >
      <div className="border-r px-2 text-right text-muted-foreground">
        {line.old_line_number ?? ""}
      </div>

      <div className="border-r px-2 text-right text-muted-foreground">
        {line.new_line_number ?? ""}
      </div>

      <div className="select-none px-2">
        {symbol}
      </div>

      <pre className="overflow-x-auto whitespace-pre-wrap px-2">
        {line.content}
      </pre>
    </div>
  );
}