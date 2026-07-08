import { HeadContent, Scripts, createRootRoute } from '@tanstack/react-router';
import { TanStackRouterDevtoolsPanel } from '@tanstack/react-router-devtools';
import { TanStackDevtools } from '@tanstack/react-devtools';

import appCss from '../styles.css?url';
import { RefreshCcw, GitPullRequest, Bell } from 'lucide-react';
import { Button } from '#/components/ui/button';
import { Separator } from '#/components/ui/separator';
import { AppSidebar } from '#/components/app-sidebar';
import { SidebarProvider } from '#/components/ui/sidebar';
import { useState, type ReactNode } from 'react';
import { SearchBar } from '#/components/searchbar';

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
  return (
    <html lang="en">
      <head>
        <HeadContent />
      </head>
      <body>
        <SidebarProvider open={isSidebarOpen} onOpenChange={setIsSidebarOpen} className="h-svh overflow-hidden">
          <AppSidebar />
          <div className="flex min-h-0 flex-1 flex-col">
            <header className="shrink-0 p-4 bg-card text-card-foreground">
              <div className="flex gap-2 mt-2">
                <SearchBar placeholder="Search repos, commits, files..." className="min-w-sm" />
                <div className="flex items-center gap-3 ml-auto">
                  <Button variant="default" className="ml-2 cursor-pointer bg-background hover:bg-(--brand) hover:text-background text-foreground border border-input">
                    <RefreshCcw className="w-5 h-5 cursor-pointer" />
                  </Button>
                  <Button variant="default" className="cursor-pointer bg-background hover:bg-(--brand) hover:text-background text-foreground border border-input">
                    <GitPullRequest className="w-5 h-5 cursor-pointer" />
                  </Button>
                  <Button variant="default" className="cursor-pointer bg-background hover:bg-(--brand) hover:text-background text-foreground border border-input">
                    <Bell className="w-5 h-5 cursor-pointer" />
                  </Button>
                  <Button variant="default" className="cursor-pointer bg-(--brand) hover:bg-(--brand-hover) text-white">
                    + Add Repository
                  </Button>
                </div>
              </div>
            </header>
            <Separator className="shrink-0 border-t border-border" />
            <main className="min-h-0 flex-1 overflow-y-auto">
              {children}
            </main>
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
      </body>
    </html>
  );
}
