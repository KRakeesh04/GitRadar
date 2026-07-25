import { tauri } from "./tauri";

export type IndexingJobStatus = "pending" | "running" | "completed" | "failed" | string;

export interface IndexingJob {
  id: number;
  repoId: number;
  jobType: string;
  status: IndexingJobStatus;
  progress: number;
  totalItems: number | null;
  processedItems: number;
  errorMessage: string | null;
  startedAt: string | null;
  completedAt: string | null;
  createdAt: string;
  updatedAt: string;
}

export interface SyncProgressEvent {
  repo_id: number;
  job_id: number;
  progress: number;
  processed_items: number;
  total_items: number;
  status: IndexingJobStatus;
}

interface IndexingJobResponse {
  id: number;
  repo_id: number;
  job_type: string;
  status: IndexingJobStatus;
  progress: number;
  total_items: number | null;
  processed_items: number;
  error_message: string | null;
  started_at: string | null;
  completed_at: string | null;
  created_at: string;
  updated_at: string;
}

function toIndexingJob(job: IndexingJobResponse): IndexingJob {
  return {
    id: job.id,
    repoId: job.repo_id,
    jobType: job.job_type,
    status: job.status,
    progress: job.progress,
    totalItems: job.total_items,
    processedItems: job.processed_items,
    errorMessage: job.error_message,
    startedAt: job.started_at,
    completedAt: job.completed_at,
    createdAt: job.created_at,
    updatedAt: job.updated_at,
  };
}

export function startRepositorySync(repoId: number): Promise<void> {
  // Tauri exposes Rust's `repo_id` argument as `repoId` in JavaScript.
  return tauri<void>("sync_repository", { repoId });
}

export async function syncRepositories(repoIds: number[]): Promise<void> {
  // Keep syncs sequential because each command writes to the same SQLite database.
  for (const repoId of repoIds) {
    await startRepositorySync(repoId);
  }
}

export async function getLatestIndexingJob(repoId: number): Promise<IndexingJob | null> {
  const job = await tauri<IndexingJobResponse | null>("get_latest_indexing_job_by_repo", {
    repoId,
  });
  return job ? toIndexingJob(job) : null;
}

export async function getIndexingJobs(repoId: number, limit = 20): Promise<IndexingJob[]> {
  const jobs = await tauri<IndexingJobResponse[]>("get_indexing_jobs_by_repo", {
    repoId,
    limit,
  });
  return jobs.map(toIndexingJob);
}

export async function getPendingIndexingJobs(): Promise<IndexingJob[]> {
  const jobs = await tauri<IndexingJobResponse[]>("get_pending_indexing_jobs");
  return jobs.map(toIndexingJob);
}

export function cleanupCompletedIndexingJobs(daysOld: number): Promise<number> {
  return tauri<number>("cleanup_completed_indexing_jobs", { daysOld });
}
