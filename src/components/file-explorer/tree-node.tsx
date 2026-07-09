import { useMemo, useState } from "react";
import { ChevronDown, ChevronRight } from "lucide-react";
import FileIcon from "./file-icon";
import type { FileTreeNode } from "#/routes/repository.$id/files";
import { sortNodes } from "./sort-nodes";

type Props = {
  node: FileTreeNode;
  selected: string;
  onSelect: (path: string) => void;
  level?: number;
};

export default function TreeNode({
  node,
  selected,
  onSelect,
  level = 0,
}: Props) {
  // Folders are CLOSED by default
  const [open, setOpen] = useState(false);

  // Always show folders first, then files, alphabetically
  const children = useMemo(() => sortNodes(node.children), [node.children]);

  if (node.is_directory) {
    return (
      <div>
        <button
          onClick={() => setOpen((v) => !v)}
          className="flex w-full items-center gap-1 rounded px-2 py-1 text-left hover:bg-accent"
          style={{ paddingLeft: `${level * 16 + 8}px` }}
        >
          {open ? (
            <ChevronDown className="h-4 w-4 shrink-0" />
          ) : (
            <ChevronRight className="h-4 w-4 shrink-0" />
          )}

          <FileIcon
            name={node.name}
            isDirectory
            isOpen={open}
          />

          <span className="truncate">{node.name}</span>
        </button>

        {open &&
          children.map((child) => (
            <TreeNode
              key={child.path}
              node={child}
              selected={selected}
              onSelect={onSelect}
              level={level + 1}
            />
          ))}
      </div>
    );
  }

  return (
    <button
      onClick={() => onSelect(node.path)}
      className={`flex w-full items-center gap-2 rounded py-1 text-left hover:bg-accent ${selected === node.path ? "bg-accent" : ""
        }`}
      style={{ paddingLeft: `${level * 16 + 28}px` }}
    >
      <FileIcon
        name={node.name}
        isDirectory={false}
      />

      <span className="truncate">{node.name}</span>
    </button>
  );
}