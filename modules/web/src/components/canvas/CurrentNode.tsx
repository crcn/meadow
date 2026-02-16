import { Handle, Position } from '@xyflow/react'
import type { GraphNode } from '../../api/types'

interface CurrentNodeProps {
  data: GraphNode & { topicRootId: string }
}

export function CurrentNode({ data }: CurrentNodeProps) {
  return (
    <div className="node node-current">
      <div className="node-title">{data.title}</div>
      <div className="node-description">{data.description}</div>
      {data.visitCount > 1 && (
        <div className="node-visitors">{data.visitCount} visits</div>
      )}
      <Handle type="target" position={Position.Top} />
      <Handle type="source" position={Position.Bottom} />
    </div>
  )
}
