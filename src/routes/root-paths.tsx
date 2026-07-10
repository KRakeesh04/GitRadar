import { Button } from '#/components/ui/button';
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '#/components/ui/collapsible';
import { createFileRoute, Link } from '@tanstack/react-router';
import {
  AlertTriangle,
  ChevronDown,
  ChevronRight,
  Folder,
  GitBranch,
  HardDrive,
  Pencil,
  RefreshCw,
  Save,
  Trash2,
  X,
} from 'lucide-react';
import { useState } from 'react';

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
  const [expandedPathId, setExpandedPathId] = useState<string | null>('work-apps');
  const [editingPathId, setEditingPathId] = useState<string | null>(null);
  const [deletingPathId, setDeletingPathId] = useState<string | null>(null);

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
          >
            + Add Path
          </Button>
        </div>
      </div>

      <div className="flex w-full gap-4 border-b px-10 pb-4 lg:px-20">
        <div className="flex w-full flex-col items-center gap-1 rounded-md border border-border p-4">
          <span className="text-lg font-medium">{rootPathsData.length}</span>
          <span className="text-sm text-muted-foreground">Total Root Paths</span>
        </div>
        <div className="flex w-full flex-col items-center gap-1 rounded-md border border-border p-4">
          <span className="text-lg font-medium">
            {rootPathsData.reduce((acc, rootPath) => acc + rootPath.repos.length, 0)}
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

      <div className="mx-auto flex w-full max-w-5xl flex-col gap-4">
        {rootPathsData.map(rootPath => {
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
