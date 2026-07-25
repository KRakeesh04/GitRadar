import { SearchBar } from '#/components/searchbar';
import { Button } from '#/components/ui/button';
import { Card, CardContent, CardHeader } from '#/components/ui/card';
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '#/components/ui/dropdown-menu';
import { Separator } from '#/components/ui/separator';
import { createFileRoute, Link, Outlet, useRouterState } from '@tanstack/react-router'
import { ArrowUpNarrowWide, ChevronsUpDown, Clock, FileText, Filter, GitBranch, GitCommit, Users } from 'lucide-react';
import { useState } from 'react';

export const Route = createFileRoute('/repository')({
  component: RouteComponent,
})

enum RepositorySearchFilter {
  All = 'All',
  Clean = 'Clean',
  Dirty = 'Dirty',
}

const searchFilterOptions = [
  { label: RepositorySearchFilter.All, value: RepositorySearchFilter.All },
  { label: RepositorySearchFilter.Clean, value: RepositorySearchFilter.Clean },
  { label: RepositorySearchFilter.Dirty, value: RepositorySearchFilter.Dirty },
];

enum RepositoryDropdownFilter {
  RecentlyAccessed = 'Recently accessed',
  Name = 'Name',
  MostCommits = 'Most commits',
}
const DropdownFilterOptions = [
  { label: RepositoryDropdownFilter.RecentlyAccessed, value: RepositoryDropdownFilter.RecentlyAccessed },
  { label: RepositoryDropdownFilter.Name, value: RepositoryDropdownFilter.Name },
  { label: RepositoryDropdownFilter.MostCommits, value: RepositoryDropdownFilter.MostCommits },
];

const repoList = [
  {
    name: 'Repo 1',
    description: 'This is the description for Repo 1.',
    path: 'there/is/something/in/this/path/to/repo1',
    status: 'Clean',
    branch: 'main',
    lastCommit: '2023-08-01',
    totalCommits: 10,
    fileCount: 100,
    contributors: 2
  },
  {
    name: 'Repo 2',
    description: 'This is the description for Rep/path/to/repo1o 2.',
    path: '/path/to/repo2',
    status: 'Dirty',
    branch: 'develop',
    lastCommit: '2023-08-02',
    totalCommits: 15,
    fileCount: 150,
    contributors: 3
  },
  {
    name: 'Repo 3',
    description: 'This is the description for Repo 3.',
    path: '/path/to/repo1',
    status: 'Clean',
    branch: 'main',
    lastCommit: '2023-08-01',
    totalCommits: 10,
    fileCount: 100,
    contributors: 2
  },
  {
    name: 'Repo 4',
    description: 'This is the description for Repo 4.',
    path: '/path/to/repo1',
    status: 'Dirty',
    branch: 'develop',
    lastCommit: '2023-08-02',
    totalCommits: 15,
    fileCount: 150,
    contributors: 3
  },
  {
    name: 'Repo 5',
    description: 'This is the description for Repo 5.',
    path: '/path/to/repo1',
    status: 'Clean',
    branch: 'main',
    lastCommit: '2023-08-01',
    totalCommits: 10,
    fileCount: 100,
    contributors: 2
  },
  {
    name: 'Repo 6',
    description: 'This is the description for Repo 6.',
    path: '/path/to/repo1',
    status: 'Clean',
    branch: 'develop',
    lastCommit: '2023-08-02',
    totalCommits: 15,
    fileCount: 150,
    contributors: 3
  },
  {
    name: 'Repo 7',
    description: 'This is the description for Repo 7.',
    path: '/path/to/repo1',
    status: 'Clean',
    branch: 'main',
    lastCommit: '2023-08-01',
    totalCommits: 10,
    fileCount: 100,
    contributors: 2
  },
  {
    name: 'Repo 8',
    description: 'This is the description for Repo 8.',
    path: '/path/to/repo1',
    status: 'Clean',
    branch: 'develop',
    lastCommit: '2023-08-02',
    totalCommits: 15,
    fileCount: 150,
    contributors: 3
  }
]

function RouteComponent() {
  const [filter, setFilter] = useState<RepositorySearchFilter>(RepositorySearchFilter.All);
  const [activeDropdownFilter, setActiveDropdownFilter] = useState<RepositoryDropdownFilter>(RepositoryDropdownFilter.RecentlyAccessed);
  const pathname = useRouterState({ select: (state) => state.location.pathname })

  if (pathname.replace(/\/$/, '') !== '/repository') {
    return <Outlet />
  }

  return (
    <div className="flex flex-col gap-2 scroll-auto px-[clamp(0.5rem,2vw,2.5rem)] py-5 overflow-y-auto">
      <span className="text-2xl font-medium">Repositories</span>
      <span className='text-muted-foreground'>5 repositories tracked · 2 with uncommitted changes</span>
      <div className="flex mt-4 lg:mt-5">
        <div className="flex items-center gap-3 ">
          <SearchBar placeholder="Filter repositories..." className='w-[clamp(10rem,20vw,15rem)]' />
          <Filter className="w-3 h-3 text-muted-foreground" />
          {searchFilterOptions.map((option) => (
            <Button
              key={option.value}
              variant='outline'
              className={`cursor-pointer border border-input ${filter === option.value ? 'text-foreground' : 'text-muted-foreground bg-background'}`}
              onClick={() => setFilter(option.value)}
            >
              <span>{option.label}</span>
            </Button>
          ))}
        </div>
        <div className="flex gap-2 ml-auto items-center">
          <ArrowUpNarrowWide className="w-3 h-3 text-muted-foreground" />
          <div className="flex-1 items-left w-42">
            <DropdownMenu>
              <DropdownMenuTrigger render={
                <Button
                  variant="outline"
                  className={'w-full text-left cursor-pointer'}
                >
                  <span>{activeDropdownFilter}</span>
                  <ChevronsUpDown className="ml-auto h-4 w-4 opacity-50" />
                </Button>
              } />
              <DropdownMenuContent>
                {DropdownFilterOptions.map((option) => (
                  <DropdownMenuItem
                    key={option.value}
                    onClick={() => setActiveDropdownFilter(option.value)}
                    className={`cursor-pointer ${activeDropdownFilter === option.value ? 'bg-(--brand-low) focus:bg-(--brand-low)' : 'focus:bg-muted '}`}
                  >
                    {option.label}
                  </DropdownMenuItem>
                ))}
              </DropdownMenuContent>
            </DropdownMenu>
          </div>
        </div>
      </div>
      <div className="flex flex-wrap justify-center gap-6 my-4 lg:my-5 py-5 place-items-center w-full overflow-y-auto">
        {/* Repository list content */}
        {repoList.map((repo, index) => (
          <Link
            key={index}
            to='/repository/$id'
            params={{ id: String(index) }}
            className="block"
          >
            <Card className="p-4 mb-2 w-90 lg:w-100 xl:w-120 w-clump(20rem, 30vw, 25rem) transition-transform duration-300 hover:border hover:border-(--brand) hover:shadow-lg cursor-pointer">
              <CardHeader className="flex flex-col gap-1">
                <div className="flex items-center gap-2 w-full">
                  <span className="text-lg font-semibold">{repo.name}</span>
                  <span className={`px-2 py-0.5 rounded-full text-xs ${repo.status === 'Clean' ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'}`}>
                    {repo.status}
                  </span>
                  <span className='ml-auto border border-input px-2 py-1 rounded-md bg-muted'>TypeScript</span>
                </div>
                <span className="text-sm text-muted-foreground">{repo.description}</span>
              </CardHeader>
              <CardContent>
                <div className="flex flex-wrap gap-5 mt-2 text-sm text-foreground">
                  <span className='flex items-center'><GitBranch className="w-4 h-4 mr-2" />{repo.branch}</span>
                  <span className='flex items-center'><Clock className="w-4 h-4 mr-2" />{repo.lastCommit}</span>
                </div>
                <Separator orientation="horizontal" className="my-2" />
                <div className="flex flex-wrap gap-5 mt-2 text-sm text-muted-foreground">
                  <span className='flex items-center'><GitCommit className="w-4 h-4 mr-2" />{repo.totalCommits}</span>
                  <span className='flex items-center'><FileText className="w-4 h-4 mr-2" />{repo.fileCount}</span>
                  <span className='flex items-center'><Users className="w-4 h-4 mr-2" />{repo.contributors}</span>
                  <span className='ml-auto'>{repo.path.length > 20 ? `...${repo.path.substring(repo.path.length - 20)}` : repo.path}</span>
                </div>
              </CardContent>
            </Card>
          </Link>
        ))}
      </div>
    </div >
  );
}
