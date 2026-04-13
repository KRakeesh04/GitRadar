import { createFileRoute } from '@tanstack/react-router';
import { RepositoryList } from '../../components/RepositoryList';

export const Route = createFileRoute('/dashboard')({
  component: Dashboard,
});

function Dashboard() {
  return (
    <div className="container mx-auto">
      <h1 className="text-2xl font-bold mb-6">GitRadar Dashboard</h1>
      <RepositoryList />
    </div>
  );
}
