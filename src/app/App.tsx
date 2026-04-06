import { RouterProvider } from "@tanstack/react-router";
import { router } from "./router";
import { QueryProvider } from "./providers/QueryProvider";

export default function App() {
  return (
    <QueryProvider>
      <RouterProvider router={router} />
    </QueryProvider>
  );
}