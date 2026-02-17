import { Handle, Position } from '@xyflow/react'
import type { GraphNode } from '../../api/types'

interface TopicRootNodeProps {
  data: GraphNode & { topicRootId: string }
}

export function TopicRootNode({ data, selected }: TopicRootNodeProps & { selected?: boolean }) {
  return (
    <div className={`px-5 py-4 rounded-xl bg-gradient-to-br from-meadow-accent to-meadow-root text-white text-center min-w-[160px] max-w-[260px] shadow-[0_2px_8px_rgba(0,0,0,0.06),0_0_0_1px_rgba(0,0,0,0.03)] ${selected ? 'ring-2 ring-meadow-accent ring-offset-2' : ''}`}>
      <div className="text-xl mb-1.5">&#x2600;</div>
      <div className="font-semibold text-base leading-tight text-white">{data.title}</div>
      <div className="text-[11px] text-white/75 mt-1 uppercase tracking-wide">Topic Root</div>
      <Handle type="source" position={Position.Bottom} className="card-handle" />
    </div>
  )
}
