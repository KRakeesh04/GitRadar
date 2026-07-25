export function getPathLabel(path: string) {
  return path.split(/[\\/]/).filter(Boolean).at(-1) || path;
}
export function formatUpdatedAt(value: string) {
  const date = new Date(value);
  return Number.isNaN(date.getTime()) ? value : date.toLocaleString();
}
export function getErrorMessage(error: unknown) {
  return error instanceof Error ? error.message : String(error);
}