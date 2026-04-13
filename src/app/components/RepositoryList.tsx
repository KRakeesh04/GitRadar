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
    <div className="p-4">
      <div className="flex justify-between items-center mb-4">
        <h2 className="text-xl font-bold">Repositories ({repositories.length})</h2>
        <button 
          onClick={fetchRepositories}
          className="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600"
        >
          Refresh
        </button>
      </div>
      
      {repositories.length === 0 ? (
        <div className="text-gray-500 text-center py-8">
          No repositories found. Add a repository to get started.
        </div>
      ) : (
        <div className="space-y-2">
          {repositories.map((repo) => (
            <div 
              key={repo.id} 
              className="border rounded-lg p-4 hover:bg-gray-50 transition-colors"
            >
              <div className="flex justify-between items-start">
                <div>
                  <h3 className="font-semibold text-lg">{repo.name}</h3>
                  <p className="text-sm text-gray-600">{repo.path}</p>
                  <div className="flex items-center gap-4 mt-2 text-sm">
                    <span className={`px-2 py-1 rounded ${
                      repo.is_dirty 
                        ? 'bg-red-100 text-red-700' 
                        : 'bg-green-100 text-green-700'
                    }`}>
                      {repo.is_dirty ? 'Dirty' : 'Clean'}
                    </span>
                    {repo.last_commit_hash && (
                      <span className="text-gray-500">
                        Last: {repo.last_commit_hash.substring(0, 7)}
                      </span>
                    )}
                  </div>
                </div>
                <div className="text-right text-sm text-gray-500">
                  <div>Updated: {new Date(repo.updated_at).toLocaleDateString()}</div>
                  {repo.index_status && (
                    <div>Status: {repo.index_status}</div>
                  )}
                </div>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};
