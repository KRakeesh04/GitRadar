import { useMemo, useState } from "react";
import TreeNode from "./tree-node";
import type { FileTreeNode } from "@/routes/repository.$id/files";
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from "../ui/dropdown-menu";
import { Button } from "../ui/button";
import { ChevronsUpDown, GitBranch } from "lucide-react";
import { sortNodes } from "./sort-nodes";

type Props = {
  tree: FileTreeNode[];
  branches: string[];
  selected: string;
  onSelect: (path: string) => void;
};

export default function FileExplorer({
  tree,
  branches,
  selected,
  onSelect,
}: Props) {
  // Root folders first, then files, alphabetically
  const sortedTree = useMemo(() => sortNodes(tree), [tree]);

  const [activeBranch, setActiveBranch] = useState<string>(branches[0] || "");

  return (
    <aside className="flex h-full w-72 flex-col border-r bg-card">
      {/* Header */}
      <div className="flex items-center gap-2 border-b px-3 py-2 font-medium h-12.25">
        <DropdownMenu>
          <DropdownMenuTrigger render={
            <Button
              variant="outline"
              className={'w-full text-left cursor-pointer'}
            >
              <GitBranch className="mr-2 h-4 w-4" />
              <span>{activeBranch}</span>
              <ChevronsUpDown className="ml-auto h-4 w-4 opacity-50" />
            </Button>
          } />
          <DropdownMenuContent>
            {branches.map((branch) => (
              <DropdownMenuItem
                key={branch}
                onClick={() => setActiveBranch(branch)}
                className={`cursor-pointer ${activeBranch === branch ? 'bg-(--brand-low) focus:bg-(--brand-low)' : 'focus:bg-muted '}`}
              >
                {branch}
              </DropdownMenuItem>
            ))}
          </DropdownMenuContent>
        </DropdownMenu>
      </div>

      {/* Tree */}
      <div className="flex-1 overflow-y-auto py-2">
        {sortedTree.map((node) => (
          <TreeNode
            key={node.path}
            node={node}
            selected={selected}
            onSelect={onSelect}
          />
        ))}
      </div>
    </aside>
  );
}