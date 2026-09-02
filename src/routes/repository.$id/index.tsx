import { ReadmeViewer } from '#/components/readme-previewer';
import { Card } from '#/components/ui/card';
import { useRepoFileContent } from '#/hooks/useFiles';
import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/repository/$id/')({
  component: RouteComponent,
})

function RouteComponent() {
  const { id } = Route.useParams();
  const readmeFile = useRepoFileContent(Number(id), 'README.md').data?.content ?? "No README file found.";

  return (
    <div className="flex flex-col gap-4">
      <div className="flex items-center gap-2">
        <Card className="p-4">
          <span className="font-semibold">Recent Activity</span>
        </Card>
        <Card className="p-4">
          <span className="font-semibold">Last commits</span>
        </Card>
      </div>
      <Card className="overflow-hidden">
        <ReadmeViewer content={readmeFile} />
      </Card>
    </div>
  );
}
