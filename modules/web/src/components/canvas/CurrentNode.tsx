import { Handle, Position } from '@xyflow/react'
import type { GraphNode } from '../../api/types'

interface CurrentNodeProps {
  data: GraphNode & { topicRootId: string }
}

const MOVEMENT_LABELS: Record<string, string> = {
  SUPPORTS: 'Foundation',
  DEEPENS: 'Going deeper',
  RELATES_TO: 'Related',
  APPLIES: 'Applied',
  CONTEXTUALIZES: 'Context',
}

export function CurrentNode({ data }: CurrentNodeProps) {
  const movementLabel = data.movement ? MOVEMENT_LABELS[data.movement] : null
  const noteCount = data.notes?.length || 0
  const youtubeResource = data.resources?.find((r) => r.youtubeId)
  const otherResourceCount = data.resources?.filter((r) => !r.youtubeId).length || 0

  return (
    <div className="card card-current card-wide">
      <div className="card-header">
        {movementLabel && <span className="card-badge card-badge-current">{movementLabel}</span>}
        <span className="card-badge card-badge-active">You are here</span>
      </div>
      <div className="card-title">{data.title}</div>
      {data.description && (
        <div className="card-description">{data.description.slice(0, 120)}{data.description.length > 120 ? '...' : ''}</div>
      )}
      {youtubeResource && (
        <div className="card-video">
          <iframe
            src={`https://www.youtube.com/embed/${youtubeResource.youtubeId}`}
            title={youtubeResource.title}
            allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
            allowFullScreen
          />
          <div className="card-video-title">{youtubeResource.title}</div>
        </div>
      )}
      <div className="card-meta">
        {otherResourceCount > 0 && <span className="card-meta-item">+{otherResourceCount} more resource{otherResourceCount !== 1 ? 's' : ''}</span>}
        {noteCount > 0 && <span className="card-meta-item">{noteCount} note{noteCount !== 1 ? 's' : ''}</span>}
        {data.visitCount > 1 && <span className="card-meta-item">Visited {data.visitCount}x</span>}
      </div>
      <Handle type="target" position={Position.Top} className="card-handle" />
      <Handle type="target" position={Position.Left} className="card-handle" />
      <Handle type="target" position={Position.Right} className="card-handle" />
      <Handle type="source" position={Position.Bottom} className="card-handle" />
    </div>
  )
}
