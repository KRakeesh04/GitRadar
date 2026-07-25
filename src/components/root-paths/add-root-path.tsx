import { Folder, FolderPlus, X } from "lucide-react";
import { useEffect, useState } from "react";
import { Button } from "../ui/button";
import { Input } from "../ui/input";
import { open } from "@tauri-apps/plugin-dialog";
import { getErrorMessage } from "./utils";

export function AddRootPathPopover({
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
