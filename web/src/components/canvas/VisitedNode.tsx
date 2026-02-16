import { Handle, Position } from '@xyflow/react'
import type { GraphNode } from '../../api/types'

interface VisitedNodeProps {
  data: GraphNode & { topicRootId: string }
}

export function VisitedNode({ data }: VisitedNodeProps) {
  return (
    <div className="node node-visited">
      <div className="node-title">{data.title}</div>
      <Handle type="target" position={Position.Top} />
      <Handle type="source" position={Position.Bottom} />
    </div>
  )
}
