import { tauri } from "./tauri";

interface RepositoryActivityDailyResponse {
  id: number,
  repo_id: number,
  activity_date: string,
  commit_count: number,
  additions: number,
  deletions: number,
  files_changed: number,
}

export interface RepositoryActivityDaily {
  id: number,
  repoId: number,
  activityDate: string,
  commitCount: number,
  additions: number,
  deletions: number,
  filesChanged: number,
}

function toRepositoryActivityDaily(activity: RepositoryActivityDailyResponse): RepositoryActivityDaily {
  return {
    id: activity.id,
    repoId: activity.repo_id,
    activityDate: activity.activity_date,
    commitCount: activity.commit_count,
    additions: activity.additions,
    deletions: activity.deletions,
    filesChanged: activity.files_changed,
  };
}

export interface Contributor {
  id: number,
  repoId: number,
  name: string,
  email: string,
  commitCount: number,
  additions: number,
  deletions: number,
  activeDays: number,
  lastCommitAt: string | null,
  impactScore: number,
  contributorLevel: string,
  isActive: boolean,
}

interface ContributorResponse {
  id: number,
  repo_id: number,
  name: string,
  email: string,
  commit_count: number,
  additions: number,
  deletions: number,
  active_days: number,
  last_commit_at: string | null,
  impact_score: number,
  contributor_level: string,
  is_active: boolean,
}

function toContributor(contributor: ContributorResponse): Contributor {
  return {
    id: contributor.id,
    repoId: contributor.repo_id,
    name: contributor.name,
    email: contributor.email,
    commitCount: contributor.commit_count,
    additions: contributor.additions,
    deletions: contributor.deletions,
    activeDays: contributor.active_days,
    lastCommitAt: contributor.last_commit_at,
    impactScore: contributor.impact_score,
    contributorLevel: contributor.contributor_level,
    isActive: contributor.is_active,
  };
}

export interface LanguageStat {
  language: string,
  bytes: number,
}

export interface LanguageStatsResponse {
  total_bytes: number,
  languages: LanguageStat[],
}

export async function getContributorsByRepoId(repoId: number): Promise<Contributor[]> {
  const contributors = await tauri<ContributorResponse[]>('get_contributors', { repoId });
  return contributors.map(toContributor);
}

export async function getTopContributorsByRepoId(repoId: number, limit: number): Promise<Contributor[]> {
  const contributors = await tauri<ContributorResponse[]>('get_top_contributors', { repoId, limit });
  return contributors.map(toContributor);
}

export async function getContributorByEmail(repoId: number, email: string): Promise<Contributor | null> {
  const contributor = await tauri<ContributorResponse | null>('get_contributor_by_email', { repoId, email });
  return contributor ? toContributor(contributor) : null;
}

export async function getRepositoryActivityDaily(repoId: number, startDate: string | null, endDate: string | null): Promise<RepositoryActivityDaily[]> {
  const activities = await tauri<RepositoryActivityDailyResponse[]>('get_repository_activity', { repoId, startDate, endDate });
  return activities.map(toRepositoryActivityDaily);
}
export async function getRepoLanguagesStats(repoId: number): Promise<LanguageStatsResponse> {
  const languagesStatsResponse = await tauri<LanguageStatsResponse>('get_repo_languages_stats', { repoId });
  return languagesStatsResponse;
}
