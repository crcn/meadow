import { Handle, Position } from '@xyflow/react'
import type { GraphNode } from '../../api/types'
import { useTopicGraphStore } from '../../stores/topicGraph'

interface CurrentNodeProps {
  data: GraphNode & { topicRootId: string; loading?: boolean; showMoreFromStore?: boolean }
}

const MOVEMENT_LABELS: Record<string, string> = {
  DEEPER: 'Go deeper',
  BROADER: 'Explore',
  FOUNDATION: 'Foundation',
  PRACTICE: 'Practice',
  INSPIRE: 'Inspire',
}

function extractYoutubeId(url: string | null): string | null {
  if (!url) return null
  const match = url.match(/(?:youtube\.com\/watch\?v=|youtu\.be\/)([a-zA-Z0-9_-]{11})/)
  return match ? match[1] : null
}

export function CurrentNode({ data, selected }: CurrentNodeProps & { selected?: boolean }) {
  const movementLabel = data.movement ? MOVEMENT_LABELS[data.movement] : null
  const noteCount = data.notes?.length || 0
  const youtubeResource = data.resources?.find((r) => r.youtubeId || extractYoutubeId(r.url))
  const videoId = youtubeResource ? (youtubeResource.youtubeId || extractYoutubeId(youtubeResource.url)) : null
  const otherResourceCount = data.resources?.filter((r) => r !== youtubeResource).length || 0

  const handleShowMore = () => {
    useTopicGraphStore.getState().showMore(data.id)
  }

  return (
    <div className={`px-5 py-4 rounded-xl border-2 border-meadow-accent bg-gradient-to-b from-[#fff9f0] to-white text-left min-w-[300px] max-w-[360px] shadow-[0_2px_8px_rgba(0,0,0,0.06),0_0_0_1px_rgba(0,0,0,0.03)] transition-shadow hover:shadow-[0_4px_16px_rgba(0,0,0,0.1),0_0_0_1px_rgba(0,0,0,0.05)] ${selected ? 'ring-2 ring-meadow-accent ring-offset-2' : ''}`}>
      <div className="flex gap-1.5 flex-wrap mb-2">
        {movementLabel && <span className="inline-block text-[10px] font-medium px-2 py-0.5 rounded-full bg-[#fff0db] text-[#a0632a]">{movementLabel}</span>}
        <span className="inline-block text-[10px] font-medium px-2 py-0.5 rounded-full bg-meadow-accent text-white">You are here</span>
      </div>
      <div className="font-semibold text-sm leading-tight text-meadow-text">{data.title}</div>
      {data.description && (
        <div className="text-xs text-meadow-muted mt-1.5 leading-snug">{data.description.slice(0, 120)}{data.description.length > 120 ? '...' : ''}</div>
      )}
      {videoId && (
        <div className="mt-2.5 rounded-lg overflow-hidden border border-meadow-border">
          <div className="card-video">
            <iframe
              src={`https://www.youtube.com/embed/${videoId}`}
              title={youtubeResource!.title}
              allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
              allowFullScreen
            />
          </div>
          <div className="px-2.5 py-1.5 text-[11px] text-meadow-muted bg-meadow-bg">{youtubeResource!.title}</div>
        </div>
      )}
      <div className="flex gap-2.5 mt-2.5 flex-wrap">
        {otherResourceCount > 0 && <span className="text-[11px] text-meadow-muted">+{otherResourceCount} more resource{otherResourceCount !== 1 ? 's' : ''}</span>}
        {noteCount > 0 && <span className="text-[11px] text-meadow-muted">{noteCount} note{noteCount !== 1 ? 's' : ''}</span>}
        {data.visitCount > 1 && <span className="text-[11px] text-meadow-muted">Visited {data.visitCount}x</span>}
      </div>
      {data.loading ? (
        <div className="mt-3 flex items-center gap-2 text-[11px] text-meadow-accent">
          <span className="inline-block w-3.5 h-3.5 border-2 border-meadow-accent border-t-transparent rounded-full animate-spin" />
          Exploring...
        </div>
      ) : data.showMoreFromStore ? (
        <button
          onClick={(e) => { e.stopPropagation(); handleShowMore() }}
          className="mt-3 w-full text-[11px] py-1.5 text-meadow-accent border border-dashed border-meadow-accent/40 rounded-lg hover:bg-meadow-accent/5 transition-colors"
        >
          Show me more
        </button>
      ) : null}
      <Handle type="target" position={Position.Top} className="card-handle" />
      <Handle type="source" position={Position.Bottom} className="card-handle" />
    </div>
  )
}
