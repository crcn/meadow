import { Handle, Position } from '@xyflow/react'
import type { GraphNode } from '../../api/types'

interface TopicRootNodeProps {
  data: GraphNode & { topicRootId: string }
}

export function TopicRootNode({ data }: TopicRootNodeProps) {
  return (
    <div className="card card-root">
      <div className="card-icon">&#x2600;</div>
      <div className="card-title">{data.title}</div>
      <div className="card-subtitle">Topic Root</div>
      <Handle type="source" position={Position.Bottom} className="card-handle" />
      <Handle type="source" position={Position.Left} className="card-handle" />
      <Handle type="source" position={Position.Right} className="card-handle" />
      <Handle type="target" position={Position.Top} className="card-handle" />
    </div>
  )
}
