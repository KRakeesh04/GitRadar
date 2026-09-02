import { RepositoryMetadataBar } from '#/components/repository-matadata-bar'
import { RepositoryTab, RepositoryTabs } from '#/components/repository-tabs'
import { Button } from '#/components/ui/button'
import { createFileRoute, Outlet, useRouterState } from '@tanstack/react-router'
import { ChevronLeft } from 'lucide-react'

export const Route = createFileRoute('/repository/$id')({
  component: RepositoryLayout,
})

// Common ui layout for all repository/:id/* routes
function RepositoryLayout() {
  const { id } = Route.useParams()
  const pathname = useRouterState({ select: (state) => state.location.pathname })

  const activeTab = pathname.includes('/commits') || pathname.includes('/diff')
    ? RepositoryTab.Commits
    : pathname.includes('/files')
      ? RepositoryTab.Files
      : pathname.includes('/pulls')
        ? RepositoryTab.PullRequests
        : pathname.includes('/insights')
          ? RepositoryTab.Insights
          : RepositoryTab.Overview

  return (
    <main className="flex flex-col gap-4 p-6">
      <Button variant="default" className="mb-2 flex items-center cursor-pointer w-18 bg-muted-background text-foreground hover:bg-muted" onClick={() => window.history.back()}>
        <ChevronLeft className="h-4 w-4" />
        <span>Back</span>
      </Button>
      <RepositoryMetadataBar repoId={id} />
      <RepositoryTabs activeTab={activeTab} repoId={id} />
      <Outlet />
    </main>
  )
}
