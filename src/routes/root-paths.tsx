import { useEffect, useMemo, useState } from 'react';
import { createFileRoute, Link } from '@tanstack/react-router';
import { open } from '@tauri-apps/plugin-dialog';
import { toast } from 'sonner';
import {
  AlertTriangle,
  ChevronDown,
  ChevronRight,
  Folder,
  FolderPlus,
  GitBranch,
  HardDrive,
  RefreshCw,
  Trash2,
  X,
} from 'lucide-react';
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '#/components/ui/collapsible';
import { Button } from '#/components/ui/button';
import { Input } from '#/components/ui/input';
import { Skeleton } from '#/components/ui/skeleton';
import { Switch } from '#/components/ui/switch';
import {
  consumeAddRootPathPopoverRequest,
  OPEN_ADD_ROOT_PATH_EVENT,
  requestAddRootPathPopover,
} from '#/lib/root-path-actions';
import {
  useAddTrackedRoot,
  useDeleteTrackedRoot,
  useRepositories,
  useRescanTrackedRoots,
  useToggleTrackedRoot,
  useTrackedRoots,
} from '#/hooks/useTrackedRoots';
import type { Repository, TrackedRoot } from '#/lib/tauri/tracked-roots';

export const Route = createFileRoute('/root-paths')({ component: RouteComponent });

type RootRepository = Repository;

type RootPath = TrackedRoot & {
  name: string;
  updatedLabel: string;
  repos: RootRepository[];
};

function RouteComponent() {
  const rootsQuery = useTrackedRoots();
  const repositoriesQuery = useRepositories();
  const addMutation = useAddTrackedRoot();
  const rescanMutation = useRescanTrackedRoots();
  const toggleMutation = useToggleTrackedRoot();
  const deleteMutation = useDeleteTrackedRoot();
  const [expandedPathId, setExpandedPathId] = useState<number | null>(null);
  const [isAddPopoverOpen, setIsAddPopoverOpen] = useState(false);
  const [rootToDelete, setRootToDelete] = useState<RootPath | null>(null);

  const rootPaths = useMemo(
    () =>
      (rootsQuery.data ?? []).map(root => ({
        ...root,
        name: getPathLabel(root.path),
        updatedLabel: formatUpdatedAt(root.updatedAt),
        repos: (repositoriesQuery.data ?? []).filter(repo => repo.rootId === root.id),
      })),
    [repositoriesQuery.data, rootsQuery.data]
  );

  useEffect(() => {
    const openPopover = () => setIsAddPopoverOpen(true);
    window.addEventListener(OPEN_ADD_ROOT_PATH_EVENT, openPopover);
    if (consumeAddRootPathPopoverRequest()) openPopover();
    return () => window.removeEventListener(OPEN_ADD_ROOT_PATH_EVENT, openPopover);
  }, []);

  const allRepos = rootPaths.flatMap(root => root.repos);
  const cleanCount = allRepos.filter(repo => !repo.isDirty).length;
  const modifiedCount = allRepos.filter(repo => repo.isDirty).length;

  const addRootPath = async (path: string) => {
    await addMutation.mutateAsync(path);
    await rescanMutation.mutateAsync();
    setIsAddPopoverOpen(false);
    toast.success('Root path added');
  };

  const deleteRootPath = () => {
    if (!rootToDelete) return;
    deleteMutation.mutate(rootToDelete.id, {
      onSuccess: () => {
        setRootToDelete(null);
        toast.success('Root path deleted');
      },
      onError: error => toast.error(getErrorMessage(error)),
    });
  };

  const rescanRootPaths = () => {
    rescanMutation.mutate(undefined, {
      onSuccess: () => toast.success('Root paths rescanned'),
      onError: error => toast.error(getErrorMessage(error)),
    });
  };

  return (
    <div className="flex h-full w-full flex-col gap-4 overflow-y-auto bg-muted/20 py-4">
      <div className="flex flex-row items-center gap-3 px-10 lg:px-20">
        <div className="flex h-10 w-10 items-center justify-center rounded-md border bg-(--brand)/20 text-(--brand)">
          <HardDrive size={28} />
        </div>
        <div className="flex flex-col gap-1">
          <span className="text-2xl font-medium">Root Paths</span>
          <span className="text-muted-foreground">
            Manage the directories GitRadar scans for Git repositories
          </span>
        </div>
        <div className="ml-auto flex gap-2">
          <Button
            className="cursor-pointer bg-(--brand) py-4 text-white hover:bg-(--brand-hover)"
            onClick={() => requestAddRootPathPopover()}
          >
            + Add Path
          </Button>
        </div>
      </div>

      <div className="flex w-full gap-4 border-b px-10 pb-4 lg:px-20">
        <Stat value={rootPaths.length} label="Total Root Paths" />
        <Stat value={allRepos.length} label="Total Repositories" />
        <Stat value={cleanCount} label="Clean Repositories" />
        <Stat value={modifiedCount} label="Modified Repositories" />
      </div>

      <div className="mx-auto flex w-full max-w-5xl flex-col gap-4 px-10 pb-6 lg:px-20">
        {rootsQuery.isPending || repositoriesQuery.isPending ? (
          <LoadingCards />
        ) : rootsQuery.isError || repositoriesQuery.isError ? (
          <ErrorState
            error={rootsQuery.error ?? repositoriesQuery.error}
            onRetry={() => void Promise.all([rootsQuery.refetch(), repositoriesQuery.refetch()])}
          />
        ) : rootPaths.length === 0 ? (
          <EmptyState />
        ) : (
          rootPaths.map(rootPath => (
            <RootPathCard
              key={rootPath.id}
              rootPath={rootPath}
              isExpanded={expandedPathId === rootPath.id}
              isToggling={
                toggleMutation.isPending && toggleMutation.variables.path === rootPath.path
              }
              isRescanning={rescanMutation.isPending}
              onToggle={() =>
                setExpandedPathId(current => (current === rootPath.id ? null : rootPath.id))
              }
              onEnabledChange={enabled =>
                toggleMutation.mutate(
                  { path: rootPath.path, enabled },
                  { onError: error => toast.error(getErrorMessage(error)) }
                )
              }
              onRescan={rescanRootPaths}
              onDelete={() => setRootToDelete(rootPath)}
            />
          ))
        )}
      </div>

      <AddRootPathPopover
        open={isAddPopoverOpen}
        onOpenChange={setIsAddPopoverOpen}
        onAdd={addRootPath}
        isSaving={addMutation.isPending}
      />
      <DeleteRootPathDialog
        rootPath={rootToDelete}
        isDeleting={deleteMutation.isPending}
        onCancel={() => setRootToDelete(null)}
        onDelete={deleteRootPath}
      />
    </div>
  );
}

function Stat({ value, label }: { value: number; label: string }) {
  return (
    <div className="flex w-full flex-col items-center gap-1 rounded-md border border-border p-4">
      <span className="text-lg font-medium">{value}</span>
      <span className="text-sm text-muted-foreground">{label}</span>
    </div>
  );
}

function RootPathCard({
  rootPath,
  isExpanded,
  isToggling,
  isRescanning,
  onToggle,
  onEnabledChange,
  onRescan,
  onDelete,
}: {
  rootPath: RootPath;
  isExpanded: boolean;
  isToggling: boolean;
  isRescanning: boolean;
  onToggle: () => void;
  onEnabledChange: (enabled: boolean) => void;
  onRescan: () => void;
  onDelete: () => void;
}) {
  const cleanCount = rootPath.repos.filter(repo => !repo.isDirty).length;
  const modifiedCount = rootPath.repos.filter(repo => repo.isDirty).length;

  return (
    <Collapsible open={isExpanded}>
      <div className="overflow-hidden rounded-xl border border-border bg-card shadow-sm">
        <div className="flex gap-3 px-4 py-4">
          <div className="flex h-12 w-12 shrink-0 items-center justify-center rounded-lg bg-blue-500/10 text-blue-600">
            <Folder className="h-6 w-6" />
          </div>
          <div className="min-w-0 flex-1">
            <div className="flex items-start gap-3">
              <div className="min-w-0">
                <h2 className="truncate text-lg font-semibold leading-6">{rootPath.name}</h2>
                <p className="truncate font-mono text-sm text-muted-foreground">{rootPath.path}</p>
              </div>
              <div className="ml-auto flex shrink-0 items-center gap-1 text-muted-foreground">
                <Switch
                  checked={rootPath.enabled}
                  onChange={event => onEnabledChange(event.currentTarget.checked)}
                  disabled={isToggling}
                  aria-label={`Enable ${rootPath.name}`}
                />
                <Button
                  variant="ghost"
                  size="icon-sm"
                  aria-label={`Refresh ${rootPath.name}`}
                  onClick={onRescan}
                  disabled={isRescanning}
                >
                  <RefreshCw className={isRescanning ? 'h-4 w-4 animate-spin' : 'h-4 w-4'} />
                </Button>
                <Button
                  variant="ghost"
                  size="icon-sm"
                  aria-label={`Delete ${rootPath.name}`}
                  onClick={onDelete}
                >
                  <Trash2 className="h-4 w-4" />
                </Button>
              </div>
            </div>
            <div className="mt-3 flex flex-wrap items-center gap-x-4 gap-y-2 text-xs text-muted-foreground">
              <span className="flex items-center gap-1.5">
                <span className="h-2 w-2 rounded-full bg-emerald-500" />
                {cleanCount} clean
              </span>
              <span className="flex items-center gap-1.5">
                <span className="h-2 w-2 rounded-full bg-amber-500" />
                {modifiedCount} modified
              </span>
              <span>{rootPath.repos.length} repos</span>
              <span className="ml-auto flex items-center gap-1">
                <RefreshCw className="h-3.5 w-3.5" />
                {rootPath.updatedLabel}
              </span>
            </div>
          </div>
        </div>
        {rootPath.repos.length > 0 ? (
          <>
            <CollapsibleTrigger
              render={
                <button
                  type="button"
                  className="flex w-full items-center border-t px-4 py-3 text-left text-sm text-muted-foreground transition-colors hover:bg-muted/50"
                  onClick={onToggle}
                >
                  <span>{rootPath.repos.length} repositories</span>
                  {isExpanded ? (
                    <ChevronDown className="ml-auto h-4 w-4" />
                  ) : (
                    <ChevronRight className="ml-auto h-4 w-4" />
                  )}
                </button>
              }
            />
            {isExpanded ? (
              <CollapsibleContent>
                <div className="border-t px-3 py-2">
                  {rootPath.repos.map(repo => (
                    <RepositoryRow key={repo.id} repo={repo} />
                  ))}
                </div>
              </CollapsibleContent>
            ) : null}
          </>
        ) : (
          <div className="flex items-center gap-2 px-4 pb-4 text-sm text-muted-foreground">
            <Folder className="h-4 w-4" />
            No repositories found
            <span className="ml-auto flex items-center gap-1 text-xs">
              <RefreshCw className="h-3.5 w-3.5" />
              {rootPath.updatedLabel}
            </span>
          </div>
        )}
      </div>
    </Collapsible>
  );
}

function RepositoryRow({ repo }: { repo: RootRepository }) {
  return (
    <Link
      to="/repository/$id"
      params={{ id: String(repo.id) }}
      className="flex min-h-10 items-center gap-3 rounded-md px-3 py-2 text-sm transition-colors hover:bg-muted"
    >
      <span
        className={
          !repo.isDirty
            ? 'h-2 w-2 shrink-0 rounded-full bg-emerald-500'
            : 'h-2 w-2 shrink-0 rounded-full bg-amber-500'
        }
      />
      <span className="min-w-0 flex-1 truncate font-medium">{repo.name}</span>
      <span className="hidden items-center gap-1 text-muted-foreground sm:flex">
        <GitBranch className="h-3.5 w-3.5" />
        {repo.headBranch ?? 'Detached'}
      </span>
      <span className="hidden w-24 text-right text-xs text-muted-foreground sm:block">
        {formatUpdatedAt(repo.updatedAt)}
      </span>
      <ChevronRight className="h-4 w-4 text-muted-foreground" />
    </Link>
  );
}

function AddRootPathPopover({
  open: isOpen,
  onOpenChange,
  onAdd,
  isSaving,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onAdd: (path: string) => Promise<void>;
  isSaving: boolean;
}) {
  const [path, setPath] = useState('');
  const [error, setError] = useState<string | null>(null);
  const [isSelecting, setIsSelecting] = useState(false);

  useEffect(() => {
    if (!isOpen) {
      setPath('');
      setError(null);
      setIsSelecting(false);
    }
  }, [isOpen]);

  if (!isOpen) return null;

  const selectFolder = async () => {
    setError(null);
    setIsSelecting(true);
    try {
      const selected = await open({ directory: true, multiple: false, title: 'Select root path' });
      if (typeof selected === 'string') setPath(selected);
    } catch (selectError) {
      setError(getErrorMessage(selectError));
    } finally {
      setIsSelecting(false);
    }
  };

  const submit = async () => {
    const trimmedPath = path.trim();
    if (!trimmedPath) {
      setError('Select a folder before adding a root path.');
      return;
    }
    setError(null);
    try {
      await onAdd(trimmedPath);
    } catch (saveError) {
      setError(getErrorMessage(saveError));
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-start justify-center bg-black/20 px-4 pt-24 backdrop-blur-xs">
      <div className="w-full max-w-lg rounded-lg border border-border bg-popover p-5 text-popover-foreground shadow-xl">
        <div className="flex items-start gap-3">
          <div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-md bg-(--brand)/15 text-(--brand)">
            <FolderPlus className="h-5 w-5" />
          </div>
          <div className="min-w-0 flex-1">
            <h2 className="text-base font-semibold">Add root path</h2>
            <p className="mt-1 text-sm text-muted-foreground">
              Choose a directory GitRadar should scan for repositories.
            </p>
          </div>
          <Button
            variant="ghost"
            size="icon-sm"
            aria-label="Close"
            onClick={() => onOpenChange(false)}
          >
            <X className="h-4 w-4" />
          </Button>
        </div>
        <div className="mt-5 grid gap-4">
          <label className="grid gap-1.5 text-sm font-medium">
            Root folder
            <div className="flex gap-2">
              <Input
                className="h-9 font-mono"
                value={path}
                onChange={event => setPath(event.target.value)}
                placeholder="/home/user/projects"
              />
              <Button
                variant="outline"
                className="h-9"
                onClick={selectFolder}
                disabled={isSelecting}
              >
                <Folder className="h-4 w-4" />
                {isSelecting ? 'Opening' : 'Browse'}
              </Button>
            </div>
          </label>
          {error ? (
            <div className="rounded-md border border-red-500/30 bg-red-500/10 px-3 py-2 text-sm text-red-600">
              {error}
            </div>
          ) : null}
        </div>
        <div className="mt-5 flex justify-end gap-2">
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            Cancel
          </Button>
          <Button
            className="bg-(--brand) text-white hover:bg-(--brand-hover)"
            onClick={submit}
            disabled={isSaving}
          >
            {isSaving ? 'Adding' : 'Add root path'}
          </Button>
        </div>
      </div>
    </div>
  );
}

function DeleteRootPathDialog({
  rootPath,
  isDeleting,
  onCancel,
  onDelete,
}: {
  rootPath: RootPath | null;
  isDeleting: boolean;
  onCancel: () => void;
  onDelete: () => void;
}) {
  if (!rootPath) return null;
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/30 p-4">
      <div
        role="alertdialog"
        aria-modal="true"
        className="w-full max-w-md rounded-xl border border-red-500/40 bg-card p-6 shadow-xl"
      >
        <div className="flex gap-3">
          <div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-red-500/10 text-red-600">
            <AlertTriangle className="h-5 w-5" />
          </div>
          <div>
            <h2 className="font-semibold">Delete "{rootPath.name}"?</h2>
            <p className="mt-1 text-sm text-muted-foreground">
              This removes the root path from GitRadar. Repositories inside it won't be deleted from
              disk.
            </p>
            <p className="mt-2 text-sm text-amber-600">
              {rootPath.repos.length} tracked repos will no longer be monitored.
            </p>
          </div>
        </div>
        <div className="mt-6 flex justify-end gap-2">
          <Button variant="outline" onClick={onCancel}>
            Cancel
          </Button>
          <Button variant="destructive" onClick={onDelete} disabled={isDeleting}>
            {isDeleting ? 'Deleting' : 'Delete path'}
          </Button>
        </div>
      </div>
    </div>
  );
}

function LoadingCards() {
  return (
    <>
      {[1, 2, 3].map(item => (
        <Skeleton key={item} className="h-36 w-full rounded-xl" />
      ))}
    </>
  );
}
function EmptyState() {
  return (
    <div className="rounded-xl border border-dashed border-border bg-card px-6 py-16 text-center">
      <Folder className="mx-auto h-10 w-10 text-muted-foreground" />
      <p className="mt-3 font-medium">No tracked folders</p>
      <p className="mt-1 text-sm text-muted-foreground">
        Add a folder to start discovering repositories.
      </p>
    </div>
  );
}
function ErrorState({ error, onRetry }: { error: unknown; onRetry: () => void }) {
  return (
    <div className="rounded-xl border border-red-500/30 bg-card px-6 py-16 text-center">
      <p className="text-sm text-red-600">Could not load tracked folders.</p>
      <p className="mx-auto mt-2 max-w-2xl break-words text-xs text-muted-foreground">
        {getErrorMessage(error)}
      </p>
      <Button className="mt-4" variant="outline" onClick={onRetry}>
        <RefreshCw /> Retry
      </Button>
    </div>
  );
}
function getPathLabel(path: string) {
  return path.split(/[\\/]/).filter(Boolean).at(-1) || path;
}
function formatUpdatedAt(value: string) {
  const date = new Date(value);
  return Number.isNaN(date.getTime()) ? value : date.toLocaleString();
}
function getErrorMessage(error: unknown) {
  return error instanceof Error ? error.message : String(error);
}
