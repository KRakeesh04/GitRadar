import { useMemo, useState } from 'react';
import {
  Bell,
  Clock,
  FolderGit2,
  FolderOpen,
  GitBranch,
  HardDrive,
  LayoutDashboard,
  Moon,
  PanelLeftClose,
  PanelRightClose,
  Search,
  Settings,
  Star,
  Sun,
} from 'lucide-react';
import { useLocation } from '@tanstack/react-router';
import { useQueries } from '@tanstack/react-query';

import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroupContent,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  useSidebar,
} from './ui/sidebar';
import { Tabs, TabsContent, TabsList, TabsTrigger } from './ui/tabs';
import { Separator } from './ui/separator';
import { cn } from '@/lib/utils';
import { useTheme } from '#/contexts/ThemeContext';
import { getRecentRepositories, getStarredRepositories } from '#/lib/tauri/repositories';
import { queryKeys } from '#/lib/query-keys';
import type { RepositoryInfo } from '#/lib/tauri/repositories';

interface SidebarmenuItem {
  name: string;
  icon: React.ReactNode;
  link: string;
  match: string[];
  indicator?: boolean;
}

const sidebarMenuItems: SidebarmenuItem[] = [
  {
    name: 'Dashboard',
    icon: <LayoutDashboard className="w-4 h-4" />,
    link: '/',
    match: ['/'],
  },
  {
    name: 'Repositories',
    icon: <FolderOpen className="w-4 h-4" />,
    link: '/repository',
    match: ['/repository'],
  },
  {
    name: 'Search',
    icon: <Search className="w-4 h-4" />,
    link: '/search',
    match: ['/search'],
  },
  {
    name: 'Activity',
    icon: <Bell className="w-4 h-4" />,
    link: '/activity',
    match: ['/activity'],
    indicator: true,
  },
  {
    name: 'Root Paths',
    icon: <HardDrive className="w-4 h-4" />,
    link: '/root-paths',
    match: ['/root-paths'],
  },
];

const PAGE_SIZE = 11;

function RepoListItem({ repo }: { repo: RepositoryInfo }) {
  return (
    <a
      href={`/repository/${repo.id}`}
      className="block rounded-md px-0.5 py-0.5 text-sidebar-foreground transition-colors hover:text-(--brand)"
    >
      <div className="flex items-center gap-1.5">
        <span className="text-sm lg:font-semibold leading-5 truncate">{repo.name}</span>
        <span
          className={cn(
            'h-1.5 w-1.5 rounded-full shrink-0',
            repo.isDirty ? 'bg-amber-500' : 'bg-emerald-500'
          )}
        />
      </div>
      <div className="mt-0.5 flex items-center gap-1.5 text-xs text-muted-foreground">
        <GitBranch className="h-3 w-3" />
        <span className="truncate">{repo.headBranch ?? 'No branch'}</span>
      </div>
    </a>
  );
}

export function AppSidebar() {
  const location = useLocation();
  const { resolvedTheme, setTheme } = useTheme();
  const { toggleSidebar, open } = useSidebar();

  const [recentOffsets, setRecentOffsets] = useState<number[]>([0]);
  const [starredOffsets, setStarredOffsets] = useState<number[]>([0]);
  const [activeTab, setActiveTab] = useState<'recent' | 'starred'>(() => {
    if (typeof window === 'undefined') return 'recent';
    const saved = window.localStorage.getItem('sidebar-active-tab');
    return saved === 'starred' ? 'starred' : 'recent';
  });

  // Query every loaded page so the list is derived from the cache and stays in
  // sync when mutations invalidate/refetch these query keys.
  const recentQueries = useQueries({
    queries: recentOffsets.map(offset => ({
      queryKey: queryKeys.recentRepositories(PAGE_SIZE, offset),
      queryFn: () => getRecentRepositories(PAGE_SIZE, offset),
    })),
  });
  const starredQueries = useQueries({
    queries: starredOffsets.map(offset => ({
      queryKey: queryKeys.starredRepositories(PAGE_SIZE, offset),
      queryFn: () => getStarredRepositories(PAGE_SIZE, offset),
    })),
  });

  const recentRepos = useMemo(() => {
    const seen = new Map<number, RepositoryInfo>();
    for (const q of recentQueries) {
      for (const repo of q.data ?? []) seen.set(repo.id, repo);
    }
    return [...seen.values()];
  }, [recentQueries]);

  const starredRepos = useMemo(() => {
    const seen = new Map<number, RepositoryInfo>();
    for (const q of starredQueries) {
      for (const repo of q.data ?? []) seen.set(repo.id, repo);
    }
    return [...seen.values()];
  }, [starredQueries]);

  const lastRecentQuery = recentQueries[recentQueries.length - 1];
  const lastStarredQuery = starredQueries[starredQueries.length - 1];

  const recentHasMore = (lastRecentQuery.data?.length ?? 0) >= PAGE_SIZE;
  const starredHasMore = (lastStarredQuery.data?.length ?? 0) >= PAGE_SIZE;

  const isLoadingRecent =
    recentQueries.length > 0 && recentRepos.length === 0 && recentQueries.some(q => q.isLoading);
  const isLoadingStarred =
    starredQueries.length > 0 && starredRepos.length === 0 && starredQueries.some(q => q.isLoading);

  const loadMoreRecent = () => {
    if (!recentHasMore) return;
    setRecentOffsets(prev => [...prev, prev[prev.length - 1] + PAGE_SIZE]);
  };

  const loadMoreStarred = () => {
    if (!starredHasMore) return;
    setStarredOffsets(prev => [...prev, prev[prev.length - 1] + PAGE_SIZE]);
  };

  const toggleTheme = () => {
    setTheme(resolvedTheme === 'dark' ? 'light' : 'dark');
  };

  return (
    <Sidebar collapsible="icon" className="border-sidebar-border">
      <SidebarHeader className="flex h-18 flex-row items-center gap-5 px-4 py-0 group-data-[collapsible=icon]:px-1.5 group">
        <div className="w-9 h-9 bg-(--brand) rounded-lg flex items-center justify-center shrink-0 ml-0.5 group-data-[collapsible=icon]:ml-0 group-data-[collapsible=icon]:group-hover:hidden transition-colors">
          <FolderGit2 className="text-white" size={25} />
        </div>
        <span className="text-sidebar-foreground text-2xl font-bold group-data-[collapsible=icon]:hidden">
          GitRadar
        </span>
        <button
          className="ml-auto text-sidebar-foreground cursor-pointer group-data-[collapsible=icon]:absolute left-3.5"
          onClick={toggleSidebar}
        >
          {open ? (
            <PanelLeftClose className="w-5 h-5 group-data-[collapsible=icon]:hidden" />
          ) : (
            <PanelRightClose className="hidden w-5 h-5 group-data-[collapsible=icon]:group-hover:block" />
          )}
        </button>
      </SidebarHeader>
      <Separator className="bg-sidebar-border" />
      <SidebarContent className="min-h-0 overflow-hidden bg-sidebar">
        <SidebarGroupContent className="shrink-0 px-2 py-2">
          <SidebarMenu className="gap-1">
            {sidebarMenuItems.map(item => (
              <SidebarMenuItem key={item.name}>
                <SidebarMenuButton
                  isActive={item.match.some(
                    path =>
                      location.pathname === path ||
                      (path !== '/' && location.pathname.startsWith(path))
                  )}
                  className="h-9 rounded-md px-3 text-sm font-normal data-active:bg-(--brand) data-active:text-white data-active:hover:bg-(--brand) data-active:hover:text-white"
                  render={
                    <a href={item.link}>
                      {!open && item.indicator ? (
                        <span className="text-(--brand)">{item.icon}</span>
                      ) : (
                        item.icon
                      )}
                      <span>{item.name}</span>
                      {item.indicator ? (
                        <span className="ml-auto h-2 w-2 rounded-full bg-(--brand)" />
                      ) : null}
                    </a>
                  }
                />
              </SidebarMenuItem>
            ))}
          </SidebarMenu>
        </SidebarGroupContent>
        <Separator className="bg-sidebar-border" />
        <SidebarGroupContent className="min-h-0 flex-1 px-3 py-2 group-data-[collapsible=icon]:hidden">
          <Tabs
            value={activeTab}
            onValueChange={value => {
              if (value === 'recent' || value === 'starred') {
                setActiveTab(value);
                if (typeof window !== 'undefined') {
                  window.localStorage.setItem('sidebar-active-tab', value);
                }
              }
            }}
            className="flex h-full min-h-0 w-full flex-col gap-3"
          >
            <TabsList className="h-8 w-fit shrink-0 grid-cols-2 bg-muted/50 p-0">
              <TabsTrigger value="recent" className="h-8 rounded-md px-2.5 text-sm">
                <Clock className="w-4 h-4" />
                <span>Recent</span>
              </TabsTrigger>
              <TabsTrigger value="starred" className="h-8 rounded-md px-2.5 text-sm">
                <Star className="w-4 h-4" />
                <span>Starred</span>
              </TabsTrigger>
            </TabsList>

            <TabsContent value="recent" className="mt-0 min-h-0 flex-1 overflow-hidden">
              <div className="h-full space-y-1 overflow-y-auto pr-1">
                {isLoadingRecent ? (
                  <div className="space-y-2 py-2 min-h-55">
                    {[1, 2, 3, 4, 5, 6].map(i => (
                      <div key={i} className="animate-pulse space-y-1.5 px-0.5">
                        <div className="h-4 w-3/4 rounded bg-muted" />
                        <div className="h-3 w-1/2 rounded bg-muted" />
                      </div>
                    ))}
                  </div>
                ) : recentRepos.length === 0 ? (
                  <p className="py-4 text-center text-xs text-muted-foreground">
                    No recent repositories
                  </p>
                ) : (
                  <>
                    {recentRepos.map((repo, index) => (
                      <RepoListItem key={`recent-${repo.id}-${index}`} repo={repo} />
                    ))}
                    {recentHasMore && (
                      <button
                        onClick={loadMoreRecent}
                        disabled={lastRecentQuery.isFetching}
                        className="w-full rounded-md py-1.5 text-center text-xs text-muted-foreground transition-colors hover:bg-muted/50 hover:text-foreground cursor-pointer disabled:opacity-50"
                      >
                        {lastRecentQuery.isFetching ? 'Loading...' : 'Load more'}
                      </button>
                    )}
                  </>
                )}
              </div>
            </TabsContent>

            <TabsContent value="starred" className="mt-0 min-h-0 flex-1 overflow-hidden">
              <div className="h-full space-y-1 overflow-y-auto pr-1">
                {isLoadingStarred ? (
                  <div className="space-y-2 py-2 min-h-55">
                    {[1, 2, 3, 4, 5, 6].map(i => (
                      <div key={i} className="animate-pulse space-y-1.5 px-0.5">
                        <div className="h-4 w-3/4 rounded bg-muted" />
                        <div className="h-3 w-1/2 rounded bg-muted" />
                      </div>
                    ))}
                  </div>
                ) : starredRepos.length === 0 ? (
                  <p className="py-4 text-center text-xs text-muted-foreground">
                    No starred repositories
                  </p>
                ) : (
                  <>
                    {starredRepos.map((repo, index) => (
                      <RepoListItem key={`starred-${repo.id}-${index}`} repo={repo} />
                    ))}
                    {starredHasMore && (
                      <button
                        onClick={loadMoreStarred}
                        disabled={lastStarredQuery.isFetching}
                        className="w-full rounded-md py-1.5 text-center text-xs text-muted-foreground transition-colors hover:bg-muted/50 hover:text-foreground cursor-pointer disabled:opacity-50"
                      >
                        {lastStarredQuery.isFetching ? 'Loading...' : 'Load more'}
                      </button>
                    )}
                  </>
                )}
              </div>
            </TabsContent>
          </Tabs>
        </SidebarGroupContent>
      </SidebarContent>
      <Separator className="bg-sidebar-border" />
      <SidebarFooter className="bg-sidebar p-3">
        <SidebarMenu className="gap-1">
          <SidebarMenuItem>
            <SidebarMenuButton
              onClick={toggleTheme}
              className="h-9 rounded-md px-3 text-sm font-normal hover:bg-muted/70 cursor-pointer"
            >
              {resolvedTheme === 'dark' ? (
                <Sun className="w-4 h-4" />
              ) : (
                <Moon className="w-4 h-4" />
              )}
              <span>{resolvedTheme === 'dark' ? 'Light Mode' : 'Dark Mode'}</span>
            </SidebarMenuButton>
          </SidebarMenuItem>
          <SidebarMenuItem>
            <SidebarMenuButton
              className="h-9 rounded-md px-3 text-sm font-normal hover:bg-muted/70"
              render={
                <a href="/settings">
                  <Settings className="w-4 h-4" />
                  <span>Settings</span>
                </a>
              }
            />
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarFooter>
    </Sidebar>
  );
}
