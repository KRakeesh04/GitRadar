import { useState } from "react";
import { Eye, Code2, FileText, History } from "lucide-react";

import Editor from "@monaco-editor/react";
import { ReadmeViewer } from "../readme-previewer";
import { useTheme } from "#/contexts/ThemeContext";

type Props = {
  path: string;
  content: string;
};

function getLanguage(path: string) {
  const ext = path.split(".").pop()?.toLowerCase();

  switch (ext) {
    case "ts":
      return "typescript";

    case "tsx":
      return "typescript";

    case "js":
      return "javascript";

    case "jsx":
      return "javascript";

    case "json":
      return "json";

    case "rs":
      return "rust";

    case "py":
      return "python";

    case "go":
      return "go";

    case "java":
      return "java";

    case "cpp":
    case "cc":
    case "cxx":
      return "cpp";

    case "c":
      return "c";

    case "html":
      return "html";

    case "css":
      return "css";

    case "md":
      return "markdown";

    case "toml":
      return "toml";

    case "yaml":
    case "yml":
      return "yaml";

    default:
      return "plaintext";
  }
}

export default function PreviewPane({
  path,
  content,
}: Props) {
  const isMarkdown = path.toLowerCase().endsWith(".md");
  const [readmeMode, setReadmeMode] = useState<"preview" | "code" | "blame">("preview");
  const [fileMode, setFileMode] = useState<"code" | "blame">("code");
  const { theme } = useTheme();

  return (
    <div className="flex h-full flex-1 flex-col mb-3">
      {/* Header */}

      <div className="flex items-center justify-between border-b px-4 py-2">
        <div className="flex items-center gap-2 h-8">
          <FileText className="h-4 w-4 shrink-0" />
          <span className="truncate text-sm text-muted-foreground">
            {path}
          </span>
        </div>

        {isMarkdown ? (
          <div className="flex rounded-md border h-8 bg-card text-muted-foreground">
            <button
              onClick={() => setReadmeMode("preview")}
              className={`flex items-center gap-1 px-3 py-1 text-sm cursor-pointer ${readmeMode === "preview"
                ? "bg-(--brand-low) text-primary-foreground rounded-l-md"
                : ""
                }`}
            >
              <Eye className="h-4 w-4" />
              Preview
            </button>

            <button
              onClick={() => setReadmeMode("code")}
              className={`flex items-center gap-1 px-3 py-1 text-sm cursor-pointer ${readmeMode === "code"
                ? "bg-(--brand-low) text-primary-foreground"
                : ""
                }`}
            >
              <Code2 className="h-4 w-4" />
              Code
            </button>

            <button
              onClick={() => setReadmeMode("blame")}
              className={`flex items-center gap-1 px-3 py-1 text-sm cursor-pointer ${readmeMode === "blame"
                ? "bg-(--brand-low) text-primary-foreground rounded-r-md"
                : ""
                }`}
            >
              <History className="h-4 w-4" />
              Blame
            </button>
          </div>
        ) : (
          <div className="flex rounded-md border h-8 bg-card text-muted-foreground">
            <button
              onClick={() => setFileMode("code")}
              className={`flex items-center gap-1 px-3 py-1 text-sm cursor-pointer ${fileMode === "code"
                ? "bg-(--brand-low) text-primary-foreground rounded-l-md"
                : ""
                }`}
            >
              <Code2 className="h-4 w-4" />
              Code
            </button>

            <button
              onClick={() => setFileMode("blame")}
              className={`flex items-center gap-1 px-3 py-1 text-sm cursor-pointer ${fileMode === "blame"
                ? "bg-(--brand-low) text-primary-foreground rounded-r-md"
                : ""
                }`}
            >
              <History className="h-4 w-4" />
              Blame
            </button>
          </div>
        )}
      </div>

      {/* Body */}

      <div className="flex-1 overflow-auto pt-1">
        {isMarkdown && readmeMode === "preview" ? (
          <ReadmeViewer content={content} />
        ) : isMarkdown && readmeMode === "code" || !isMarkdown && fileMode === "code" ? (
          <Editor
            height="100%"
            language={getLanguage(path)}
            value={content}
            theme={theme === "dark" ? "vs-dark" : "light"}
            options={{
              readOnly: true,
              minimap: {
                enabled: false,
              },
              fontSize: 14,
              scrollBeyondLastLine: false,
              wordWrap: "on",
              automaticLayout: true,
            }}
          />
        ) : (
          <div className="flex h-full items-center justify-center text-muted-foreground">
            Blame view is not implemented yet.
          </div>
        )}
      </div>
    </div>
  );
}