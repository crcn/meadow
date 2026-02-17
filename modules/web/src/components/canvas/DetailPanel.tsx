import type { GraphNode } from '../../api/types'
import { useTopicGraphStore } from '../../stores/topicGraph'
import { ResourcePreview } from './ResourcePreview'

interface DetailPanelProps {
  node: GraphNode
  onClose: () => void
}

export function DetailPanel({ node, onClose }: DetailPanelProps) {
  const loading = useTopicGraphStore((s) => s.loading)
  const upvoteResource = useTopicGraphStore((s) => s.upvoteResource)
  const loadMoreResources = useTopicGraphStore((s) => s.loadMoreResources)

  return (
    <div className="fixed right-0 top-0 bottom-0 w-[30vw] bg-meadow-surface border-l border-meadow-border p-6 overflow-y-auto z-20 shadow-[-4px_0_16px_rgba(0,0,0,0.06)]">
      <div className="flex justify-between items-start mb-4">
        <h3 className="text-lg font-semibold">{node.title}</h3>
        <button className="bg-none text-meadow-muted text-2xl px-1 leading-none cursor-pointer hover:text-meadow-text" onClick={onClose}>&times;</button>
      </div>

      <p className="text-meadow-muted text-sm mb-6 leading-relaxed">{node.description}</p>

      {node.resources.length > 0 && (
        <div>
          <h4 className="text-xs uppercase tracking-wide text-meadow-muted mb-3">Resources</h4>
          {node.resources.map((r, i) => (
            <ResourcePreview
              key={i}
              resource={r}
              onUpvote={() => upvoteResource(node.id, i)}
            />
          ))}
          <button
            onClick={() => loadMoreResources(node.id)}
            disabled={loading}
            className="w-full text-[12px] py-2 text-meadow-muted border border-dashed border-meadow-border rounded-lg hover:border-meadow-accent hover:text-meadow-accent transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {loading ? 'Finding more...' : 'Load more resources'}
          </button>
        </div>
      )}
    </div>
  )
}
