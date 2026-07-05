import { HeadContent, Scripts, createRootRoute } from '@tanstack/react-router';
import { TanStackRouterDevtoolsPanel } from '@tanstack/react-router-devtools';
import { TanStackDevtools } from '@tanstack/react-devtools';

import appCss from '../styles.css?url';
import { RefreshCcw, GitPullRequest, Bell, Search } from 'lucide-react';
import { Button } from '#/components/ui/button';
import { Input } from '#/components/ui/input';
import { Separator } from '#/components/ui/separator';
import { AppSidebar } from '#/components/app-sidebar';
import { SidebarProvider } from '#/components/ui/sidebar';

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

function RootDocument({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <head>
        <HeadContent />
      </head>
      <body>
        <SidebarProvider>
          <AppSidebar />
          <div className="flex flex-col flex-1 min-h-screen">
            <header className="p-4 bg-card text-card-foreground">
              <div className="flex gap-2 mt-2">
                <div className="relative min-w-sm max-w-lg flex-1 mr-auto">
                  <Search className="pointer-events-none absolute left-3 top-1/2 h-5 w-5 -translate-y-1/2 text-muted-foreground" />
                  <Input
                    type="text"
                    placeholder="Search repos, commits, files..."
                    className="pl-10 pr-4 bg-background text-foreground placeholder:text-muted-foreground border border-input focus-visible:outline-none focus-visible:ring-2 w-full rounded-md text-sm shadow-sm transition-all focus-visible:ring-(--brand)"
                  />
                </div>
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
            <Separator className="border-t border-border" />
            {children}
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
