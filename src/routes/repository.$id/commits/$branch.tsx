import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/repository/$id/commits/$branch')({
  component: RouteComponent,
})

function RouteComponent() {
  const { branch } = Route.useParams()
  return (
    <div className="rounded-md border border-border bg-card p-3 text-sm">
      Commit list for branch: {branch}
    </div>
  )
}
