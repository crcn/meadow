import { useCallback, useEffect, useRef } from 'react'
import {
  ReactFlow,
  Background,
  Controls,
  useReactFlow,
  useNodesInitialized,
  useNodesState,
  useEdgesState,
  type NodeMouseHandler,
  type NodeTypes,
  type EdgeTypes,
  type Node,
  type Edge,
} from '@xyflow/react'
import dagre from '@dagrejs/dagre'
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

// ─── Dagre layout ────────────────────────────────────────────────────

function applyDagreLayout(nodes: Node[], edges: Edge[]): Node[] {
  if (nodes.length === 0) return nodes

  const g = new dagre.graphlib.Graph()
  g.setDefaultEdgeLabel(() => ({}))
  g.setGraph({ rankdir: 'TB', nodesep: 80, ranksep: 100 })

  for (const node of nodes) {
    const w = node.measured?.width ?? 260
    const h = node.measured?.height ?? 100
    g.setNode(node.id, { width: w, height: h })
  }

  for (const edge of edges) {
    g.setEdge(edge.source, edge.target)
  }

  dagre.layout(g)

  return nodes.map((node) => {
    const pos = g.node(node.id)
    const w = node.measured?.width ?? 260
    const h = node.measured?.height ?? 100
    return {
      ...node,
      position: { x: pos.x - w / 2, y: pos.y - h / 2 },
    }
  })
}

// ─── GraphCanvas ─────────────────────────────────────────────────────

export function GraphCanvas() {
  const flowNodes = useTopicGraphStore((s) => s.flowNodes)
  const flowEdges = useTopicGraphStore((s) => s.flowEdges)
  const selectedNodeId = useTopicGraphStore((s) => s.selectedNodeId)

  // React Flow owns node/edge state internally (uncontrolled).
  // We push new data in when the store changes.
  const [nodes, setNodes, onNodesChange] = useNodesState([])
  const [edges, setEdges, onEdgesChange] = useEdgesState([])

  const { getNodes, fitView } = useReactFlow()
  const nodesInitialized = useNodesInitialized()
  const layoutDoneForKey = useRef('')
  const graphKeyRef = useRef('')

  // Push store data into React Flow when graph structure changes
  useEffect(() => {
    if (flowNodes.length === 0) return
    const key = flowNodes.map((n) => n.id).sort().join(',')
      + '|' + flowEdges.map((e) => e.id).sort().join(',')
    if (key === graphKeyRef.current) return
    graphKeyRef.current = key
    // Reset layout key so dagre runs after measurement
    layoutDoneForKey.current = ''

    // Apply selection state
    const withSelection = flowNodes.map((n) => ({
      ...n,
      selected: n.id === selectedNodeId,
    }))
    setNodes(withSelection)
    setEdges(flowEdges)
  }, [flowNodes, flowEdges, selectedNodeId, setNodes, setEdges])

  // Update selection without resetting layout
  useEffect(() => {
    setNodes((nds) =>
      nds.map((n) => ({
        ...n,
        selected: n.id === selectedNodeId,
      })),
    )
  }, [selectedNodeId, setNodes])

  // After React Flow measures all nodes, run dagre layout
  useEffect(() => {
    if (!nodesInitialized || nodes.length === 0) return
    if (layoutDoneForKey.current === graphKeyRef.current) return

    const rfNodes = getNodes()
    const allMeasured = rfNodes.every((n) => n.measured?.width)
    if (!allMeasured) return

    layoutDoneForKey.current = graphKeyRef.current
    const laid = applyDagreLayout(rfNodes, edges)
    setNodes(laid)
    requestAnimationFrame(() => fitView({ padding: 0.15, duration: 300 }))
  }, [nodesInitialized, nodes, edges, getNodes, setNodes, fitView])

  const handleNodeClick: NodeMouseHandler = useCallback(
    (_event, node) => {
      const graphNode = node.data as unknown as GraphNode
      const { graph: currentGraph, traverse, showMore, setSelectedNodeId } =
        useTopicGraphStore.getState()
      const curId = currentGraph?.currentNodeId

      if (graphNode.state === NodeState.PROPOSAL && graphNode.movement) {
        traverse(node.id, graphNode.movement as Movement)
      } else {
        setSelectedNodeId(node.id)
        if (graphNode.state === NodeState.VISITED || graphNode.state === NodeState.TOPIC_ROOT) {
          showMore(node.id)
        }
      }
    },
    [],
  )

  return (
    <ReactFlow
      nodes={nodes}
      edges={edges}
      onNodesChange={onNodesChange}
      onEdgesChange={onEdgesChange}
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
