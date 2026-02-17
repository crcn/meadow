import { useEffect } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { ReactFlowProvider } from '@xyflow/react'
import { useTopicGraphStore } from '../stores/topicGraph'
import { GraphCanvas } from '../components/canvas/GraphCanvas'
import { DetailPanel } from '../components/canvas/DetailPanel'

export function Canvas() {
  const { topicId } = useParams<{ topicId: string }>()
  const navigate = useNavigate()

  if (!topicId) {
    navigate('/')
    return null
  }

  return (
    <ReactFlowProvider>
      <CanvasInner topicRootId={topicId} />
    </ReactFlowProvider>
  )
}

function CanvasInner({ topicRootId }: { topicRootId: string }) {
  const graph = useTopicGraphStore((s) => s.graph)
  const loading = useTopicGraphStore((s) => s.loading)
  const error = useTopicGraphStore((s) => s.error)
  const selectedNodeId = useTopicGraphStore((s) => s.selectedNodeId)
  const backUp = useTopicGraphStore((s) => s.backUp)
  const setSelectedNodeId = useTopicGraphStore((s) => s.setSelectedNodeId)
  const clearError = useTopicGraphStore((s) => s.clearError)
  const init = useTopicGraphStore((s) => s.init)

  useEffect(() => {
    init(topicRootId)
  }, [topicRootId, init])

  const resolved = graph?.nodes.find((n) => n.id === selectedNodeId) ?? null

  if (!graph && loading) {
    return <div className="flex items-center justify-center h-screen text-meadow-muted text-lg">Loading your meadow...</div>
  }

  return (
    <div className="h-screen flex flex-col">
      <div className="flex items-center gap-4 px-5 py-3 bg-meadow-surface border-b border-meadow-border">
        <button
          className="bg-transparent text-meadow-muted text-[13px] px-3 py-1.5 border border-meadow-border rounded-lg hover:border-meadow-accent transition-colors"
          onClick={() => history.back()}
        >
          &larr; Back
        </button>
        <h2 className="flex-1 text-lg font-semibold">{graph?.topicRoot.name}</h2>
        <button
          className="bg-transparent text-meadow-muted text-[13px] px-3 py-1.5 border border-meadow-border rounded-lg hover:border-meadow-accent transition-colors disabled:opacity-50"
          onClick={backUp}
          disabled={loading}
        >
          &#x21A9; Back up
        </button>
      </div>

      <div className="flex-1 relative">
        <GraphCanvas />
        {error && (
          <div className="absolute bottom-4 left-1/2 -translate-x-1/2 bg-red-50 border border-red-200 text-red-700 text-sm px-4 py-2 rounded-lg shadow-md flex items-center gap-3 z-30">
            <span>{error}</span>
            <button onClick={clearError} className="text-red-400 hover:text-red-600 text-lg leading-none">&times;</button>
          </div>
        )}
      </div>

      {resolved && (
        <DetailPanel
          node={resolved}
          onClose={() => setSelectedNodeId(null)}
        />
      )}
    </div>
  )
}
