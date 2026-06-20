import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface Repository {
  id: number;
  name: string;
  path: string;
  git_dir_path: string;
  default_branch?: string;
  head_branch?: string;
  is_dirty: boolean;
  last_commit_hash?: string;
  last_commit_at?: string;
  last_scanned_at?: string;
  last_indexed_at?: string;
  index_status?: string;
  created_at: string;
  updated_at: string;
}

export const RepositoryList: React.FC = () => {
  const [repositories, setRepositories] = useState<Repository[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchRepositories = async () => {
    try {
      setLoading(true);
      const repos = await invoke<Repository[]>('get_all_repositories');
      setRepositories(repos);
      setError(null);
    } catch (err) {
      setError(err as string);
      console.error('Failed to fetch repositories:', err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    // Initial fetch
    fetchRepositories();

    // Set up periodic refresh to sync with background job
    const interval = setInterval(fetchRepositories, 10000); // Refresh every 10 seconds

    return () => clearInterval(interval);
  }, []);

  if (loading) {
    return (
      <div className="p-4">
        <div className="text-center">Loading repositories...</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="p-4">
        <div className="text-red-500">Error: {error}</div>
        <button
          onClick={fetchRepositories}
          className="mt-2 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
        >
          Retry
        </button>
      </div>
    );
  }

  return (
    <div className="p-6">
      <div className="flex justify-between items-center mb-6">
        <h2 className="text-2xl font-bold tracking-tight">Repositories ({repositories.length})</h2>
        <button
          onClick={fetchRepositories}
          className="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-10 px-4 py-2"
        >
          Refresh
        </button>
      </div>

      {repositories.length === 0 ? (
        <div className="flex flex-col items-center justify-center py-12 text-center">
          <div className="text-muted-foreground text-lg">
            No repositories found. Add a repository to get started.
          </div>
        </div>
      ) : (
        <div className="space-y-4">
          {repositories.map(repo => (
            <div
              key={repo.id}
              className="rounded-lg border bg-card text-card-foreground shadow-sm hover:shadow-md transition-shadow"
            >
              <div className="p-6">
                <div className="flex justify-between items-start">
                  <div className="flex-1">
                    <h3 className="text-lg font-semibold leading-none tracking-tight mb-2">
                      {repo.name}
                    </h3>
                    <p className="text-sm text-muted-foreground mb-3 font-mono">{repo.path}</p>
                    <div className="flex items-center gap-4 text-sm">
                      <span
                        className={`inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium ${
                          repo.is_dirty
                            ? 'bg-destructive/20 text-destructive'
                            : 'bg-success/20 text-success'
                        }`}
                      >
                        {repo.is_dirty ? 'Dirty' : 'Clean'}
                      </span>
                      {repo.last_commit_hash && (
                        <span className="text-muted-foreground font-mono">
                          Last: {repo.last_commit_hash.substring(0, 7)}
                        </span>
                      )}
                    </div>
                  </div>
                  <div className="text-right text-sm text-muted-foreground ml-4">
                    <div className="font-medium">
                      Updated: {new Date(repo.updated_at).toLocaleDateString()}
                    </div>
                    {repo.index_status && <div className="mt-1">Status: {repo.index_status}</div>}
                  </div>
                </div>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};
