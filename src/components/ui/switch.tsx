import * as React from 'react';
import { cn } from '#/lib/utils';

export function Switch({ className, ...props }: React.ComponentProps<'input'>) {
  return (
    <input
      type="checkbox"
      role="switch"
      className={cn(
        'relative h-5 w-9 cursor-pointer appearance-none rounded-full bg-muted transition-colors',
        'before:absolute before:left-0.5 before:top-0.5 before:h-4 before:w-4 before:rounded-full before:bg-background before:transition-transform',
        'checked:bg-primary checked:before:translate-x-4 disabled:cursor-not-allowed disabled:opacity-50',
        className
      )}
      {...props}
    />
  );
}
