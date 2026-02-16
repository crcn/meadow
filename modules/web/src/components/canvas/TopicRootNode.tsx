import { Handle, Position } from '@xyflow/react'
import type { GraphNode } from '../../api/types'

interface TopicRootNodeProps {
  data: GraphNode & { topicRootId: string }
}

export function TopicRootNode({ data }: TopicRootNodeProps) {
  return (
    <div className="node node-root">
      <div className="node-title">{data.title}</div>
      <Handle type="source" position={Position.Bottom} />
    </div>
  )
}
