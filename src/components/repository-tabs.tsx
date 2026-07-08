import { Link } from "@tanstack/react-router";

export enum RepositoryTab {
  Overview = 'overview',
  Commits = 'commits',
  Files = 'files',
  PullRequests = 'pulls',
  Insights = 'insights'
}

const tabLabels: Record<RepositoryTab, string> = {
  [RepositoryTab.Overview]: 'Overview',
  [RepositoryTab.Commits]: 'Commits',
  [RepositoryTab.Files]: 'Files',
  [RepositoryTab.PullRequests]: 'Pull Requests',
  [RepositoryTab.Insights]: 'Insights'
}

const tabRoutes: Record<RepositoryTab, '/repository/$id' | '/repository/$id/commits' | '/repository/$id/files' | '/repository/$id/pulls' | '/repository/$id/insights'> = {
  [RepositoryTab.Overview]: '/repository/$id',
  [RepositoryTab.Commits]: '/repository/$id/commits',
  [RepositoryTab.Files]: '/repository/$id/files',
  [RepositoryTab.PullRequests]: '/repository/$id/pulls',
  [RepositoryTab.Insights]: '/repository/$id/insights'
}

export function RepositoryTabs({ activeTab, repoId }: { activeTab: RepositoryTab, repoId: string }) {
  return (
    <div className="flex gap-4 border-b border-gray-300 px-4 py-2">
      {Object.values(RepositoryTab).map((tab) => (
        <Link
          key={tab}
          to={tabRoutes[tab]}
          params={{ id: repoId }}
          className={`px-3 py-2 font-medium ${activeTab === tab ? 'border-b-2 border-blue-500 text-blue-500' : 'text-gray-600'
            }`}
        >
          {tabLabels[tab]}
        </Link>
      ))}
    </div>
  );
}
