import {
  File,
  FileCode2,
  FileJson,
  FileText,
  Folder,
  FolderOpen,
  Image,
  Package,
  Archive,
} from "lucide-react";

type Props = {
  name: string;
  isDirectory: boolean;
  isOpen?: boolean;
};

export default function FileIcon({
  name,
  isDirectory,
  isOpen = false,
}: Props) {
  if (isDirectory) {
    return isOpen ? (
      <FolderOpen className="h-4 w-4 text-sky-500 shrink-0" />
    ) : (
      <Folder className="h-4 w-4 text-sky-500 shrink-0" />
    );
  }

  const ext = name.split(".").pop()?.toLowerCase();

  switch (ext) {
    case "ts":
    case "tsx":
    case "js":
    case "jsx":
    case "rs":
    case "py":
    case "go":
    case "java":
    case "cpp":
    case "c":
      return <FileCode2 className="h-4 w-4 shrink-0" />;

    case "json":
      return <FileJson className="h-4 w-4 shrink-0" />;

    case "md":
    case "txt":
      return <FileText className="h-4 w-4 shrink-0" />;

    case "png":
    case "jpg":
    case "jpeg":
    case "gif":
    case "svg":
    case "webp":
      return <Image className="h-4 w-4 shrink-0" />;

    case "zip":
    case "rar":
    case "7z":
    case "tar":
      return <Archive className="h-4 w-4 shrink-0" />;

    case "toml":
    case "lock":
    case "yaml":
    case "yml":
      return <Package className="h-4 w-4 shrink-0" />;

    default:
      return <File className="h-4 w-4 shrink-0" />;
  }
}