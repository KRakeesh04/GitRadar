import { tauri } from './tauri';

export interface SearchHit {
  repoId: number;
  repoName: string;
  entityType: string;
  entityId: number;
  title: string;
  body: string;
}

interface SearchHitResponse {
  repo_id: number;
  repo_name: string;
  entity_type: string;
  entity_id: number;
  title: string;
  body: string;
}

export interface SearchResponse {
  query: string;
  items: SearchHit[];
  totalCount: number;
}

interface SearchResponseData {
  query: string;
  items: SearchHitResponse[];
  total_count: number;
}

function toSearchHit(hit: SearchHitResponse): SearchHit {
  return {
    repoId: hit.repo_id,
    repoName: hit.repo_name,
    entityType: hit.entity_type,
    entityId: hit.entity_id,
    title: hit.title,
    body: hit.body,
  };
}

export async function searchEverything(
  query: string,
  limit?: number,
  offset?: number
): Promise<SearchResponse> {
  const response = await tauri<SearchResponseData>('search_everything', {
    query,
    limit: limit ?? 50,
    offset: offset ?? 0,
  });
  return {
    query: response.query,
    items: response.items.map(toSearchHit),
    totalCount: response.total_count,
  };
}

export function reindexSearchIndex(repoId: number): Promise<void> {
  return tauri<void>('reindex_search_index', { repoId });
}
