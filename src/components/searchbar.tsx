import { Search } from "lucide-react";
import { Input } from "./ui/input";
import { cn } from "#/lib/utils";

export function SearchBar({ placeholder, className }: { placeholder: string, className?: string }) {
  return (
    <div className={cn("relative max-w-lg flex-1 mr-auto", className)}>
      <Search className="pointer-events-none absolute left-3 top-1/2 h-5 w-5 -translate-y-1/2 text-muted-foreground" />
      <Input
        type="text"
        placeholder={placeholder}
        className="pl-10 pr-4 bg-background text-foreground placeholder:text-muted-foreground border border-input focus-visible:outline-none focus-visible:ring-2 w-full rounded-md text-sm shadow-sm transition-all focus-visible:ring-(--brand)"
      />
    </div>
  );
}