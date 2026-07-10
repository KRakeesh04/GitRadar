import { CommitDiffViewer } from '#/components/commit-previewer/commit-diff-viewer';
import type { CommitDiff } from '#/components/commit-previewer/types';
import { Button } from '#/components/ui/button';
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '#/components/ui/dropdown-menu';
import { createFileRoute, Link, Outlet } from '@tanstack/react-router'
import { ChevronsUpDown, GitBranch } from 'lucide-react';
import { useState } from 'react';

export const Route = createFileRoute('/repository/$id/commits')({
  component: RouteComponent,
})

const branchesSampleData = ["main", "dev", "feature-1", "feature-2"];

function RouteComponent() {
  const { id } = Route.useParams();
  const [activeBranch, setActiveBranch] = useState<string>("All");

  return (
    <div className="flex flex-col gap-4">
      <div className="rounded-md border border-border bg-card p-3 text-sm">
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
            <DropdownMenuItem
              key={"All"}
              onClick={() => setActiveBranch("All")}
              className={`cursor-pointer ${activeBranch === "All" ? 'bg-(--brand-low) focus:bg-(--brand-low)' : 'focus:bg-muted '}`}
            >
              All Branches
            </DropdownMenuItem>
            {branchesSampleData.map((branch) => (
              <Link
                to="/repository/$id/commits/$branch"
                params={{
                  id: id,
                  branch: branch,
                }}
                activeProps={{ className: 'text-black font-bold' }}
              >
                <DropdownMenuItem
                  key={branch}
                  onClick={() => {
                    setActiveBranch(branch);
                  }}
                  className={`cursor-pointer ${activeBranch === branch ? 'bg-(--brand-low) focus:bg-(--brand-low)' : 'focus:bg-muted '}`}
                >
                  {branch}
                </DropdownMenuItem>
              </Link>
            ))}
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
      {activeBranch === "All" && (
        <div className="text-sm text-muted-foreground">
          Showing commits for all branches. To view commits for a specific branch, please select a branch from the dropdown menu above.
          <CommitDiffViewer diff={mockCommitDiff} />
        </div>
      )}
      <Outlet />
    </div>
  )
}

const mockCommitDiff: CommitDiff = {
  commit_hash: "6e9c17c0d20f2d1e4d6f87f80c9a4a9a7d9e11ef",

  files: [
    {
      old_path: "src/main.rs",
      new_path: "src/main.rs",
      change_type: "Modified",

      additions: 5,
      deletions: 3,

      hunks: [
        {
          old_start: 12,
          old_lines: 8,
          new_start: 12,
          new_lines: 10,

          lines: [
            {
              line_type: "Context",
              old_line_number: 12,
              new_line_number: 12,
              content: "fn main() {",
            },
            {
              line_type: "Removed",
              old_line_number: 13,
              new_line_number: null,
              content: "    println!(\"Hello\");",
            },
            {
              line_type: "Added",
              old_line_number: null,
              new_line_number: 13,
              content: "    println!(\"Hello GitRadar\");",
            },
            {
              line_type: "Context",
              old_line_number: 14,
              new_line_number: 14,
              content: "}",
            },
          ],
        },
      ],
    },

    {
      old_path: null,
      new_path: "src/utils/logger.rs",
      change_type: "Added",

      additions: 18,
      deletions: 0,

      hunks: [
        {
          old_start: 0,
          old_lines: 0,
          new_start: 1,
          new_lines: 18,

          lines: [
            {
              line_type: "Added",
              old_line_number: null,
              new_line_number: 1,
              content: "pub fn log(message: &str) {",
            },
            {
              line_type: "Added",
              old_line_number: null,
              new_line_number: 2,
              content: "    println!(\"{}\", message);",
            },
            {
              line_type: "Added",
              old_line_number: null,
              new_line_number: 3,
              content: "}",
            },
          ],
        },
      ],
    },

    {
      old_path: "src/old_config.rs",
      new_path: "src/old_config.rs",
      change_type: "Deleted",

      additions: 0,
      deletions: 12,

      hunks: [
        {
          old_start: 1,
          old_lines: 12,
          new_start: 0,
          new_lines: 0,

          lines: [
            {
              line_type: "Removed",
              old_line_number: 1,
              new_line_number: null,
              content: "pub const HOST: &str = \"localhost\";",
            },
            {
              line_type: "Removed",
              old_line_number: 2,
              new_line_number: null,
              content: "pub const PORT: u16 = 8080;",
            },
          ],
        },
      ],
    },

    {
      old_path: "src/git/mod.rs",
      new_path: "src/git/repository.rs",
      change_type: "Renamed",

      additions: 2,
      deletions: 2,

      hunks: [
        {
          old_start: 35,
          old_lines: 4,
          new_start: 35,
          new_lines: 4,

          lines: [
            {
              line_type: "Context",
              old_line_number: 35,
              new_line_number: 35,
              content: "pub fn open_repository() {",
            },
            {
              line_type: "Removed",
              old_line_number: 36,
              new_line_number: null,
              content: "    println!(\"Opening...\");",
            },
            {
              line_type: "Added",
              old_line_number: null,
              new_line_number: 36,
              content: "    tracing::info!(\"Opening repository\");",
            },
            {
              line_type: "Context",
              old_line_number: 37,
              new_line_number: 37,
              content: "}",
            },
          ],
        },
      ],
    },
  ],
};
