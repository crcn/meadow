import { create } from 'zustand'
import type { Node, Edge } from '@xyflow/react'
import { Position } from '@xyflow/react'
import { client } from '../api/client'
import {
  TOPIC_GRAPH,
  TRAVERSE,
  JUMP_TO_NODE,
  BACK_UP,
  SHOW_MORE,
  REFRESH_RESOURCES,
  UPVOTE_RESOURCE,
} from '../api/operations'
import type { TopicGraph, GraphNode, Movement } from '../api/types'
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

function toReactFlowElements(graph: TopicGraph): { nodes: Node[]; edges: Edge[] } {
  const nodes: Node[] = graph.nodes.map((node) => {
    const nodeType = getNodeType(node)
    return {
      id: node.id,
      type: nodeType,
      position: { x: 0, y: 0 },
      data: { ...node, topicRootId: graph.topicRoot.id },
    }
  })

  const edges: Edge[] = graph.edges.map((edge) => ({
    id: edge.id,
    source: edge.sourceId,
    target: edge.targetId,
    sourcePosition: Position.Bottom,
    targetPosition: Position.Top,
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

// ─── Tree layout ────────────────────────────────────────────────────

const H_SPACING = 420
const V_SPACING = 240

function applyLayout(nodes: Node[], edges: Edge[], _currentNodeId: string): Node[] {
  if (nodes.length === 0) return nodes

  const children: Record<string, string[]> = {}
  for (const edge of edges) {
    if (!children[edge.source]) children[edge.source] = []
    children[edge.source].push(edge.target)
  }

  const rootNode = nodes.find((n) => n.type === 'topicRoot') || nodes[0]
  const visited = new Set<string>()
  const positions: Record<string, { x: number; y: number }> = {}

  function subtreeWidth(id: string): number {
    visited.add(id)
    const kids = (children[id] || []).filter((c) => !visited.has(c))
    if (kids.length === 0) return 1
    return kids.reduce((sum, kid) => sum + subtreeWidth(kid), 0)
  }
  const totalWidth = subtreeWidth(rootNode.id)
  visited.clear()

  function assignPositions(id: string, x: number, y: number, availableWidth: number) {
    if (visited.has(id)) return
    visited.add(id)
    positions[id] = { x, y }

    const kids = (children[id] || []).filter((c) => !visited.has(c))
    if (kids.length === 0) return

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

  const orphans = nodes.filter((n) => !positions[n.id])
  orphans.forEach((node, i) => {
    positions[node.id] = { x: (i - orphans.length / 2) * H_SPACING, y: -V_SPACING }
  })

  return nodes.map((node) => ({
    ...node,
    position: positions[node.id] || { x: 0, y: 0 },
  }))
}

// ─── Helpers ────────────────────────────────────────────────────────

function errorMessage(e: unknown): string {
  if (e instanceof Error) return e.message
  return String(e)
}

// ─── Zustand store ──────────────────────────────────────────────────

interface TopicGraphState {
  topicRootId: string | null
  graph: TopicGraph | null
  flowNodes: Node[]
  flowEdges: Edge[]
  selectedNodeId: string | null
  loading: boolean
  error: string | null
}

interface TopicGraphActions {
  init: (topicRootId: string) => Promise<void>
  traverse: (fromNodeId: string, toNodeId: string, movement: Movement) => Promise<void>
  jumpToNode: (nodeId: string) => Promise<void>
  backUp: () => Promise<void>
  showMore: (nodeId: string, nodeTitle: string) => Promise<void>
  loadMoreResources: (nodeId: string) => Promise<void>
  upvoteResource: (nodeId: string, resourceIndex: number) => Promise<void>
  setSelectedNodeId: (id: string | null) => void
  clearError: () => void
}

type TopicGraphStore = TopicGraphState & TopicGraphActions

function enrichFlowNodes(flowNodes: Node[], graph: TopicGraph | null, loading: boolean): Node[] {
  if (!graph) return flowNodes
  return flowNodes.map((node) =>
    node.id === graph.currentNodeId
      ? { ...node, data: { ...node.data, loading, showMoreFromStore: true } }
      : node
  )
}

export const useTopicGraphStore = create<TopicGraphStore>((set, get) => ({
  // State
  topicRootId: null,
  graph: null,
  flowNodes: [],
  flowEdges: [],
  selectedNodeId: null,
  loading: true,
  error: null,

  // Actions
  setSelectedNodeId: (id) => set({ selectedNodeId: id }),
  clearError: () => set({ error: null }),

  init: async (topicRootId) => {
    set({ topicRootId, loading: true, error: null })
    try {
      const data = await client.request<{ topicGraph: TopicGraph }>(TOPIC_GRAPH, { topicRootId })
      const { nodes, edges } = toReactFlowElements(data.topicGraph)
      set({
        graph: data.topicGraph,
        flowNodes: enrichFlowNodes(nodes, data.topicGraph, false),
        flowEdges: edges,
      })
    } catch (e: unknown) {
      console.error(e)
      set({ error: errorMessage(e) })
    } finally {
      set({ loading: false })
    }
  },

  traverse: async (fromNodeId, toNodeId, movement) => {
    const { topicRootId } = get()
    if (!topicRootId) return
    set({ loading: true, selectedNodeId: null, error: null })
    try {
      const data = await client.request<{ traverse: TopicGraph }>(TRAVERSE, {
        topicRootId,
        fromNodeId,
        toNodeId,
        movement,
      })
      const { nodes, edges } = toReactFlowElements(data.traverse)
      set({
        graph: data.traverse,
        flowNodes: enrichFlowNodes(nodes, data.traverse, false),
        flowEdges: edges,
      })
    } catch (e: unknown) {
      console.error(e)
      set({ error: errorMessage(e) })
    } finally {
      set({ loading: false })
    }
  },

  jumpToNode: async (nodeId) => {
    const { topicRootId } = get()
    if (!topicRootId) return
    set({ loading: true, error: null })
    try {
      const data = await client.request<{ jumpToNode: TopicGraph }>(JUMP_TO_NODE, {
        topicRootId,
        nodeId,
      })
      const { nodes, edges } = toReactFlowElements(data.jumpToNode)
      set({
        graph: data.jumpToNode,
        flowNodes: enrichFlowNodes(nodes, data.jumpToNode, false),
        flowEdges: edges,
      })
    } catch (e: unknown) {
      console.error(e)
      set({ error: errorMessage(e) })
    } finally {
      set({ loading: false })
    }
  },

  backUp: async () => {
    const { topicRootId } = get()
    if (!topicRootId) return
    set({ loading: true, error: null })
    try {
      const data = await client.request<{ backUp: TopicGraph }>(BACK_UP, { topicRootId })
      const { nodes, edges } = toReactFlowElements(data.backUp)
      set({
        graph: data.backUp,
        flowNodes: enrichFlowNodes(nodes, data.backUp, false),
        flowEdges: edges,
      })
    } catch (e: unknown) {
      console.error(e)
      set({ error: errorMessage(e) })
    } finally {
      set({ loading: false })
    }
  },

  showMore: async (nodeId, nodeTitle) => {
    const { topicRootId, graph, flowNodes } = get()
    if (!topicRootId) return
    set({ loading: true, error: null, flowNodes: enrichFlowNodes(flowNodes, graph, true) })
    try {
      const data = await client.request<{ showMore: TopicGraph }>(SHOW_MORE, {
        topicRootId,
        nodeId,
        nodeTitle,
      })
      const { nodes, edges } = toReactFlowElements(data.showMore)
      set({
        graph: data.showMore,
        flowNodes: enrichFlowNodes(nodes, data.showMore, false),
        flowEdges: edges,
      })
    } catch (e: unknown) {
      console.error(e)
      set({ error: errorMessage(e) })
    } finally {
      const { flowNodes: currentNodes, graph: currentGraph } = get()
      set({ loading: false, flowNodes: enrichFlowNodes(currentNodes, currentGraph, false) })
    }
  },

  loadMoreResources: async (nodeId) => {
    const { topicRootId } = get()
    if (!topicRootId) return
    set({ loading: true, error: null })
    try {
      const data = await client.request<{ refreshResources: TopicGraph }>(REFRESH_RESOURCES, {
        topicRootId,
        nodeId,
      })
      const { nodes, edges } = toReactFlowElements(data.refreshResources)
      set({
        graph: data.refreshResources,
        flowNodes: enrichFlowNodes(nodes, data.refreshResources, false),
        flowEdges: edges,
      })
    } catch (e: unknown) {
      console.error(e)
      set({ error: errorMessage(e) })
    } finally {
      set({ loading: false })
    }
  },

  upvoteResource: async (nodeId, resourceIndex) => {
    const { topicRootId } = get()
    if (!topicRootId) return
    try {
      const data = await client.request<{ upvoteResource: TopicGraph }>(UPVOTE_RESOURCE, {
        topicRootId,
        nodeId,
        resourceIndex,
      })
      const { nodes, edges } = toReactFlowElements(data.upvoteResource)
      set({
        graph: data.upvoteResource,
        flowNodes: enrichFlowNodes(nodes, data.upvoteResource, false),
        flowEdges: edges,
      })
    } catch (e: unknown) {
      console.error(e)
      set({ error: errorMessage(e) })
    }
  },
}))
