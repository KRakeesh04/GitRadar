export type ChangeType =
  | "Added"
  | "Modified"
  | "Deleted"
  | "Renamed"
  | "Copied";

export type DiffLineType =
  | "Context"
  | "Added"
  | "Removed";

export interface CommitDiff {
  commit_hash: string;
  files: FileDiff[];
}

export interface FileDiff {
  old_path?: string | null;
  new_path: string;
  change_type: ChangeType;

  additions: number;
  deletions: number;

  hunks: DiffHunk[];
}

export interface DiffHunk {
  old_start: number;
  old_lines: number;

  new_start: number;
  new_lines: number;

  lines: DiffLine[];
}

export interface DiffLine {
  line_type: DiffLineType;

  old_line_number?: number | null;
  new_line_number?: number | null;

  content: string;
}