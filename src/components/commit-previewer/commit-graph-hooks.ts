import { invoke } from '@tauri-apps/api/core';
import { useCallback, useEffect, useMemo, useRef, useState } from 'react';

import type { CommitGraphNode } from './types';

export const COMMIT_ROW_HEIGHT = 76;
export const GRAPH_MIN_WIDTH = 72;
export const GRAPH_LANE_WIDTH = 15;
export const GRAPH_TOP_PADDING = COMMIT_ROW_HEIGHT / 2;

const GRAPH_COLORS = [
  '#3b82f6',
  '#22c55e',
  '#f59e0b',
  '#ef4444',
  '#a855f7',
  '#06b6d4',
  '#ec4899',
  '#84cc16',
];

export type CommitGraphLayoutNode = {
  hash: string;
  lane: number;
  x: number;
  y: number;
  color: string;
};

export type CommitGraphLayoutPath = {
  id: string;
  d: string;
  color: string;
};

export type CommitGraphLayout = {
  nodes: CommitGraphLayoutNode[];
  paths: CommitGraphLayoutPath[];
  width: number;
  height: number;
};

export function useCommitGraphInfinite(repoId: string, limit = 50) {
  const [commits, setCommits] = useState<CommitGraphNode[]>([]);
  const [selectedHash, setSelectedHash] = useState<string | null>(null);
  const [offset, setOffset] = useState(0);
  const [hasMore, setHasMore] = useState(true);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const seenHashesRef = useRef(new Set<string>());

  const loadNextPage = useCallback(async () => {
    if (isLoading || !hasMore) return;

    setIsLoading(true);
    setError(null);

    try {
      const page = await invoke<CommitGraphNode[]>('get_commit_graph', {
        repoId: Number(repoId),
        limit,
        offset,
      });

      if (page.length === 0) {
        setHasMore(false);
        return;
      }

      const nextCommits = page.filter(commit => {
        if (seenHashesRef.current.has(commit.hash)) return false;
        seenHashesRef.current.add(commit.hash);
        return true;
      });

      setCommits(current => {
        const merged = [...current, ...nextCommits];
        if (!selectedHash && merged.length > 0) {
          setSelectedHash(merged[0].hash);
        }
        return merged;
      });
      setOffset(current => current + limit);
      if (page.length < limit) setHasMore(false);
    } catch (unknownError) {
      setError(unknownError instanceof Error ? unknownError.message : String(unknownError));
    } finally {
      setIsLoading(false);
    }
  }, [hasMore, isLoading, limit, offset, repoId, selectedHash]);

  useEffect(() => {
    setCommits([]);
    setSelectedHash(null);
    setOffset(0);
    setHasMore(true);
    setError(null);
    seenHashesRef.current = new Set();
  }, [repoId, limit]);

  useEffect(() => {
    if (commits.length === 0 && offset === 0 && hasMore && !isLoading) {
      void loadNextPage();
    }
  }, [commits.length, hasMore, isLoading, loadNextPage, offset]);

  const selectedCommit = useMemo(
    () => commits.find(commit => commit.hash === selectedHash) ?? commits[0] ?? null,
    [commits, selectedHash]
  );

  return {
    commits,
    selectedHash,
    selectedCommit,
    setSelectedHash,
    loadNextPage,
    hasMore,
    isLoading,
    error,
  };
}

export function useLastRowObserver({
  isLoading,
  hasMore,
  onLoadMore,
}: {
  isLoading: boolean;
  hasMore: boolean;
  onLoadMore: () => void;
}) {
  const observerRef = useRef<IntersectionObserver | null>(null);

  return useCallback(
    (node: HTMLDivElement | null) => {
      observerRef.current?.disconnect();
      if (!node || isLoading || !hasMore) return;

      observerRef.current = new IntersectionObserver(
        entries => {
          if (entries[0]?.isIntersecting) onLoadMore();
        },
        { root: null, rootMargin: '320px 0px', threshold: 0.01 }
      );
      observerRef.current.observe(node);
    },
    [hasMore, isLoading, onLoadMore]
  );
}

export function useCommitGraphLayout(commits: CommitGraphNode[]): CommitGraphLayout {
  return useMemo(() => {
    const layout = buildCommitGraphLayout(commits);

    return {
      ...layout,
      width: Math.max(GRAPH_MIN_WIDTH, layout.width),
      height: Math.max(COMMIT_ROW_HEIGHT, layout.height),
    };
  }, [commits]);
}

function buildCommitGraphLayout(commits: CommitGraphNode[]): CommitGraphLayout {
  const activeLanes: Array<string | null> = [];
  const nodes: CommitGraphLayoutNode[] = [];
  const paths: CommitGraphLayoutPath[] = [];
  let maxLaneUsed = 0;
  const knownHashes = new Set(commits.map(commit => commit.hash));

  for (let index = 0; index < commits.length; index += 1) {
    appendCommitLayout(
      {
        activeLanes,
        nodes,
        paths,
        setMaxLane: lane => {
          maxLaneUsed = Math.max(maxLaneUsed, lane);
        },
      },
      commits[index],
      index,
      commits.length,
      knownHashes,
    );
  }

  const laneCount = Math.max(
    1,
    ...nodes.map(node => node.lane + 1),
    activeLanes.length,
    maxLaneUsed + 1,
  );

  return {
    nodes,
    paths,
    width: laneCount * GRAPH_LANE_WIDTH + 36,
    height: Math.max(COMMIT_ROW_HEIGHT, commits.length * COMMIT_ROW_HEIGHT),
  };
}

function appendCommitLayout(
  layout: {
    activeLanes: Array<string | null>;
    nodes: CommitGraphLayoutNode[];
    paths: CommitGraphLayoutPath[];
    setMaxLane: (lane: number) => void;
  },
  commit: CommitGraphNode,
  index: number,
  commitCount: number,
  knownHashes: Set<string>,
) {
  const activeLanes = layout.activeLanes;
  let lane = activeLanes.indexOf(commit.hash);

  if (lane === -1) {
    lane = firstOpenLane(activeLanes);
    activeLanes[lane] = commit.hash;
  }
  layout.setMaxLane(lane);

  const x = laneToX(lane);
  const y = indexToY(index);
  const color = laneColor(lane);

  layout.nodes.push({
    hash: commit.hash,
    lane,
    x,
    y,
    color,
  });

  const firstParent = commit.parent_hashes[0];
  const remainingParents = commit.parent_hashes.slice(1);
  const nextY = index + 1 < commitCount
    ? indexToY(index + 1)
    : (index + 1) * COMMIT_ROW_HEIGHT;

  const graphBottom = commitCount * COMMIT_ROW_HEIGHT;

  if (firstParent && knownHashes.has(firstParent)) {
    const existingParentLane = activeLanes.indexOf(firstParent);
    if (existingParentLane !== -1 && existingParentLane !== lane) {
      activeLanes[existingParentLane] = null;
    }
    activeLanes[lane] = firstParent;
    layout.paths.push({
      id: `${commit.hash}-${firstParent}-main`,
      d: `M ${x} ${y} L ${x} ${nextY}`,
      color,
    });
  } else if (firstParent) {
    activeLanes[lane] = null;
    layout.paths.push({
      id: `${commit.hash}-${firstParent}-continuation`,
      d: `M ${x} ${y} L ${x} ${graphBottom}`,
      color,
    });
  } else {
    activeLanes[lane] = null;
  }

  for (const parentHash of remainingParents) {
    if (!knownHashes.has(parentHash)) {
      const continuationLane = firstOpenLane(activeLanes);
      layout.setMaxLane(continuationLane);
      const continuationX = laneToX(continuationLane);
      const controlY = y + COMMIT_ROW_HEIGHT * 0.45;

      layout.paths.push({
        id: `${commit.hash}-${parentHash}-continuation`,
        d: `M ${x} ${y} C ${x} ${controlY}, ${continuationX} ${controlY}, ${continuationX} ${graphBottom}`,
        color: laneColor(continuationLane),
      });
      continue;
    }

    let parentLane = activeLanes.indexOf(parentHash);
    if (parentLane === -1) {
      parentLane = firstOpenLane(activeLanes);
      activeLanes[parentLane] = parentHash;
    }
    layout.setMaxLane(parentLane);

    const parentX = laneToX(parentLane);
    const controlY = y + COMMIT_ROW_HEIGHT * 0.45;

    layout.paths.push({
      id: `${commit.hash}-${parentHash}-merge`,
      d: `M ${x} ${y} C ${x} ${controlY}, ${parentX} ${controlY}, ${parentX} ${nextY}`,
      color: laneColor(parentLane),
    });
  }

  trimTrailingEmptyLanes(activeLanes);
}

function firstOpenLane(lanes: Array<string | null>) {
  const openIndex = lanes.findIndex(value => value === null);
  return openIndex === -1 ? lanes.length : openIndex;
}

function trimTrailingEmptyLanes(lanes: Array<string | null>) {
  while (lanes.length > 0 && lanes[lanes.length - 1] === null) {
    lanes.pop();
  }
}

function laneToX(lane: number) {
  return 18 + lane * GRAPH_LANE_WIDTH;
}

function indexToY(index: number) {
  return GRAPH_TOP_PADDING + index * COMMIT_ROW_HEIGHT;
}

function laneColor(lane: number) {
  return GRAPH_COLORS[lane % GRAPH_COLORS.length];
}
