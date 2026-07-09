import type { FileTreeNode } from "@/routes/repository.$id/files";

export function sortNodes(nodes: FileTreeNode[]) {
  return [...nodes].sort((a, b) => {
    if (a.is_directory !== b.is_directory) {
      return a.is_directory ? -1 : 1;
    }

    return a.name.localeCompare(b.name);
  });
}