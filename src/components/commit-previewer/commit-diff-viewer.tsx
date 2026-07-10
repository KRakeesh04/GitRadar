import { FileDiffCard } from "./file-diff-card";
import type { CommitDiff } from "./types";
interface Props {
  diff: CommitDiff;
}

export function CommitDiffViewer({ diff }: Props) {
  return (
    <div className="space-y-6">
      {diff.files.map((file) => (
        <FileDiffCard
          key={`${file.old_path}-${file.new_path}`}
          file={file}
        />
      ))}
    </div>
  );
}