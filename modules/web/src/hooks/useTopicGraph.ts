import { useState, useCallback, useEffect } from 'react'
import type { Node, Edge } from '@xyflow/react'
import { client } from '../api/client'
import {
  TOPIC_GRAPH,
  TRAVERSE,
  BACK_UP,
  SHOW_MORE,
  LEAVE_NOTE,
  ASK_ABOUT_NODE,
} from '../api/operations'
import type { TopicGraph, GraphNode, GraphEdge, Movement, AiResponse } from '../api/types'
import { NodeState, EdgeState } from '../api/types'

// ─── Movement colors ─────────────────────────────────────────────────

const MOVEMENT_COLORS: Record<string, string> = {
  SUPPORTS: '#4CAF50',
  DEEPENS: '#2196F3',
  RELATES_TO: '#FF9800',
  APPLIES: '#9C27B0',
  CONTEXTUALIZES: '#00BCD4',
}

// ─── Convert TopicGraph → React Flow elements ───────────────────────

function toReactFlowElements(graph: TopicGraph): { nodes: Node[]; edges: Edge[] } {
  const nodes: Node[] = graph.nodes.map((node) => {
    const nodeType = getNodeType(node)
    return {
      id: node.id,
      type: nodeType,
      position: { x: 0, y: 0 }, // will be laid out
      data: { ...node, topicRootId: graph.topicRoot.id },
    }
  })

  const edges: Edge[] = graph.edges.map((edge) => ({
    id: edge.id,
    source: edge.sourceId,
    target: edge.targetId,
    type: 'movement',
    data: edge,
    style: {
      stroke: MOVEMENT_COLORS[edge.movement] || '#999',
      strokeWidth: 2,
      strokeDasharray: edge.state === EdgeState.PROPOSAL ? '5 5' : undefined,
      opacity: edge.state === EdgeState.PROPOSAL ? 0.6 : 1,
    },
    animated: edge.state === EdgeState.PROPOSAL,
  }))

  return { nodes: applyLayout(nodes, edges, graph.currentNodeId), edges }
}

function getNodeType(node: GraphNode): string {
  switch (node.state) {
    case NodeState.TOPIC_ROOT:
      return 'topicRoot'
    case NodeState.CURRENT:
      return 'current'
    case NodeState.VISITED:
      return 'visited'
    case NodeState.PROPOSAL:
      return 'proposal'
    default:
      return 'proposal'
  }
}

// ─── Simple radial layout ────────────────────────────────────────────

function applyLayout(nodes: Node[], edges: Edge[], currentNodeId: string): Node[] {
  if (nodes.length === 0) return nodes

  // Build adjacency
  const children: Record<string, string[]> = {}
  for (const edge of edges) {
    if (!children[edge.source]) children[edge.source] = []
    children[edge.source].push(edge.target)
  }

  // BFS from topic root (first node in array, or find it)
  const rootNode = nodes.find((n) => n.type === 'topicRoot') || nodes[0]
  const visited = new Set<string>()
  const positions: Record<string, { x: number; y: number }> = {}

  // Place root at center
  positions[rootNode.id] = { x: 0, y: 0 }
  visited.add(rootNode.id)

  const queue: { id: string; depth: number; parentAngle: number }[] = []

  const rootChildren = children[rootNode.id] || []
  rootChildren.forEach((childId, i) => {
    const angle = (2 * Math.PI * i) / rootChildren.length - Math.PI / 2
    queue.push({ id: childId, depth: 1, parentAngle: angle })
  })

  const RING_SPACING = 200

  while (queue.length > 0) {
    const { id, depth, parentAngle } = queue.shift()!
    if (visited.has(id)) continue
    visited.add(id)

    const radius = depth * RING_SPACING
    positions[id] = {
      x: Math.cos(parentAngle) * radius,
      y: Math.sin(parentAngle) * radius,
    }

    const nodeChildren = (children[id] || []).filter((c) => !visited.has(c))
    const spreadAngle = Math.PI / 3
    nodeChildren.forEach((childId, i) => {
      const offset =
        nodeChildren.length === 1
          ? 0
          : ((i / (nodeChildren.length - 1)) - 0.5) * spreadAngle
      queue.push({
        id: childId,
        depth: depth + 1,
        parentAngle: parentAngle + offset,
      })
    })
  }

  // Assign positions — any unvisited nodes get placed around the edge
  let unvisitedAngle = 0
  return nodes.map((node) => ({
    ...node,
    position: positions[node.id] || {
      x: Math.cos(unvisitedAngle++) * 400,
      y: Math.sin(unvisitedAngle) * 400,
    },
  }))
}

// ─── Hook ────────────────────────────────────────────────────────────

export function useTopicGraph(topicRootId: string) {
  const [graph, setGraph] = useState<TopicGraph | null>(null)
  const [flowNodes, setFlowNodes] = useState<Node[]>([])
  const [flowEdges, setFlowEdges] = useState<Edge[]>([])
  const [selectedNodeId, setSelectedNodeId] = useState<string | null>(null)
  const [loading, setLoading] = useState(true)

  const updateFlow = useCallback((newGraph: TopicGraph) => {
    setGraph(newGraph)
    const { nodes, edges } = toReactFlowElements(newGraph)
    setFlowNodes(nodes)
    setFlowEdges(edges)
  }, [])

  // Initial load
  useEffect(() => {
    setLoading(true)
    client
      .request<{ topicGraph: TopicGraph }>(TOPIC_GRAPH, { topicRootId })
      .then((data) => updateFlow(data.topicGraph))
      .catch(console.error)
      .finally(() => setLoading(false))
  }, [topicRootId, updateFlow])

  const traverse = useCallback(
    async (fromNodeId: string, toNodeId: string, movement: Movement) => {
      setLoading(true)
      try {
        const data = await client.request<{ traverse: TopicGraph }>(TRAVERSE, {
          topicRootId,
          fromNodeId,
          toNodeId,
          movement,
        })
        updateFlow(data.traverse)
      } finally {
        setLoading(false)
      }
    },
    [topicRootId, updateFlow],
  )

  const backUp = useCallback(async () => {
    setLoading(true)
    try {
      const data = await client.request<{ backUp: TopicGraph }>(BACK_UP, { topicRootId })
      updateFlow(data.backUp)
    } finally {
      setLoading(false)
    }
  }, [topicRootId, updateFlow])

  const showMore = useCallback(
    async (nodeId: string, nodeTitle: string) => {
      setLoading(true)
      try {
        const data = await client.request<{ showMore: TopicGraph }>(SHOW_MORE, {
          topicRootId,
          nodeId,
          nodeTitle,
        })
        updateFlow(data.showMore)
      } finally {
        setLoading(false)
      }
    },
    [topicRootId, updateFlow],
  )

  const leaveNote = useCallback(
    async (nodeId: string, body: string) => {
      const data = await client.request<{ leaveNote: TopicGraph }>(LEAVE_NOTE, {
        nodeId,
        body,
      })
      updateFlow(data.leaveNote)
    },
    [updateFlow],
  )

  const askAboutNode = useCallback(
    async (nodeId: string, question: string): Promise<string> => {
      const data = await client.request<{ askAboutNode: AiResponse }>(ASK_ABOUT_NODE, {
        topicRootId,
        nodeId,
        question,
      })
      return data.askAboutNode.answer
    },
    [topicRootId],
  )

  const selectedNode = graph?.nodes.find((n) => n.id === selectedNodeId) || null

  return {
    graph,
    flowNodes,
    flowEdges,
    selectedNode,
    selectedNodeId,
    setSelectedNodeId,
    loading,
    traverse,
    backUp,
    showMore,
    leaveNote,
    askAboutNode,
  }
}
