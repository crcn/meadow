import { Handle, Position } from '@xyflow/react'
import type { GraphNode } from '../../api/types'

const MOVEMENT_LABELS: Record<string, string> = {
  SUPPORTS: 'Foundation',
  DEEPENS: 'Going deeper',
  RELATES_TO: 'Related',
  APPLIES: 'Applied',
  CONTEXTUALIZES: 'Context',
}

const MOVEMENT_ICONS: Record<string, string> = {
  SUPPORTS: '\u2193',
  DEEPENS: '\u2195',
  RELATES_TO: '\u2194',
  APPLIES: '\u2192',
  CONTEXTUALIZES: '\u21BB',
}

interface ProposalNodeProps {
  data: GraphNode & { topicRootId: string }
}

export function ProposalNode({ data, selected }: ProposalNodeProps & { selected?: boolean }) {
  const movementLabel = data.movement ? MOVEMENT_LABELS[data.movement] : null
  const movementIcon = data.movement ? MOVEMENT_ICONS[data.movement] : null

  return (
    <div className="px-5 py-4 rounded-xl border-2 border-dashed border-[#e8d4b8] bg-[#fffcf8] text-left min-w-[180px] max-w-[260px] cursor-pointer shadow-[0_2px_8px_rgba(0,0,0,0.06),0_0_0_1px_rgba(0,0,0,0.03)] transition-all hover:border-meadow-accent hover:bg-[#fff8ee] hover:-translate-y-0.5 hover:shadow-[0_4px_16px_rgba(0,0,0,0.1),0_0_0_1px_rgba(0,0,0,0.05)]">
      <div className="flex gap-1.5 flex-wrap mb-2">
        {movementLabel && (
          <span className="inline-block text-[10px] font-medium px-2 py-0.5 rounded-full bg-[#faf0e4] text-[#b07830]">
            {movementIcon} {movementLabel}
          </span>
        )}
      </div>
      <div className="font-semibold text-sm leading-tight text-meadow-text">{data.title}</div>
      {data.description && (
        <div className="text-xs text-meadow-muted mt-1.5 leading-snug">{data.description.slice(0, 60)}{data.description.length > 60 ? '...' : ''}</div>
      )}
      {data.isWildcard && <span className="inline-block text-[10px] font-medium px-2 py-0.5 rounded-full bg-[#fce8d0] text-[#a0632a] mt-2">Wildcard</span>}
      <div className="mt-2.5 text-[11px] text-meadow-accent font-medium">Click to explore</div>
      <Handle type="target" position={Position.Top} className="card-handle" />
      <Handle type="source" position={Position.Bottom} className="card-handle" />
    </div>
  )
}
