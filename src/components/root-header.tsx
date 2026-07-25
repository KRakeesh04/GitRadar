import { useRepositories } from "#/hooks/useRepositories";
import { useSyncRepositories } from "#/hooks/useSync";
import { useState } from "react";
import { toast } from "sonner";
import { SearchBar } from "./searchbar";
import { Button } from "./ui/button";
import { Bell, GitPullRequest, RefreshCcw } from "lucide-react";

export function RootHeader({ onAddRootPath }: { onAddRootPath: () => void }) {
  const repositoriesQuery = useRepositories();
  const syncMutation = useSyncRepositories();
  const repositoryIds = (repositoriesQuery.data ?? []).map(repository => repository.id);
  const [isSyncingAll, setIsSyncingAll] = useState(false);

  const syncAllRepositories = () => {
    if (isSyncingAll || syncMutation.isPending || repositoryIds.length === 0) return;

    setIsSyncingAll(true);
    syncMutation.mutate(repositoryIds, {
      onSuccess: () => toast.success('All repositories synced'),
      onError: error => toast.error(error instanceof Error ? error.message : String(error)),
      onSettled: () => setIsSyncingAll(false),
    });
  };

  return (
    <header className="shrink-0 bg-card p-4 text-card-foreground">
      <div className="mt-2 flex gap-2">
        <SearchBar placeholder="Search repos, commits, files..." className="min-w-sm" />
        <div className="ml-auto flex items-center gap-3">
          <Button
            variant="default"
            className="ml-2 cursor-pointer border border-input bg-background/20 text-foreground hover:bg-(--brand) hover:text-background"
            onClick={syncAllRepositories}
            disabled={isSyncingAll || repositoriesQuery.isPending || repositoryIds.length === 0}
            aria-label="Sync all repositories"
            title="Sync all repositories"
          >
            <RefreshCcw className={isSyncingAll ? 'h-5 w-5 animate-spin' : 'h-5 w-5'} />
          </Button>
          <Button
            variant="default"
            className="cursor-pointer border border-input bg-background/20 text-foreground hover:bg-(--brand) hover:text-background"
          >
            <GitPullRequest className="h-5 w-5" />
          </Button>
          <Button
            variant="default"
            className="cursor-pointer border border-input bg-background/20 text-foreground hover:bg-(--brand) hover:text-background"
          >
            <Bell className="h-5 w-5" />
          </Button>
          <Button
            variant="default"
            className="cursor-pointer bg-(--brand) text-white hover:bg-(--brand-hover)"
            onClick={onAddRootPath}
          >
            + Add Root Path
          </Button>
        </div>
      </div>
    </header>
  );
}
