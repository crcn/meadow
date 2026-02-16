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
  SUPPORTS: '#d4a574',
  DEEPENS: '#c49060',
  RELATES_TO: '#dbb894',
  APPLIES: '#c9986c',
  CONTEXTUALIZES: '#d4b898',
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

// ─── Tree layout ────────────────────────────────────────────────────

const H_SPACING = 360 // horizontal gap between siblings
const V_SPACING = 200 // vertical gap between rows

function applyLayout(nodes: Node[], edges: Edge[], _currentNodeId: string): Node[] {
  if (nodes.length === 0) return nodes

  // Build parent→children adjacency
  const children: Record<string, string[]> = {}
  for (const edge of edges) {
    if (!children[edge.source]) children[edge.source] = []
    children[edge.source].push(edge.target)
  }

  const rootNode = nodes.find((n) => n.type === 'topicRoot') || nodes[0]
  const visited = new Set<string>()
  const positions: Record<string, { x: number; y: number }> = {}

  // First pass: compute subtree widths (leaf count)
  function subtreeWidth(id: string): number {
    visited.add(id)
    const kids = (children[id] || []).filter((c) => !visited.has(c))
    if (kids.length === 0) return 1
    return kids.reduce((sum, kid) => sum + subtreeWidth(kid), 0)
  }
  const totalWidth = subtreeWidth(rootNode.id)
  visited.clear()

  // Second pass: assign positions top-down
  function assignPositions(id: string, x: number, y: number, availableWidth: number) {
    if (visited.has(id)) return
    visited.add(id)

    positions[id] = { x, y }

    const kids = (children[id] || []).filter((c) => !visited.has(c))
    if (kids.length === 0) return

    // Compute widths for each subtree to allocate space proportionally
    const kidVisited = new Set<string>(visited)
    const widths = kids.map((kid) => {
      const tempVisited = new Set<string>(kidVisited)
      function tw(nid: string): number {
        tempVisited.add(nid)
        const ch = (children[nid] || []).filter((c) => !tempVisited.has(c))
        if (ch.length === 0) return 1
        return ch.reduce((s, c) => s + tw(c), 0)
      }
      return tw(kid)
    })

    const totalKidWidth = widths.reduce((s, w) => s + w, 0)
    const slotWidth = Math.max(availableWidth, totalKidWidth * H_SPACING)

    let offsetX = x - slotWidth / 2
    kids.forEach((kid, i) => {
      const kidSlot = (widths[i] / totalKidWidth) * slotWidth
      const kidX = offsetX + kidSlot / 2
      assignPositions(kid, kidX, y + V_SPACING, kidSlot)
      offsetX += kidSlot
    })
  }

  assignPositions(rootNode.id, 0, 0, totalWidth * H_SPACING)

  // Place any orphan nodes that weren't reached
  const orphans = nodes.filter((n) => !positions[n.id])
  orphans.forEach((node, i) => {
    positions[node.id] = { x: (i - orphans.length / 2) * H_SPACING, y: -V_SPACING }
  })

  return nodes.map((node) => ({
    ...node,
    position: positions[node.id] || { x: 0, y: 0 },
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
