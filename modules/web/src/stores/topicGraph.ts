import { create } from 'zustand'
import type { Node, Edge } from '@xyflow/react'
import { Position } from '@xyflow/react'
import { client } from '../api/client'
import {
  TOPIC_GRAPH,
  EXPANSION_SUGGESTIONS,
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
  DEEPER: '#c49060',
  BROADER: '#dbb894',
  FOUNDATION: '#d4a574',
  PRACTICE: '#b8860b',
  INSPIRE: '#9b59b6',
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
      // Positions start at origin — dagre computes real positions
      // after React Flow measures actual node dimensions
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

  return { nodes, edges }
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
  traverse: (toNodeId: string, movement: Movement) => Promise<void>
  jumpToNode: (nodeId: string) => Promise<void>
  backUp: () => Promise<void>
  showMore: (nodeId: string, prompt?: string) => Promise<void>
  fetchExpansionSuggestions: (nodeId: string) => Promise<string[]>
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

  traverse: async (toNodeId, movement) => {
    const { topicRootId } = get()
    if (!topicRootId) return
    set({ loading: true, selectedNodeId: null, error: null })
    try {
      const data = await client.request<{ traverse: TopicGraph }>(TRAVERSE, {
        topicRootId,
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

  fetchExpansionSuggestions: async (nodeId) => {
    const { topicRootId } = get()
    if (!topicRootId) return []
    try {
      const data = await client.request<{ topicGraph: { expansionSuggestions: string[] } }>(
        EXPANSION_SUGGESTIONS,
        { topicRootId, nodeId },
      )
      return data.topicGraph.expansionSuggestions
    } catch (e: unknown) {
      console.error(e)
      set({ error: errorMessage(e) })
      return []
    }
  },

  showMore: async (nodeId, prompt) => {
    const { topicRootId, graph, flowNodes } = get()
    if (!topicRootId) return
    set({ loading: true, error: null, flowNodes: enrichFlowNodes(flowNodes, graph, true) })
    try {
      const data = await client.request<{ showMore: TopicGraph }>(SHOW_MORE, {
        topicRootId,
        nodeId,
        prompt,
      })
      const { nodes, edges } = toReactFlowElements(data.showMore)
      set({
        graph: data.showMore,
        flowNodes: enrichFlowNodes(nodes, data.showMore, false),
        flowEdges: edges,
      })
    } catch (e: unknown) {
      console.error(e)
      const { flowNodes: currentNodes, graph: currentGraph } = get()
      set({ error: errorMessage(e), loading: false, flowNodes: enrichFlowNodes(currentNodes, currentGraph, false) })
      return
    }
    set({ loading: false })
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
