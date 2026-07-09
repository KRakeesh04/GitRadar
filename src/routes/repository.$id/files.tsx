import { useMemo, useState } from "react";

import FileExplorer from "#/components/file-explorer/file-explorer";
import PreviewPane from "#/components/file-explorer/preview-pane";

import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/repository/$id/files")({
  component: RouteComponent,
});

type FileData = {
  mime_type: string;
  data: Uint8Array | string;
};

export type FileTreeNode = {
  name: string;
  path: string;
  is_directory: boolean;
  size_or_file_count: number;
  children: FileTreeNode[];
};

const treeSampleData: FileTreeNode[] = [
  {
    name: "src",
    path: "src",
    is_directory: true,
    size_or_file_count: 3,
    children: [
      {
        name: "index.tsx",
        path: "src/index.tsx",
        is_directory: false,
        size_or_file_count: 1024,
        children: [],
      },
      {
        name: "App.tsx",
        path: "src/App.tsx",
        is_directory: false,
        size_or_file_count: 2048,
        children: [],
      },
      {
        name: "components",
        path: "src/components",
        is_directory: true,
        size_or_file_count: 2,
        children: [
          {
            name: "Button.tsx",
            path: "src/components/Button.tsx",
            is_directory: false,
            size_or_file_count: 512,
            children: [],
          },
          {
            name: "Input.tsx",
            path: "src/components/Input.tsx",
            is_directory: false,
            size_or_file_count: 1024,
            children: [],
          },
        ],
      },
    ],
  },
  {
    name: "README.md",
    path: "README.md",
    is_directory: false,
    size_or_file_count: 512,
    children: [],
  },
  {
    name: "package.json",
    path: "package.json",
    is_directory: false,
    size_or_file_count: 256,
    children: [],
  },
  {
    name: "public",
    path: "public",
    is_directory: true,
    size_or_file_count: 1,
    children: [],
  },
];

const sampleFiles: Record<string, string> = {
  "README.md": `# Sample README

This is a sample README file for the repository.

## Features

- Feature 1
- Feature 2
- Feature 3

## Installation

1. Clone repository
2. Install dependencies
3. Run project

| Name | Commits |
|------|---------|
| John | 24 |
| Jane | 17 |

## Features

- Feature 1
- Feature 2
- Feature 3

## Installation

1. Clone repository
2. Install dependencies
3. Run project

| Name | Commits |
|------|---------|
| John | 24 |
| Jane | 17 |

## Features

- Feature 1
- Feature 2
- Feature 3

## Installation

1. Clone repository
2. Install dependencies
3. Run project

| Name | Commits |
|------|---------|
| John | 24 |
| Jane | 17 |

## Features

- Feature 1
- Feature 2
- Feature 3

## Installation

1. Clone repository
2. Install dependencies
3. Run project

| Name | Commits |
|------|---------|
| John | 24 |
| Jane | 17 |

## Features

- Feature 1
- Feature 2
- Feature 3

## Installation

1. Clone repository
2. Install dependencies
3. Run project

| Name | Commits |
|------|---------|
| John | 24 |
| Jane | 17 |
`,

  "package.json": `{
  "name": "gitradar",
  "version": "0.1.0",
  "private": true
}
`,

  "src/index.tsx": `import React from "react";
import ReactDOM from "react-dom/client";

import App from "./App";

ReactDOM.createRoot(document.getElementById("root")!).render(
    <App />
);
`,

  "src/App.tsx": `export default function App() {
    return <h1>Hello GitRadar</h1>;
}
`,

  "src/components/Button.tsx": `export function Button() {
    return <button>Click Me</button>;
}
`,

  "src/components/Input.tsx": `export function Input() {
    return <input />;
}
`,
};

const branchesSampleData = ["main", "dev", "feature-1", "feature-2"];

function findReadme(nodes: FileTreeNode[]): string | undefined {
  for (const node of nodes) {
    if (
      !node.is_directory &&
      node.name.toLowerCase() === "readme.md"
    ) {
      return node.path;
    }

    if (node.is_directory) {
      const result = findReadme(node.children);

      if (result) return result;
    }
  }

  return undefined;
}

function RouteComponent() {
  const defaultFile = useMemo(
    () => findReadme(treeSampleData) ?? "package.json",
    []
  );

  const [selectedPath, setSelectedPath] =
    useState(defaultFile);

  const content =
    sampleFiles[selectedPath] ??
    "// No sample content available.";

  return (
    <div className="flex h-[calc(100vh-180px)] overflow-hidden rounded-lg border bg-card">
      <FileExplorer
        tree={treeSampleData}
        branches={branchesSampleData}
        selected={selectedPath}
        onSelect={setSelectedPath}
      />

      <PreviewPane
        path={selectedPath}
        content={content}
      />
    </div>
  );
}