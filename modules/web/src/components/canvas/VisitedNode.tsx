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

function extractYoutubeId(url: string | null): string | null {
  if (!url) return null
  const match = url.match(/(?:youtube\.com\/watch\?v=|youtu\.be\/)([a-zA-Z0-9_-]{11})/)
  return match ? match[1] : null
}

export function VisitedNode({ data, selected }: VisitedNodeProps & { selected?: boolean }) {
  const movementLabel = data.movement ? MOVEMENT_LABELS[data.movement] : null
  const resourceCount = data.resources?.length || 0
  const youtubeResource = data.resources?.find((r) => r.youtubeId || extractYoutubeId(r.url))
  const videoId = youtubeResource ? (youtubeResource.youtubeId || extractYoutubeId(youtubeResource.url)) : null

  return (
    <div className={`px-5 py-4 rounded-xl bg-[#faf5ef] text-left min-w-[180px] max-w-[260px] shadow-[0_2px_8px_rgba(0,0,0,0.06),0_0_0_1px_rgba(0,0,0,0.03)] transition-shadow hover:shadow-[0_4px_16px_rgba(0,0,0,0.1),0_0_0_1px_rgba(0,0,0,0.05)] ${selected ? 'border-2 border-meadow-accent' : 'border border-[#ede0d0]'}`}>
      <div className="flex gap-1.5 flex-wrap mb-2">
        {movementLabel && <span className="inline-block text-[10px] font-medium px-2 py-0.5 rounded-full bg-[#f5e6d4] text-[#8c6a4a]">{movementLabel}</span>}
      </div>
      <div className="font-semibold text-sm leading-tight text-[#6b5a48]">{data.title}</div>
      {videoId && (
        <div className="mt-2.5 rounded-lg overflow-hidden border border-meadow-border">
          <div className="card-thumbnail">
            <img
              src={`https://img.youtube.com/vi/${videoId}/mqdefault.jpg`}
              alt={youtubeResource!.title}
            />
          </div>
        </div>
      )}
      <div className="flex gap-2.5 mt-2.5 flex-wrap">
        {resourceCount > 0 && <span className="text-[11px] text-meadow-muted">{resourceCount} resource{resourceCount !== 1 ? 's' : ''}</span>}
        {data.visitCount > 1 && <span className="text-[11px] text-meadow-muted">Visited {data.visitCount}x</span>}
      </div>
      <Handle type="target" position={Position.Top} className="card-handle" />
      <Handle type="source" position={Position.Bottom} className="card-handle" />
    </div>
  )
}
