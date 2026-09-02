import { cn } from '#/lib/utils';

import type { CommitGraphLayout } from './commit-graph-hooks';

export function CommitGraphSvg({
  layout,
  selectedHash,
}: {
  layout: CommitGraphLayout;
  selectedHash: string | null;
}) {
  return (
    <svg
      className="pointer-events-none absolute left-3 top-0 z-0 h-full overflow-visible"
      width={layout.width}
      height={layout.height}
      viewBox={`0 0 ${layout.width} ${layout.height}`}
      aria-hidden="true"
    >
      {layout.paths.map(path => (
        <path
          key={path.id}
          d={path.d}
          fill="none"
          stroke={path.color}
          strokeLinecap="round"
          strokeLinejoin="round"
          strokeWidth={1.5}
          opacity={0.85}
        />
      ))}

      {layout.nodes.map(node => {
        const selected = node.hash === selectedHash;

        return (
          <g key={node.hash}>
            {selected ? (
              <circle
                cx={node.x}
                cy={node.y}
                r={9}
                fill={node.color}
                opacity={0.18}
                className="transition-opacity"
              />
            ) : null}
            <circle
              cx={node.x}
              cy={node.y}
              r={selected ? 5.5 : 4.5}
              fill="hsl(var(--card))"
              stroke={node.color}
              strokeWidth={selected ? 3 : 2.25}
              className={cn('transition-all', selected && 'drop-shadow-sm')}
            />
          </g>
        );
      })}
    </svg>
  );
}
