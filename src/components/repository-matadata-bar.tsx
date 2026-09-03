import { Clock, FileText, GitBranch, GitCommit, Mail, Star, Users, X } from "lucide-react";
import { useMemo, useState } from "react";
import { cn } from "#/lib/utils";
import { useContributors, useRepositoryById, useRepositoryLanguagesStats, useToggleRepositoryStarred } from "#/hooks/useRepositories";
import { useRepoFiles } from "#/hooks/useFiles";
import { useCommits } from "#/hooks/useCommits";
import type { Contributor, LanguageStatsResponse } from "#/lib/tauri/analytics";
import { HoverCard, HoverCardContent, HoverCardTrigger } from "./ui/hover-card";
import { openUrl } from "@tauri-apps/plugin-opener";
import { formatUpdatedAt } from "./root-paths/utils";

const languageColors: Record<string, string> = {
  TypeScript: "bg-blue-500", JavaScript: "bg-yellow-400", Rust: "bg-orange-500", Go: "bg-cyan-500",
  Python: "bg-green-500", Ruby: "bg-red-500", C: "bg-gray-500", Cpp: "bg-gray-500", "C++": "bg-gray-500",
  Java: "bg-red-600", PHP: "bg-purple-500", Swift: "bg-orange-400", Kotlin: "bg-purple-400", Dart: "bg-blue-400",
  Scala: "bg-red-400", Haskell: "bg-purple-600", Lua: "bg-blue-300", Perl: "bg-blue-600", R: "bg-blue-700",
  Shell: "bg-green-700", HTML: "bg-orange-300", CSS: "bg-blue-200", SQL: "bg-blue-800", Other: "bg-gray-500",
};

type LanguageDetail = { name: string; percentage: string };

function getLanguageDetails(stats: LanguageStatsResponse | undefined): LanguageDetail[] {
  if (!stats || stats.total_bytes <= 0) return [];

  const details = stats.languages.map((language) => ({
    name: language.language,
    percentage: (language.bytes / stats.total_bytes) * 100,
  }));
  const otherPercentage = details
    .filter((language) => language.percentage < 1)
    .reduce((total, language) => total + language.percentage, 0);
  const visible = details.filter((language) => language.percentage >= 1);

  if (otherPercentage > 0) visible.push({ name: "Other", percentage: otherPercentage });

  return visible
    .sort((a, b) => b.percentage - a.percentage)
    .map((language) => ({ name: language.name, percentage: language.percentage.toFixed(2) }));
}

function LanguageBar({ languages }: { languages: LanguageDetail[] }) {
  return (
    <div className="flex h-2 w-full min-w-50 max-w-100 overflow-hidden rounded-full bg-muted">
      {languages.map((language) => (
        <div
          key={language.name}
          className={languageColors[language.name] ?? languageColors.Other}
          style={{ width: `${language.percentage}%` }}
          title={`${language.name}: ${language.percentage}%`}
        />
      ))}
    </div>
  );
}

function getInitials(name: string, email: string) {
  const source = name.trim() || email.split("@")[0] || "?";
  const words = source.split(/\s+/).filter(Boolean);
  return (words.length > 1 ? `${words[0][0]}${words[1][0]}` : source.slice(0, 2)).toUpperCase();
}

function ContributorAvatar({ contributor, onClick }: { contributor?: Contributor; onClick?: () => void }) {
  const isMore = !contributor;
  return (
    <button
      type="button"
      className="flex min-w-16 flex-col items-center gap-1 rounded-md p-1 text-center transition-colors hover:bg-muted/70"
      title={isMore ? "Show all contributors" : undefined}
      onClick={onClick}
    >
      <div className={cn(
        "flex h-10 w-10 items-center justify-center rounded-full border text-xs font-semibold",
        isMore ? "border-dashed border-muted-foreground bg-muted text-muted-foreground" : "border-border bg-muted",
      )}>
        {isMore ? "+" : getInitials(contributor.name, contributor.email)}
      </div>
      <span className="max-w-24 truncate text-xs text-muted-foreground">{isMore ? "More" : contributor.name}</span>
    </button>
  );
}

function ContributorDetails({ contributor }: { contributor: Contributor }) {
  return (
    <div className="items-start justify-between gap-4 rounded-lg border p-3">
      <div className="flex min-w-0 items-start gap-3">
        <div className="flex h-9 w-9 shrink-0 items-center justify-center rounded-full bg-muted text-xs font-semibold">
          {getInitials(contributor.name, contributor.email)}
        </div>
        <div className="min-w-0">
          <p className="wrap-break-words font-medium">{contributor.name || "Unknown contributor"}</p>
          <button
            type="button"
            onClick={() => openUrl(`mailto:${contributor.email}`)}
            className="flex items-start gap-1 text-sm text-muted-foreground hover:text-foreground hover:underline cursor-pointer"
          >
            <Mail className="h-3.5 w-3.5" />
            <span>{contributor.email}</span>
          </button>
        </div>
      </div>
      <div className="shrink-0 text-center text-xs flex gap-3 mt-2">
        <p className="font-medium">{contributor.commitCount} commits</p>
        <p className="text-emerald-600">+{contributor.additions} additions</p>
        <p className="text-red-600">-{contributor.deletions} deletions</p>
      </div>
    </div>
  );
}

function ContributorsDialog({ contributors, onClose }: { contributors: Contributor[]; onClose: () => void }) {
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4" onMouseDown={onClose} role="presentation">
      <section
        className="flex max-h-[80vh] w-full max-w-2xl flex-col rounded-xl border bg-background p-5 shadow-xl"
        role="dialog"
        aria-modal="true"
        aria-labelledby="contributors-dialog-title"
        onMouseDown={(event) => event.stopPropagation()}
      >
        <div className="mb-4 flex items-center justify-between gap-4">
          <div>
            <h2 id="contributors-dialog-title" className="text-lg font-semibold">All contributors</h2>
            <p className="text-sm text-muted-foreground">Ordered by contribution to this repository</p>
          </div>
          <button type="button" className="rounded-md p-2 hover:bg-muted" onClick={onClose} aria-label="Close contributors">
            <X className="h-4 w-4" />
          </button>
        </div>
        <div className="min-h-0 space-y-2 overflow-y-auto pr-1">
          {contributors.map((contributor) => <ContributorDetails key={`${contributor.email}-${contributor.id}`} contributor={contributor} />)}
        </div>
      </section>
    </div>
  );
}

export function RepositoryMetadataBar({ repoId }: { repoId: string }) {
  const numericRepoId = Number(repoId);
  const repoInfo = useRepositoryById(numericRepoId).data;
  const repoLanguagesStats = useRepositoryLanguagesStats(numericRepoId).data;
  const contributors = useContributors(numericRepoId).data ?? [];
  const filesCount = useRepoFiles(numericRepoId).data?.length ?? 0;
  const lastCommitDate = useCommits(numericRepoId, 1, 0).data?.[0]?.committedAt ?? "N/A";
  const languagesDetails = useMemo(() => getLanguageDetails(repoLanguagesStats), [repoLanguagesStats]);
  const sortedContributors = useMemo(
    () => [...contributors].sort((a, b) => b.commitCount - a.commitCount || b.additions - a.additions),
    [contributors],
  );
  const [showAllContributors, setShowAllContributors] = useState(false);
  const visibleContributors = sortedContributors.length > 4 ? sortedContributors.slice(0, 3) : sortedContributors;

  const toggleStarred = useToggleRepositoryStarred();
  const isStarred = repoInfo?.isStarred ?? false;

  const handleToggleStarred = () => {
    if (!repoInfo) return;
    toggleStarred.mutate({ repoId: repoInfo.id, isStarred: !isStarred });
  };

  return (
    <div className="mx-10 flex flex-wrap items-center gap-10">
      <div className="flex min-w-[18rem] flex-1 flex-col items-start gap-2">
        <div className="flex w-full items-center gap-2">
          <span className="text-2xl font-semibold">{repoInfo?.name || "Repository"}</span>
          <span className={`rounded-full px-2 py-0.5 text-xs ${repoInfo?.isDirty ? "bg-red-100 text-red-800" : "bg-green-100 text-green-800"}`}>
            {repoInfo?.isDirty ? "Dirty" : "Clean"}
          </span>
          <button
            onClick={handleToggleStarred}
            disabled={toggleStarred.isPending}
            className={cn(
              "ml-2 inline-flex items-center gap-1.5 rounded-md border px-2.5 py-1 text-sm font-medium transition-colors cursor-pointer",
              isStarred
                ? "border-amber-300 bg-amber-50 text-amber-700 hover:bg-amber-100 dark:border-amber-700 dark:bg-amber-950 dark:text-amber-300 dark:hover:bg-amber-900"
                : "border-border bg-background text-muted-foreground hover:bg-muted hover:text-foreground",
              toggleStarred.isPending && "opacity-50"
            )}
          >
            <Star className={cn("h-4 w-4", isStarred && "fill-amber-400 text-amber-400")} />
            <span>{isStarred ? "Starred" : "Star"}</span>
          </button>
        </div>
        <span className="max-w-full truncate text-sm text-muted-foreground" title={repoInfo?.path ?? ""}>
          {repoInfo?.path || "Repository path unavailable"}
        </span>
        <div className="flex items-center gap-4">
          <span className="flex items-center"><GitBranch className="mr-2 h-4 w-4" />{repoInfo?.headBranch ?? "—"}</span>
          <span className="flex items-center"><GitCommit className="mr-2 h-4 w-4" />{repoInfo?.totalCommits ?? 0}</span>
          <span className="flex items-center"><FileText className="mr-2 h-4 w-4" />{filesCount}</span>
          <span className="flex items-center"><Users className="mr-2 h-4 w-4" />{contributors.length}</span>
          <span className="flex items-center text-sm"><Clock className="mr-2 h-4 w-4" />{formatUpdatedAt(lastCommitDate.toString())}</span>
        </div>
      </div>

      <div className="flex min-w-50 flex-1 flex-col items-start gap-2">
        <span className="text-lg font-semibold">Languages</span>
        <LanguageBar languages={languagesDetails} />
        <div className="flex max-w-120 flex-wrap gap-2">
          {languagesDetails.map((language) => (
            <div key={language.name} className="flex items-center gap-2">
              <div className={`h-2 w-2 rounded-full ${languageColors[language.name] ?? languageColors.Other}`} />
              <span className="text-sm">{language.name}</span>
              <span className="text-sm text-muted-foreground">{language.percentage}%</span>
            </div>
          ))}
        </div>
      </div>

      <div className="flex min-w-50 flex-1 flex-col items-start gap-2">
        <span className="text-lg font-semibold">Contributors</span>
        <div className="flex gap-2">
          {visibleContributors.map((contributor) => (
            <HoverCard key={contributor.email}>
              <HoverCardTrigger>
                <ContributorAvatar contributor={contributor} />
              </HoverCardTrigger>
              <HoverCardContent className="w-80">
                <ContributorDetails contributor={contributor} />
                <p className="mt-2 text-sm">Active days: {contributor.activeDays}</p>
              </HoverCardContent>
            </HoverCard>
          ))}
          {sortedContributors.length > 4 && <ContributorAvatar onClick={() => setShowAllContributors(true)} />}
        </div>
      </div>

      {showAllContributors && <ContributorsDialog contributors={sortedContributors} onClose={() => setShowAllContributors(false)} />}
    </div>
  );
}
