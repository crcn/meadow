import { Handle, Position } from '@xyflow/react'
import type { GraphNode } from '../../api/types'

const MOVEMENT_LABELS: Record<string, string> = {
  SUPPORTS: 'Foundation',
  DEEPENS: 'Deeper',
  RELATES_TO: 'Lateral',
  APPLIES: 'Creative',
  CONTEXTUALIZES: 'Context',
}

interface ProposalNodeProps {
  data: GraphNode & { topicRootId: string }
}

export function ProposalNode({ data }: ProposalNodeProps) {
  const movementLabel = data.movement ? MOVEMENT_LABELS[data.movement] : null

  return (
    <div className="node node-proposal">
      {movementLabel && <span className="movement-badge">{movementLabel}</span>}
      <div className="node-title">{data.title}</div>
      {data.isWildcard && <span className="wildcard-badge">wildcard</span>}
      <Handle type="target" position={Position.Top} />
      <Handle type="source" position={Position.Bottom} />
    </div>
  )
}
