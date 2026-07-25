import { AlertTriangle } from "lucide-react";
import { Button } from "../ui/button";
import type { RootPath } from "#/routes/root-paths";

export function DeleteRootPathDialog({
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