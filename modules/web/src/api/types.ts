// ─── Enums ───────────────────────────────────────────────────────────

export enum Movement {
  DEEPER = 'DEEPER',
  BROADER = 'BROADER',
  FOUNDATION = 'FOUNDATION',
  PRACTICE = 'PRACTICE',
  INSPIRE = 'INSPIRE',
}

export enum NodeState {
  CURRENT = 'CURRENT',
  VISITED = 'VISITED',
  PROPOSAL = 'PROPOSAL',
  TOPIC_ROOT = 'TOPIC_ROOT',
}

export enum EdgeState {
  TRAVERSED = 'TRAVERSED',
  PROPOSAL = 'PROPOSAL',
}

// ─── Types ───────────────────────────────────────────────────────────

export interface TopicRoot {
  id: string
  name: string
  description: string
}

export interface Resource {
  type: string
  youtubeId: string | null
  url: string | null
  title: string
  channel: string | null
  reason: string
  votes: number
}

export interface Note {
  id: string
  body: string
  createdAt: string
}

export interface GraphNode {
  id: string
  title: string
  description: string
  resources: Resource[]
  notes: Note[]
  state: NodeState
  movement: Movement | null
  isWildcard: boolean
  visitCount: number
  depth: number
}

export interface GraphEdge {
  id: string
  sourceId: string
  targetId: string
  movement: Movement
  state: EdgeState
  weight: number
}

export interface TopicGraph {
  topicRoot: TopicRoot
  currentNodeId: string
  nodes: GraphNode[]
  edges: GraphEdge[]
}

export interface SessionTopic {
  topicRoot: TopicRoot
  currentNodeId: string
  lastActive: string
}

export interface AuthResult {
  success: boolean
}

export interface VerifyResult {
  memberId: string
  token: string
}

export interface AiResponse {
  answer: string
}
