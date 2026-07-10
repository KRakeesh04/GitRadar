import { Button } from '#/components/ui/button';
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '#/components/ui/collapsible';
import { Input } from '#/components/ui/input';
import {
  consumeAddRootPathPopoverRequest,
  OPEN_ADD_ROOT_PATH_EVENT,
  requestAddRootPathPopover,
} from '#/lib/root-path-actions';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { createFileRoute, Link } from '@tanstack/react-router';
import {
  AlertTriangle,
  ChevronDown,
  ChevronRight,
  Folder,
  FolderPlus,
  GitBranch,
  HardDrive,
  Pencil,
  RefreshCw,
  Save,
  Trash2,
  X,
} from 'lucide-react';
import { useEffect, useState } from 'react';

export const Route = createFileRoute('/root-paths')({
  component: RouteComponent,
});

type RepositoryStatus = 'clean' | 'modified';

type RootRepository = {
  id: number;
  name: string;
  headBranch: string;
  status: RepositoryStatus;
  updatedLabel: string;
};

type RootPath = {
  id: string;
  name: string;
  path: string;
  updatedLabel: string;
  repos: RootRepository[];
};

type TrackedRootResponse = {
  id: number;
  path: string;
  is_enabled: boolean;
  updated_at: string;
};

const rootPathsData: RootPath[] = [
  {
    id: 'main-projects',
    name: 'Main Projects',
    path: '/home/user/projects',
    updatedLabel: '12 min ago',
    repos: [
      {
        id: 1,
        name: 'gitradar',
        headBranch: 'main',
        status: 'clean',
        updatedLabel: '2 days ago',
      },
      {
        id: 2,
        name: 'web-dashboard',
        headBranch: 'develop',
        status: 'modified',
        updatedLabel: '4 days ago',
      },
      {
        id: 3,
        name: 'api-server',
        headBranch: 'feature/auth',
        status: 'modified',
        updatedLabel: '5 days ago',
      },
    ],
  },
  {
    id: 'work-apps',
    name: 'Work Apps',
    path: '/home/user/work/apps',
    updatedLabel: '1 h ago',
    repos: [
      {
        id: 4,
        name: 'mobile-app',
        headBranch: 'main',
        status: 'clean',
        updatedLabel: '5 days ago',
      },
      {
        id: 5,
        name: 'design-system',
        headBranch: 'main',
        status: 'clean',
        updatedLabel: '1 week ago',
      },
    ],
  },
  {
    id: 'experiments',
    name: 'Experiments',
    path: '/home/user/experiments',
    updatedLabel: '2d ago',
    repos: [],
  },
];

function RouteComponent() {
  const [rootPaths, setRootPaths] = useState<RootPath[]>(rootPathsData);
  const [expandedPathId, setExpandedPathId] = useState<string | null>('work-apps');
  const [editingPathId, setEditingPathId] = useState<string | null>(null);
  const [deletingPathId, setDeletingPathId] = useState<string | null>(null);
  const [isAddPopoverOpen, setIsAddPopoverOpen] = useState(false);

  useEffect(() => {
    const loadRootPaths = async () => {
      try {
        const trackedRoots = await invoke<TrackedRootResponse[]>('get_all_tracked_root_paths');

        setRootPaths(
          trackedRoots.map(root => ({
            id: String(root.id),
            name: getPathLabel(root.path),
            path: root.path,
            updatedLabel: root.updated_at,
            repos: [],
          }))
        );
      } catch (error) {
        console.error('Failed to load tracked root paths', error);
      }
    };

    void loadRootPaths();
  }, []);

  useEffect(() => {
    const openPopover = () => setIsAddPopoverOpen(true);

    window.addEventListener(OPEN_ADD_ROOT_PATH_EVENT, openPopover);

    if (consumeAddRootPathPopoverRequest()) {
      openPopover();
    }

    return () => window.removeEventListener(OPEN_ADD_ROOT_PATH_EVENT, openPopover);
  }, []);

  const addRootPath = async ({ path, name }: { path: string; name: string }) => {
    const id = await invoke<number>('add_tracked_root_path', { path });

    const nextRootPath: RootPath = {
      id: String(id),
      name,
      path,
      updatedLabel: 'just now',
      repos: [],
    };

    setRootPaths(current => [nextRootPath, ...current]);
    setExpandedPathId(nextRootPath.id);
    setEditingPathId(null);
    setDeletingPathId(null);
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
            variant="default"
            className="cursor-pointer bg-(--brand) py-4 text-white hover:bg-(--brand-hover)"
            onClick={() => requestAddRootPathPopover()}
          >
            + Add Path
          </Button>
        </div>
      </div>

      <div className="flex w-full gap-4 border-b px-10 pb-4 lg:px-20">
        <div className="flex w-full flex-col items-center gap-1 rounded-md border border-border p-4">
          <span className="text-lg font-medium">{rootPaths.length}</span>
          <span className="text-sm text-muted-foreground">Total Root Paths</span>
        </div>
        <div className="flex w-full flex-col items-center gap-1 rounded-md border border-border p-4">
          <span className="text-lg font-medium">
            {rootPaths.reduce((acc, rootPath) => acc + rootPath.repos.length, 0)}
          </span>
          <span className="text-sm text-muted-foreground">Total Repositories</span>
        </div>
        <div className="flex w-full flex-col items-center gap-1 rounded-md border border-border p-4">
          <span className="text-lg font-medium">{3}</span>
          <span className="text-sm text-muted-foreground">Clean Repositories</span>
        </div>
        <div className="flex w-full flex-col items-center gap-1 rounded-md border border-border p-4">
          <span className="text-lg font-medium">{2}</span>
          <span className="text-sm text-muted-foreground">Modified Repositories</span>
        </div>
      </div>

      <div className="mx-auto flex w-full max-w-5xl flex-col gap-4 px-10 lg:px-20">
        {rootPaths.map(rootPath => {
          if (editingPathId === rootPath.id) {
            return (
              <EditRootPathCard
                key={rootPath.id}
                rootPath={rootPath}
                onCancel={() => setEditingPathId(null)}
                onSave={() => setEditingPathId(null)}
              />
            );
          }

          if (deletingPathId === rootPath.id) {
            return (
              <DeleteRootPathCard
                key={rootPath.id}
                rootPath={rootPath}
                onCancel={() => setDeletingPathId(null)}
                onDelete={() => setDeletingPathId(null)}
              />
            );
          }

          return (
            <RootPathCard
              key={rootPath.id}
              rootPath={rootPath}
              isExpanded={expandedPathId === rootPath.id}
              onToggle={() =>
                setExpandedPathId(current => (current === rootPath.id ? null : rootPath.id))
              }
              onEdit={() => {
                setDeletingPathId(null);
                setEditingPathId(rootPath.id);
              }}
              onDelete={() => {
                setEditingPathId(null);
                setDeletingPathId(rootPath.id);
              }}
            />
          );
        })}
      </div>

      <AddRootPathPopover
        open={isAddPopoverOpen}
        onOpenChange={setIsAddPopoverOpen}
        onAdd={addRootPath}
      />
    </div>
  );
}

function getPathLabel(path: string) {
  return path.split(/[\\/]/).filter(Boolean).at(-1) || path;
}

function AddRootPathPopover({
  open: isOpen,
  onOpenChange,
  onAdd,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onAdd: (rootPath: { path: string; name: string }) => Promise<void>;
}) {
  const [path, setPath] = useState('');
  const [name, setName] = useState('');
  const [error, setError] = useState<string | null>(null);
  const [isSelecting, setIsSelecting] = useState(false);
  const [isSaving, setIsSaving] = useState(false);

  useEffect(() => {
    if (!isOpen) {
      setPath('');
      setName('');
      setError(null);
      setIsSelecting(false);
      setIsSaving(false);
    }
  }, [isOpen]);

  if (!isOpen) {
    return null;
  }

  const selectFolder = async () => {
    setError(null);
    setIsSelecting(true);

    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: 'Select root path',
      });

      if (typeof selected === 'string') {
        setPath(selected);
        setName(current => current || getPathLabel(selected));
      }
    } catch (selectError) {
      setError(
        selectError instanceof Error ? selectError.message : 'Could not open folder picker.'
      );
    } finally {
      setIsSelecting(false);
    }
  };

  const submit = async () => {
    const trimmedPath = path.trim();
    const trimmedName = name.trim();

    if (!trimmedPath) {
      setError('Select a folder before adding a root path.');
      return;
    }

    setError(null);
    setIsSaving(true);

    try {
      await onAdd({
        path: trimmedPath,
        name: trimmedName || getPathLabel(trimmedPath),
      });
      onOpenChange(false);
    } catch (saveError) {
      setError(saveError instanceof Error ? saveError.message : 'Could not add root path.');
    } finally {
      setIsSaving(false);
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

          <label className="grid gap-1.5 text-sm font-medium">
            Label
            <Input
              className="h-9"
              value={name}
              onChange={event => setName(event.target.value)}
              placeholder="Projects"
            />
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

function RootPathCard({
  rootPath,
  isExpanded,
  onToggle,
  onEdit,
  onDelete,
}: {
  rootPath: RootPath;
  isExpanded: boolean;
  onToggle: () => void;
  onEdit: () => void;
  onDelete: () => void;
}) {
  const cleanCount = rootPath.repos.filter(repo => repo.status === 'clean').length;
  const modifiedCount = rootPath.repos.filter(repo => repo.status === 'modified').length;

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
                <Button variant="ghost" size="icon-sm" aria-label={`Refresh ${rootPath.name}`}>
                  <RefreshCw className="h-4 w-4" />
                </Button>
                <Button
                  variant="ghost"
                  size="icon-sm"
                  aria-label={`Edit ${rootPath.name}`}
                  onClick={onEdit}
                >
                  <Pencil className="h-4 w-4" />
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
          repo.status === 'clean'
            ? 'h-2 w-2 shrink-0 rounded-full bg-emerald-500'
            : 'h-2 w-2 shrink-0 rounded-full bg-amber-500'
        }
      />
      <span className="min-w-0 flex-1 truncate font-medium">{repo.name}</span>
      <span className="hidden items-center gap-1 text-muted-foreground sm:flex">
        <GitBranch className="h-3.5 w-3.5" />
        {repo.headBranch}
      </span>
      <span className="hidden w-24 text-right text-xs text-muted-foreground sm:block">
        {repo.updatedLabel}
      </span>
      <ChevronRight className="h-4 w-4 text-muted-foreground" />
    </Link>
  );
}

function EditRootPathCard({
  rootPath,
  onCancel,
  onSave,
}: {
  rootPath: RootPath;
  onCancel: () => void;
  onSave: () => void;
}) {
  return (
    <div className="rounded-xl border border-blue-500/60 bg-card p-5 shadow-sm">
      <div className="mb-4 text-sm font-medium uppercase text-blue-600">Edit root path</div>
      <div className="grid gap-3">
        <label className="grid gap-1.5 text-sm text-muted-foreground">
          Path
          <input
            className="h-10 rounded-md border bg-background px-3 font-mono text-sm text-foreground outline-none focus:border-blue-500"
            defaultValue={rootPath.path}
          />
        </label>
        <label className="grid gap-1.5 text-sm text-muted-foreground">
          Label
          <input
            className="h-10 rounded-md border bg-background px-3 text-sm text-foreground outline-none focus:border-blue-500"
            defaultValue={rootPath.name}
          />
        </label>
      </div>
      <div className="mt-4 flex justify-end gap-2">
        <Button variant="outline" onClick={onCancel}>
          <X className="h-4 w-4" />
          Cancel
        </Button>
        <Button className="bg-blue-600 text-white hover:bg-blue-700" onClick={onSave}>
          <Save className="h-4 w-4" />
          Save changes
        </Button>
      </div>
    </div>
  );
}

function DeleteRootPathCard({
  rootPath,
  onCancel,
  onDelete,
}: {
  rootPath: RootPath;
  onCancel: () => void;
  onDelete: () => void;
}) {
  return (
    <div className="rounded-xl border border-red-500/60 bg-card p-5 shadow-sm">
      <div className="flex gap-3">
        <div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-red-500/10 text-red-600">
          <AlertTriangle className="h-5 w-5" />
        </div>
        <div className="min-w-0 flex-1">
          <h2 className="font-semibold">Delete "{rootPath.name}"?</h2>
          <p className="mt-1 text-sm text-muted-foreground">
            This removes the root path from GitRadar. Repositories inside it won't be deleted from
            disk.
          </p>
          <p className="mt-2 text-sm text-amber-600">
            {rootPath.repos.length} tracked repos will no longer be monitored.
          </p>
        </div>
        <div className="flex shrink-0 items-center gap-2 self-end">
          <Button variant="outline" onClick={onCancel}>
            Cancel
          </Button>
          <Button variant="destructive" onClick={onDelete}>
            Delete path
          </Button>
        </div>
      </div>
    </div>
  );
}
