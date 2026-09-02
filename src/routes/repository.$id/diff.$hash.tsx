import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/repository/$id/diff/$hash')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/repository/$id/commits/diff/$hash"!</div>
}
