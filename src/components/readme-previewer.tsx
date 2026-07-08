import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import rehypeHighlight from "rehype-highlight";

import "github-markdown-css/github-markdown.css";
import "highlight.js/styles/github-dark.css";

import { Card } from "./ui/card";

export function ReadmeViewer({ content }: { content: string }) {
  return (
    <Card className="overflow-hidden">
      <article
        className="markdown-body p-6"
        style={{
          background: "transparent",
          color: "inherit",
        }}
      >
        <ReactMarkdown
          remarkPlugins={[remarkGfm]}
          rehypePlugins={[rehypeHighlight]}
        >
          {content}
        </ReactMarkdown>
      </article>
    </Card>
  );
}