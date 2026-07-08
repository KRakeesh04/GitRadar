import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/repository/$id/pulls')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div className="rounded-md border border-border bg-card p-3 text-sm">Pull requests route loaded successfully.</div>
}
