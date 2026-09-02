import { useEffect, useMemo, useState } from "react";
import { createFileRoute } from "@tanstack/react-router";

import FileExplorer from "#/components/file-explorer/file-explorer";
import PreviewPane from "#/components/file-explorer/preview-pane";
import {
  useRepoBranches,
  useRepoFileContent,
  useRepoFileTree,
} from "#/hooks/useFiles";

export const Route = createFileRoute("/repository/$id/files")({
  component: RouteComponent,
});

export type FileTreeNode = {
  name: string;
  path: string;
  is_directory: boolean;
  size_or_file_count: number;
  children: FileTreeNode[];
};

function findReadme(nodes: FileTreeNode[]): string | undefined {
  for (const node of nodes) {
    if (!node.is_directory && node.path.toLowerCase() === "readme.md") {
      return node.path;
    }

    if (node.is_directory) {
      const readme = findReadme(node.children);
      if (readme) return readme;
    }
  }

  return undefined;
}

function findFirstFile(nodes: FileTreeNode[]): string | undefined {
  for (const node of nodes) {
    if (!node.is_directory) return node.path;

    const childFile = findFirstFile(node.children);
    if (childFile) return childFile;
  }

  return undefined;
}

function getErrorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

function RouteComponent() {
  const { id } = Route.useParams();
  const repoId = Number(id);
  const treeQuery = useRepoFileTree(repoId);
  const branchesQuery = useRepoBranches(repoId);
  const [selectedPath, setSelectedPath] = useState<string | null>(null);
  const contentQuery = useRepoFileContent(repoId, selectedPath);

  const tree = treeQuery.data ?? [];
  const branches = useMemo(
    () => (branchesQuery.data ?? []).map(branch => branch.name),
    [branchesQuery.data],
  );

  useEffect(() => {
    if (selectedPath || tree.length === 0) return;
    setSelectedPath(findReadme(tree) ?? findFirstFile(tree) ?? null);
  }, [selectedPath, tree]);

  if (treeQuery.isPending || branchesQuery.isPending) {
    return <FilePageMessage message="Loading repository files…" />;
  }

  if (treeQuery.isError || branchesQuery.isError) {
    return (
      <FilePageMessage
        message={getErrorMessage(treeQuery.error ?? branchesQuery.error)}
      />
    );
  }

  if (tree.length === 0) {
    return <FilePageMessage message="No indexed files found. Run a repository sync first." />;
  }

  const content = contentQuery.data?.content ?? "";

  return (
    <div className="flex h-[calc(100vh-180px)] overflow-hidden rounded-lg border bg-card">
      <FileExplorer
        tree={tree}
        branches={branches}
        selected={selectedPath ?? ""}
        onSelect={setSelectedPath}
      />

      {selectedPath ? (
        contentQuery.isPending ? (
          <FilePageMessage message="Loading file content…" />
        ) : contentQuery.isError ? (
          <FilePageMessage message={getErrorMessage(contentQuery.error)} />
        ) : (
          <PreviewPane path={selectedPath} content={content} />
        )
      ) : (
        <FilePageMessage message="Select a file to preview it." />
      )}
    </div>
  );
}

function FilePageMessage({ message }: { message: string }) {
  return (
    <div className="flex h-[calc(100vh-180px)] w-full items-center justify-center rounded-lg border bg-card text-sm text-muted-foreground">
      {message}
    </div>
  );
}
