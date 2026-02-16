import { useCallback, useRef, useEffect } from 'react'
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
import type { Node, Edge } from '@xyflow/react'
import type { GraphNode, Movement } from '../../api/types'
import { NodeState } from '../../api/types'
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

interface GraphCanvasProps {
  nodes: Node[]
  edges: Edge[]
  currentNodeId: string | undefined
  onNodeClick: (nodeId: string, node: GraphNode) => void
  onTraverse: (fromNodeId: string, toNodeId: string, movement: Movement) => void
}

export function GraphCanvas({
  nodes,
  edges,
  currentNodeId,
  onNodeClick,
  onTraverse,
}: GraphCanvasProps) {
  const { fitView, setCenter } = useReactFlow()
  const prevCurrentRef = useRef(currentNodeId)

  // Pan to current node when it changes
  useEffect(() => {
    if (currentNodeId && currentNodeId !== prevCurrentRef.current) {
      const currentNode = nodes.find((n) => n.id === currentNodeId)
      if (currentNode) {
        setCenter(currentNode.position.x, currentNode.position.y, {
          zoom: 1,
          duration: 500,
        })
      }
      prevCurrentRef.current = currentNodeId
    }
  }, [currentNodeId, nodes, setCenter])

  // Fit view on first render
  useEffect(() => {
    if (nodes.length > 0) {
      setTimeout(() => fitView({ padding: 0.2, duration: 300 }), 100)
    }
  }, [nodes.length > 0]) // eslint-disable-line react-hooks/exhaustive-deps

  const handleNodeClick: NodeMouseHandler = useCallback(
    (_event, node) => {
      const graphNode = node.data as unknown as GraphNode
      if (graphNode.state === NodeState.PROPOSAL && graphNode.movement && currentNodeId) {
        onTraverse(currentNodeId, node.id, graphNode.movement)
      } else {
        onNodeClick(node.id, graphNode)
      }
    },
    [currentNodeId, onNodeClick, onTraverse],
  )

  return (
    <ReactFlow
      nodes={nodes}
      edges={edges}
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
