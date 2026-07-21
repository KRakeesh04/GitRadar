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

interface SidebarmenuItem {
  name: string;
  icon: React.ReactNode;
  link: string;
  match: string[];
  indicator?: boolean;
}

interface SidebarContentItem {
  name: string;
  icon: React.ReactNode;
  list: RepositoryItem[];
}

interface RepositoryItem {
  name: string;
  branch: string;
  status: 'warning' | 'healthy';
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

const sidebarContentItems: SidebarContentItem[] = [
  {
    name: 'Recent',
    icon: <Clock className="w-4 h-4" />,
    list: [
      { name: 'gitradar', branch: 'main', status: 'warning' },
      { name: 'web-dashboard', branch: 'develop', status: 'healthy' },
      { name: 'api-server', branch: 'feature/auth', status: 'warning' },
      { name: 'mobile-app', branch: 'main', status: 'healthy' },
      { name: 'design-system', branch: 'main', status: 'healthy' },
      { name: 'gitradar', branch: 'main', status: 'warning' },
      { name: 'web-dashboard', branch: 'develop', status: 'healthy' },
      { name: 'api-server', branch: 'feature/auth', status: 'warning' },
      { name: 'mobile-app', branch: 'main', status: 'healthy' },
      { name: 'design-system', branch: 'main', status: 'healthy' },
    ],
  },
  {
    name: 'Starred',
    icon: <Star className="w-4 h-4" />,
    list: [
      { name: 'gitradar', branch: 'main', status: 'warning' },
      { name: 'api-server', branch: 'feature/auth', status: 'warning' },
      { name: 'gitradar', branch: 'main', status: 'warning' },
      { name: 'web-dashboard', branch: 'develop', status: 'healthy' },
      { name: 'api-server', branch: 'feature/auth', status: 'warning' },
      { name: 'mobile-app', branch: 'main', status: 'healthy' },
      { name: 'design-system', branch: 'main', status: 'healthy' },
    ],
  },
];

export function AppSidebar() {
  const location = useLocation();
  const { resolvedTheme, setTheme } = useTheme();
  const { toggleSidebar, open } = useSidebar();

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
          <Tabs defaultValue="recent" className="flex h-full min-h-0 w-full flex-col gap-3">
            <TabsList className="h-8 w-fit shrink-0 grid-cols-2 bg-muted/50 p-0">
              {sidebarContentItems.map(item => (
                <TabsTrigger
                  key={item.name}
                  value={item.name.toLowerCase()}
                  className="h-8 rounded-md px-2.5 text-sm"
                >
                  {item.icon}
                  <span>{item.name}</span>
                </TabsTrigger>
              ))}
            </TabsList>
            {sidebarContentItems.map(item => (
              <TabsContent
                key={item.name}
                value={item.name.toLowerCase()}
                className="mt-0 min-h-0 flex-1 overflow-hidden"
              >
                <div className="h-full space-y-4 overflow-y-auto pr-1">
                  {item.list.map((repo, index) => (
                    <a
                      key={`${item.name}-${repo.name}-${repo.branch}-${index}`}
                      href="/repository"
                      className="block rounded-md px-0.5 py-0.5 text-sidebar-foreground transition-colors hover:text-(--brand)"
                    >
                      <div className="flex items-center gap-1.5">
                        <span className="text-sm lg:font-semibold leading-5">{repo.name}</span>
                        <span
                          className={cn(
                            'h-1.5 w-1.5 rounded-full',
                            repo.status === 'warning' ? 'bg-amber-500' : 'bg-emerald-500'
                          )}
                        />
                      </div>
                      <div className="mt-0.5 flex items-center gap-1.5 text-xs text-muted-foreground">
                        <GitBranch className="h-3 w-3" />
                        <span>{repo.branch}</span>
                      </div>
                    </a>
                  ))}
                </div>
              </TabsContent>
            ))}
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
