import { Handle, Position } from '@xyflow/react'
import type { GraphNode } from '../../api/types'

interface VisitedNodeProps {
  data: GraphNode & { topicRootId: string }
}

const MOVEMENT_LABELS: Record<string, string> = {
  SUPPORTS: 'Foundation',
  DEEPENS: 'Going deeper',
  RELATES_TO: 'Related',
  APPLIES: 'Applied',
  CONTEXTUALIZES: 'Context',
}

export function VisitedNode({ data }: VisitedNodeProps) {
  const movementLabel = data.movement ? MOVEMENT_LABELS[data.movement] : null
  const resourceCount = data.resources?.length || 0
  const youtubeResource = data.resources?.find((r) => r.youtubeId)

  return (
    <div className="card card-visited">
      <div className="card-header">
        {movementLabel && <span className="card-badge card-badge-visited">{movementLabel}</span>}
      </div>
      <div className="card-title">{data.title}</div>
      {youtubeResource && (
        <div className="card-thumbnail">
          <img
            src={`https://img.youtube.com/vi/${youtubeResource.youtubeId}/mqdefault.jpg`}
            alt={youtubeResource.title}
          />
        </div>
      )}
      <div className="card-meta">
        {resourceCount > 0 && <span className="card-meta-item">{resourceCount} resource{resourceCount !== 1 ? 's' : ''}</span>}
        {data.visitCount > 1 && <span className="card-meta-item">Visited {data.visitCount}x</span>}
      </div>
      <Handle type="target" position={Position.Top} className="card-handle" />
      <Handle type="target" position={Position.Left} className="card-handle" />
      <Handle type="target" position={Position.Right} className="card-handle" />
      <Handle type="source" position={Position.Bottom} className="card-handle" />
    </div>
  )
}
