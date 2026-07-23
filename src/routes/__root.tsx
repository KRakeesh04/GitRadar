import {
  HeadContent,
  Scripts,
  createRootRoute,
  useNavigate,
  useRouterState,
} from '@tanstack/react-router';
import { TanStackRouterDevtoolsPanel } from '@tanstack/react-router-devtools';
import { TanStackDevtools } from '@tanstack/react-devtools';

import appCss from '../styles.css?url';
import { RefreshCcw, GitPullRequest, Bell } from 'lucide-react';
import { Button } from '#/components/ui/button';
import { Separator } from '#/components/ui/separator';
import { AppSidebar } from '#/components/app-sidebar';
import { SidebarProvider } from '#/components/ui/sidebar';
import { useState } from 'react';
import type { ReactNode } from 'react';
import { SearchBar } from '#/components/searchbar';
import { ThemeProvider } from '#/contexts/ThemeContext';
import { requestAddRootPathPopover } from '#/lib/root-path-actions';
import { QueryClientProvider } from '@tanstack/react-query';
import { queryClient } from '#/lib/query-client';
import { Toaster } from '#/components/ui/sonner';

export const Route = createRootRoute({
  head: () => ({
    meta: [
      {
        charSet: 'utf-8',
      },
      {
        name: 'viewport',
        content: 'width=device-width, initial-scale=1',
      },
      {
        title: 'GitRadar',
      },
    ],
    links: [
      {
        rel: 'stylesheet',
        href: appCss,
      },
    ],
  }),
  shellComponent: RootDocument,
});

function RootDocument({ children }: { children: ReactNode }) {
  const [isSidebarOpen, setIsSidebarOpen] = useState(true);
  const navigate = useNavigate();
  const pathname = useRouterState({ select: state => state.location.pathname });

  const handleAddRootPath = () => {
    requestAddRootPathPopover({ persist: pathname !== '/root-paths' });

    if (pathname !== '/root-paths') {
      void navigate({ to: '/root-paths' });
    }
  };

  return (
    <html lang="en">
      <head>
        <HeadContent />
        <script
          dangerouslySetInnerHTML={{
            __html: `
              (() => {
                try {
                  const stored = localStorage.getItem("gitradar-theme");

                  const theme = stored
                    ? JSON.parse(stored).state.theme
                    : "system";

                  let resolved = "light";

                  if (theme === "dark") {
                    resolved = "dark";
                  } else if (theme === "system") {
                    resolved = window.matchMedia(
                      "(prefers-color-scheme: dark)"
                    ).matches
                      ? "dark"
                      : "light";
                  }
                  document.documentElement.classList.add(resolved);
                } catch {}
              })();
            `,
          }}
        />
      </head>
      <body>
        <QueryClientProvider client={queryClient}>
          <ThemeProvider>
            <SidebarProvider
              open={isSidebarOpen}
              onOpenChange={setIsSidebarOpen}
              className="h-svh overflow-hidden"
            >
              <AppSidebar />
              <div className="flex min-h-0 flex-1 flex-col">
                <header className="shrink-0 p-4 bg-card text-card-foreground">
                  <div className="flex gap-2 mt-2">
                    <SearchBar placeholder="Search repos, commits, files..." className="min-w-sm" />
                    <div className="flex items-center gap-3 ml-auto">
                      <Button
                        variant="default"
                        className="ml-2 cursor-pointer bg-background/20 hover:bg-(--brand) hover:text-background text-foreground border border-input"
                      >
                        <RefreshCcw className="w-5 h-5 cursor-pointer" />
                      </Button>
                      <Button
                        variant="default"
                        className="cursor-pointer bg-background/20 hover:bg-(--brand) hover:text-background text-foreground border border-input"
                      >
                        <GitPullRequest className="w-5 h-5 cursor-pointer" />
                      </Button>
                      <Button
                        variant="default"
                        className="cursor-pointer bg-background/20 hover:bg-(--brand) hover:text-background text-foreground border border-input"
                      >
                        <Bell className="w-5 h-5 cursor-pointer" />
                      </Button>
                      <Button
                        variant="default"
                        className="cursor-pointer bg-(--brand) hover:bg-(--brand-hover) text-white"
                        onClick={handleAddRootPath}
                      >
                        + Add Root Path
                      </Button>
                    </div>
                  </div>
                </header>
                <Separator className="shrink-0 border-t border-border" />
                <main className="min-h-0 flex-1 overflow-y-auto">{children}</main>
                <Toaster />
                <TanStackDevtools
                  config={{
                    position: 'bottom-right',
                  }}
                  plugins={[
                    {
                      name: 'Tanstack Router',
                      render: <TanStackRouterDevtoolsPanel />,
                    },
                  ]}
                />
                <Scripts />
              </div>
            </SidebarProvider>
          </ThemeProvider>
        </QueryClientProvider>
      </body>
    </html>
  );
}
