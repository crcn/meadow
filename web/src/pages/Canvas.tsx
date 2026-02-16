import { useParams, useNavigate } from 'react-router-dom'
import { ReactFlowProvider } from '@xyflow/react'
import { useTopicGraph } from '../hooks/useTopicGraph'
import { GraphCanvas } from '../components/canvas/GraphCanvas'
import { DetailPanel } from '../components/canvas/DetailPanel'
import { ShowMoreButton } from '../components/canvas/ShowMoreButton'
import { PathBreadcrumb } from '../components/canvas/PathBreadcrumb'
import type { GraphNode, Movement } from '../api/types'

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
  const {
    graph,
    flowNodes,
    flowEdges,
    selectedNode,
    setSelectedNodeId,
    loading,
    traverse,
    backUp,
    showMore,
    leaveNote,
    askAboutNode,
  } = useTopicGraph(topicRootId)

  const handleNodeClick = (_nodeId: string, node: GraphNode) => {
    setSelectedNodeId(node.id)
  }

  const handleTraverse = async (fromNodeId: string, toNodeId: string, movement: Movement) => {
    setSelectedNodeId(null)
    await traverse(fromNodeId, toNodeId, movement)
  }

  const handleShowMore = async () => {
    if (!graph) return
    const currentNode = graph.nodes.find((n) => n.id === graph.currentNodeId)
    if (currentNode) {
      await showMore(currentNode.id, currentNode.title)
    }
  }

  const handleBreadcrumbClick = (nodeId: string) => {
    const node = graph?.nodes.find((n) => n.id === nodeId)
    if (node) setSelectedNodeId(nodeId)
  }

  if (!graph && loading) {
    return <div className="canvas-loading">Loading your meadow...</div>
  }

  return (
    <div className="canvas-page">
      <div className="canvas-header">
        <button className="back-button" onClick={() => history.back()}>
          ← Back
        </button>
        <h2>{graph?.topicRoot.name}</h2>
        <button className="backup-button" onClick={backUp} disabled={loading}>
          ↩ Back up
        </button>
      </div>

      {graph && (
        <PathBreadcrumb nodes={graph.nodes} onNodeClick={handleBreadcrumbClick} />
      )}

      <div className="canvas-container">
        <GraphCanvas
          nodes={flowNodes}
          edges={flowEdges}
          currentNodeId={graph?.currentNodeId}
          onNodeClick={handleNodeClick}
          onTraverse={handleTraverse}
        />

        <ShowMoreButton onClick={handleShowMore} loading={loading} />
      </div>

      {selectedNode && (
        <DetailPanel
          node={selectedNode}
          onClose={() => setSelectedNodeId(null)}
          onLeaveNote={leaveNote}
          onAsk={(question) => askAboutNode(selectedNode.id, question)}
        />
      )}
    </div>
  )
}
