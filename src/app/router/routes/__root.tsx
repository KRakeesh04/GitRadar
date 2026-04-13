import { Navigate, createRootRoute, Outlet } from '@tanstack/react-router';

export const Route = createRootRoute({
  notFoundComponent: () => <Navigate to="/dashboard" />,
  component: () => (
    <div>
      <h1>GitRadar 🚀</h1>
      <Outlet />
    </div>
  ),
});
