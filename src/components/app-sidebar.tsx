import * as React from "react";
import { Bell, Clock, FolderGit2, FolderOpen, GitBranch, Grid2X2, LayoutDashboard, Moon, Search, Settings, Star, Sun } from "lucide-react";
import { useLocation } from "@tanstack/react-router";

import { Sidebar, SidebarContent, SidebarFooter, SidebarGroupContent, SidebarHeader, SidebarMenu, SidebarMenuButton, SidebarMenuItem } from "./ui/sidebar";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "./ui/tabs";
import { Separator } from "./ui/separator";
import { cn } from "@/lib/utils";

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
  status: "warning" | "healthy";
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
  }
];

const sidebarContentItems: SidebarContentItem[] = [
  {
    name: 'Recent',
    icon: <Clock className="w-4 h-4" />,
    list: [
      { name: "gitradar", branch: "main", status: "warning" },
      { name: "web-dashboard", branch: "develop", status: "healthy" },
      { name: "api-server", branch: "feature/auth", status: "warning" },
      { name: "mobile-app", branch: "main", status: "healthy" },
      { name: "design-system", branch: "main", status: "healthy" },
    ],
  },
  {
    name: 'Starred',
    icon: <Star className="w-4 h-4" />,
    list: [
      { name: "gitradar", branch: "main", status: "warning" },
      { name: "api-server", branch: "feature/auth", status: "warning" },
    ],
  }
];

export function AppSidebar() {
  const location = useLocation();
  const [isDarkMode, setIsDarkMode] = React.useState(false);

  React.useEffect(() => {
    setIsDarkMode(document.documentElement.classList.contains("dark"));
  }, []);

  const toggleTheme = () => {
    document.documentElement.classList.toggle("dark");
    setIsDarkMode(document.documentElement.classList.contains("dark"));
  };

  return (
    <Sidebar className="border-sidebar-border">
      <SidebarHeader className="flex h-18 flex-row items-center gap-5 px-4 py-0">
        <div className="w-9 h-9 bg-(--brand) rounded-lg flex items-center justify-center shrink-0 ml-0.5">
          <FolderGit2 className="text-white" size={30} />
        </div>
        <span className="text-sidebar-foreground text-2xl font-bold">GitRadar</span>
      </SidebarHeader>
      <Separator className="bg-sidebar-border" />
      <SidebarContent className="bg-sidebar">
        <SidebarGroupContent className="px-2 py-2">
          <SidebarMenu className="gap-1">
            {sidebarMenuItems.map((item) => (
              <SidebarMenuItem key={item.name}>
                <SidebarMenuButton
                  isActive={item.match.some((path) => location.pathname === path || (path !== "/" && location.pathname.startsWith(path)))}
                  className="h-9 rounded-md px-3 text-sm font-normal data-active:bg-(--brand) data-active:text-white data-active:hover:bg-(--brand) data-active:hover:text-white"
                  render={
                    <a href={item.link}>
                      {item.icon}
                      <span>{item.name}</span>
                      {item.indicator ? <span className="ml-auto h-2 w-2 rounded-full bg-(--brand)" /> : null}
                    </a>
                  }
                />
              </SidebarMenuItem>
            ))}
          </SidebarMenu>
        </SidebarGroupContent>
        <Separator className="bg-sidebar-border" />
        <SidebarGroupContent className="px-3 py-2">
          <Tabs defaultValue="recent" className="w-full gap-3">
            <TabsList className="h-8 w-fit grid-cols-2 bg-muted/50 p-0">
              {sidebarContentItems.map((item) => (
                <TabsTrigger key={item.name} value={item.name.toLowerCase()} className="h-8 rounded-md px-2.5 text-sm">
                  {item.icon}
                  <span>{item.name}</span>
                </TabsTrigger>
              ))}
            </TabsList>
            {sidebarContentItems.map((item) => (
              <TabsContent key={item.name} value={item.name.toLowerCase()} className="mt-0">
                <div className="space-y-4">
                  {item.list.map((repo) => (
                    <a
                      key={`${item.name}-${repo.name}`}
                      href="/repository"
                      className="block rounded-md px-0.5 py-0.5 text-sidebar-foreground transition-colors hover:text-(--brand)"
                    >
                      <div className="flex items-center gap-1.5">
                        <span className="text-sm font-semibold leading-5">{repo.name}</span>
                        <span
                          className={cn(
                            "h-1.5 w-1.5 rounded-full",
                            repo.status === "warning" ? "bg-amber-500" : "bg-emerald-500"
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
              className="h-9 rounded-md px-3 text-sm font-normal hover:bg-muted/70"
            >
              {isDarkMode ? <Sun className="w-4 h-4" /> : <Moon className="w-4 h-4" />}
              <span>{isDarkMode ? "Light Mode" : "Dark Mode"}</span>
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
