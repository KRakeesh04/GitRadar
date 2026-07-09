import { Clock, FileText, GitBranch, GitCommit, Users } from "lucide-react"
import { cn } from "#/lib/utils";
import { HoverCard, HoverCardContent, HoverCardTrigger } from "./ui/hover-card";
import { Separator } from "./ui/separator";

const repoInfo = {
  name: 'Repo 1',
  description: 'This is the description for Repo 1.',
  path: 'there/is/something/in/this/path/to/repo1',
  status: 'Clean',
  branch: 'main',
  lastCommit: '2023-08-01',
  totalCommits: 10,
  fileCount: 100,
  contributors: 2
}

const contributors = [
  { name: 'Contributor 1', email: 'contributor1@example.com', contributions: 50, additions: 100, deletions: 20 },
  { name: 'Contributor 2', email: 'contributor2@example.com', contributions: 30, additions: 50, deletions: 10 },
  { name: 'Contributor 3', email: 'contributor3@example.com', contributions: 20, additions: 30, deletions: 5 },
]

const languagesDetails = [
  { name: 'JavaScript', percentage: 50 },
  { name: 'TypeScript', percentage: 30 },
  { name: 'Python', percentage: 18 },
  { name: 'Other', percentage: 2 },
]

const languageColors: Record<string, string> = {
  TypeScript: "bg-blue-500",
  JavaScript: "bg-yellow-400",
  Rust: "bg-orange-500",
  Go: "bg-cyan-500",
  Python: "bg-green-500",
  Ruby: "bg-red-500",
  C: "bg-gray-500",
  Cpp: "bg-gray-500",
  Java: "bg-red-600",
  PHP: "bg-purple-500",
  Swift: "bg-orange-400",
  Kotlin: "bg-purple-400",
  Dart: "bg-blue-400",
  Scala: "bg-red-400",
  Haskell: "bg-purple-600",
  Lua: "bg-blue-300",
  Perl: "bg-blue-600",
  R: "bg-blue-700",
  Shell: "bg-green-700",
  HTML: "bg-orange-300",
  CSS: "bg-blue-200",
  SQL: "bg-blue-800",
  Other: "bg-gray-500",
}

function LanguageBar({
  languages,
  className,
}: {
  languages: { name: string; percentage: number }[];
  className?: string;
}) {
  return (
    <div className={cn("h-2 w-full overflow-hidden rounded-full bg-muted flex", className)}>
      {languages.map((lang) => (
        <div
          key={lang.name}
          className={languageColors[lang.name]}
          style={{ width: `${lang.percentage}%` }}
        />
      ))}
    </div>
  )
}

export function RepositoryMetadataBar({ repoId }: { repoId: string }) {
  return (
    <div className="flex gap-10 flex-wrap mx-10 items-center">
      <div className="flex-1 flex-col items-left gap-2">
        <div className="flex items-center gap-2 w-full">
          <span className="text-2xl font-semibold">{repoInfo.name}</span>
          <span className={`px-2 py-0.5 rounded-full text-xs ${repoInfo.status === 'Clean' ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'}`}>
            {repoInfo.status}
          </span>
        </div>
        <span className="text-sm text-muted-foreground">{repoInfo.description}</span>
        <span className="text-sm text-muted-foreground">{repoInfo.path}</span>
        <div className="flex items-left gap-4">
          <span className='flex items-center'><GitBranch className="w-4 h-4 mr-2" />{repoInfo.branch}</span>
          <span className='flex items-center'><GitCommit className="w-4 h-4 mr-2" />{repoInfo.totalCommits}</span>
          <span className='flex items-center'><FileText className="w-4 h-4 mr-2" />{repoInfo.fileCount}</span>
          <span className='flex items-center'><Users className="w-4 h-4 mr-2" />{repoInfo.contributors}</span>
          <span className='flex items-center text-sm'><Clock className="w-4 h-4 mr-2" />{repoInfo.lastCommit}</span>
        </div>
      </div>
      <div className="flex-1 flex-col gap-2 items-left w-auto max-w-100 min-w-50">
        <span className="text-lg font-semibold">Languages</span>
        <LanguageBar languages={languagesDetails} className="w-auto max-w-100 min-w-50" />
        <div className="flex flex-wrap gap-2 max-w-120 w-auto">
          {languagesDetails.map((lang) => (
            <div key={lang.name} className="flex items-center gap-2">
              <div className={`w-2 h-2 rounded-full ${languageColors[lang.name]}`} />
              <span className="text-sm">{lang.name}</span>
              <span className="text-sm text-muted-foreground">{lang.percentage}%</span>
            </div>
          ))}
        </div>
      </div>
      <div className="flex-1 flex-col gap-2 items-left">
        <span className="text-lg font-semibold">Contributors</span>
        <div className="flex gap-2">
          {contributors.map((contributor) => (
            <HoverCard key={contributor.email}>
              <HoverCardTrigger>
                <div className="flex flex-col gap-2 items-center cursor-pointer">
                  <div className="w-10 h-10 rounded-full bg-gray-300" />
                  <span className="text-sm text-muted-foreground">
                    {contributor.name}
                  </span>
                </div>
              </HoverCardTrigger>
              <HoverCardContent className="w-72">
                <div className="space-y-2">
                  <div className="flex flex-col items-center gap-2">
                    <div className="w-10 h-10 rounded-full bg-gray-300" />
                    <span className="font-semibold">{contributor.name}</span>
                    <span className="text-sm text-muted-foreground">{contributor.email}</span>
                  </div>
                  <Separator className="my-2" />
                  <p>Contributions: {contributor.contributions}</p>
                  <p>Additions: {contributor.additions}</p>
                  <p>Deletions: {contributor.deletions}</p>
                </div>
              </HoverCardContent>
            </HoverCard>
          ))}
        </div>
      </div>
    </div>
  );
}