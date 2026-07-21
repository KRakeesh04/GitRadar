import * as React from 'react';
import { cn } from '#/lib/utils';

export function AlertDialog({
  open,
  onOpenChange,
  children,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  children: React.ReactNode;
}) {
  if (!open) return null;

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/40 p-4"
      role="presentation"
    >
      <div
        role="alertdialog"
        aria-modal="true"
        className="w-full max-w-md rounded-xl border bg-card p-6 text-card-foreground shadow-xl"
      >
        {children}
      </div>
      <button
        className="absolute inset-0 -z-10 cursor-default"
        aria-label="Close dialog"
        onClick={() => onOpenChange(false)}
      />
    </div>
  );
}

export function AlertDialogTitle({ className, ...props }: React.ComponentProps<'h2'>) {
  return <h2 className={cn('text-lg font-semibold', className)} {...props} />;
}

export function AlertDialogDescription({ className, ...props }: React.ComponentProps<'p'>) {
  return <p className={cn('mt-2 text-sm text-muted-foreground', className)} {...props} />;
}
