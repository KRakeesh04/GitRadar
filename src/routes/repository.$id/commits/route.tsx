import { CommitGraphPage } from '#/components/commit-previewer/commit-graph-page';
import { Button } from '#/components/ui/button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '#/components/ui/dropdown-menu';
import { useBranchesByRepoId } from '#/hooks/useRepositories';
import { createFileRoute, Link, Outlet } from '@tanstack/react-router';
import { ChevronsUpDown, GitBranch } from 'lucide-react';
import { useState } from 'react';

export const Route = createFileRoute('/repository/$id/commits')({
  component: RouteComponent,
});

function RouteComponent() {
  const { id } = Route.useParams();
  const numericId = Number(id);
  const [activeBranch, setActiveBranch] = useState<string>('All Branches');
  const branches = useBranchesByRepoId(numericId).data ?? [];

  return (
    <div className="flex min-h-0 flex-col gap-4">
      <div className="rounded-md border border-border bg-card p-3 text-sm">
        <DropdownMenu>
          <DropdownMenuTrigger
            render={
              <Button variant="outline" className="max-w-100 w-full cursor-pointer text-left">
                <GitBranch className="mr-2 h-4 w-4" />
                <span>{activeBranch}</span>
                <ChevronsUpDown className="ml-auto h-4 w-4 opacity-50" />
              </Button>
            }
          />
          <DropdownMenuContent>
            <DropdownMenuItem
              key="All"
              onClick={() => setActiveBranch('All Branches')}
              className={`cursor-pointer ${activeBranch === 'All Branches' ? 'bg-(--brand-low) focus:bg-(--brand-low)' : 'focus:bg-muted '}`}
            >
              All Branches
            </DropdownMenuItem>
            {branches.map(branch => (
              <Link
                key={branch.name}
                to="/repository/$id/commits/$branch"
                params={{
                  id,
                  branch: branch.name,
                }}
                activeProps={{ className: 'text-black font-bold' }}
              >
                <DropdownMenuItem
                  onClick={() => {
                    setActiveBranch(branch.name);
                  }}
                  className={`cursor-pointer ${activeBranch === branch.name ? 'bg-(--brand-low) focus:bg-(--brand-low)' : 'focus:bg-muted '}`}
                >
                  {branch.name}
                </DropdownMenuItem>
              </Link>
            ))}
          </DropdownMenuContent>
        </DropdownMenu>
      </div>

      {activeBranch === 'All Branches' ? <CommitGraphPage repoId={id} /> : null}
      <Outlet />
    </div>
  );
}
