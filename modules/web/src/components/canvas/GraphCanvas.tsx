import { useCallback, useMemo, useEffect } from 'react'
import {
  ReactFlow,
  Background,
  Controls,
  useReactFlow,
  type NodeMouseHandler,
  type NodeTypes,
  type EdgeTypes,
} from '@xyflow/react'
import '@xyflow/react/dist/style.css'
import type { GraphNode, Movement } from '../../api/types'
import { NodeState } from '../../api/types'
import { useTopicGraphStore } from '../../stores/topicGraph'
import { TopicRootNode } from './TopicRootNode'
import { CurrentNode } from './CurrentNode'
import { VisitedNode } from './VisitedNode'
import { ProposalNode } from './ProposalNode'
import { MovementEdge } from './MovementEdge'

const nodeTypes: NodeTypes = {
  topicRoot: TopicRootNode,
  current: CurrentNode,
  visited: VisitedNode,
  proposal: ProposalNode,
}

const edgeTypes: EdgeTypes = {
  movement: MovementEdge,
}

export function GraphCanvas() {
  const flowNodes = useTopicGraphStore((s) => s.flowNodes)
  const flowEdges = useTopicGraphStore((s) => s.flowEdges)
  const selectedNodeId = useTopicGraphStore((s) => s.selectedNodeId)

  const nodesWithSelection = useMemo(
    () => flowNodes.map((n) => ({ ...n, selected: n.id === selectedNodeId })),
    [flowNodes, selectedNodeId],
  )

  const { fitView } = useReactFlow()

  // Fit view on first render
  useEffect(() => {
    if (flowNodes.length > 0) {
      setTimeout(() => fitView({ padding: 0.2, duration: 300 }), 100)
    }
  }, [flowNodes.length > 0]) // eslint-disable-line react-hooks/exhaustive-deps

  const handleNodeClick: NodeMouseHandler = useCallback(
    (_event, node) => {
      const graphNode = node.data as unknown as GraphNode
      // Read from store at click-time to avoid stale closures
      const { graph: currentGraph, traverse, showMore, setSelectedNodeId } =
        useTopicGraphStore.getState()
      const curId = currentGraph?.currentNodeId

      if (graphNode.state === NodeState.PROPOSAL && graphNode.movement && curId) {
        // Proposal → traverse (adds node to graph, generates proposals)
        traverse(curId, node.id, graphNode.movement as Movement)
      } else {
        // Select the node (opens detail panel)
        setSelectedNodeId(node.id)
        // For visited / root → also kick off exploration
        if (graphNode.state === NodeState.VISITED || graphNode.state === NodeState.TOPIC_ROOT) {
          showMore(node.id, graphNode.title)
        }
      }
    },
    [],
  )

  return (
    <ReactFlow
      nodes={nodesWithSelection}
      edges={flowEdges}
      nodeTypes={nodeTypes}
      edgeTypes={edgeTypes}
      onNodeClick={handleNodeClick}
      fitView
      minZoom={0.2}
      maxZoom={2}
      proOptions={{ hideAttribution: true }}
    >
      <Background />
      <Controls />
    </ReactFlow>
  )
}
