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

export function ProposalNode({ data }: ProposalNodeProps) {
  const movementLabel = data.movement ? MOVEMENT_LABELS[data.movement] : null
  const movementIcon = data.movement ? MOVEMENT_ICONS[data.movement] : null

  return (
    <div className="card card-proposal">
      <div className="card-header">
        {movementLabel && (
          <span className="card-badge card-badge-proposal">
            {movementIcon} {movementLabel}
          </span>
        )}
      </div>
      <div className="card-title">{data.title}</div>
      {data.description && (
        <div className="card-description">{data.description.slice(0, 60)}{data.description.length > 60 ? '...' : ''}</div>
      )}
      {data.isWildcard && <span className="card-badge card-badge-wildcard">Wildcard</span>}
      <div className="card-cta">Click to explore</div>
      <Handle type="target" position={Position.Top} className="card-handle" />
      <Handle type="target" position={Position.Left} className="card-handle" />
      <Handle type="target" position={Position.Right} className="card-handle" />
      <Handle type="source" position={Position.Bottom} className="card-handle" />
    </div>
  )
}
