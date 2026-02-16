---
date: 2026-02-16
topic: canvas-architecture-mapping
---

# Canvas UI → Architecture Mapping

## The Core Shift

Card-stack UI needed: `currentNode + proposals[]`
Canvas UI needs: **the entire visible graph for this learner on this topic**

Every visited node, every traversed edge, every active proposal,
the current position — all in one response. The canvas renders the
full picture, not just the current step.

---

## Revised GraphQL Schema

```graphql
enum Movement {
  SUPPORTS
  DEEPENS
  RELATES_TO
  APPLIES
  CONTEXTUALIZES
}

enum NodeState {
  CURRENT           # where the learner is right now
  VISITED           # been here before
  PROPOSAL          # unvisited, suggested by AI or graph
  TOPIC_ROOT        # the central hub
}

enum EdgeState {
  TRAVERSED         # learner has walked this edge
  PROPOSAL          # suggested but not yet taken
}

type Resource {
  type: String!
  youtubeId: String
  url: String
  title: String!
  channel: String
  reason: String!
}

type GraphNode {
  id: ID!
  title: String!
  description: String!
  resources: [Resource!]!
  state: NodeState!
  movement: Movement          # how this node relates to its parent (null for root)
  isWildcard: Boolean!
}

type GraphEdge {
  id: ID!
  sourceId: ID!
  targetId: ID!
  movement: Movement!
  state: EdgeState!
  weight: Float!
}

type TopicGraph {
  topicRoot: TopicRoot!
  currentNodeId: ID!
  nodes: [GraphNode!]!
  edges: [GraphEdge!]!
}

type TopicRoot {
  id: ID!
  name: String!
  description: String!
}

type AuthResult {
  success: Boolean!
}

type VerifyResult {
  memberId: ID!
  token: String!
}

type SessionTopic {
  topicRoot: TopicRoot!
  currentNodeId: ID!
  lastActive: String!
}

# ─────────────────────────────────────────────

type Query {
  # Get learner's active topics for the home screen
  myTopics: [SessionTopic!]!

  # Get the full visible graph for a topic
  # This is THE main query — powers the entire canvas
  topicGraph(topicRootId: ID!): TopicGraph!
}

type Mutation {
  # Auth
  sendOtp(phoneNumber: String!): AuthResult!
  verifyOtp(phoneNumber: String!, code: String!): VerifyResult!

  # Enter a topic — creates/matches topic, generates starting points
  # Returns the initial graph (root + starting point nodes)
  enterTopic(interest: String!): TopicGraph!

  # Pick a starting point or traverse to a proposal
  # Returns the updated full graph (new node becomes current,
  # new proposals generated, edges updated)
  traverse(fromNodeId: ID!, toNodeId: ID!, movement: Movement!): TopicGraph!

  # Go back — downvotes abandoned edge, returns updated graph
  backUp: TopicGraph!

  # Show me more — generates new proposals from current node
  # Returns updated graph with new proposal nodes + edges added
  showMore(nodeId: ID!): TopicGraph!
}
```

### What Changed

| Before (card stack) | After (canvas) |
|---|---|
| `enterTopic` → `TopicEntry { startingPoints[] }` | `enterTopic` → `TopicGraph { nodes[], edges[] }` |
| `startFrom` → `TraversalResult { currentNode, proposals[] }` | **Removed.** `traverse` handles this (pick starting point = traverse from root) |
| `traverse` → `TraversalResult { currentNode, proposals[] }` | `traverse` → `TopicGraph` (full updated graph) |
| `backUp` → `TraversalResult` | `backUp` → `TopicGraph` |
| `showMore` → `[Proposal]` | `showMore` → `TopicGraph` (new nodes/edges added) |
| `session` → `[SessionTopic]` | `myTopics` → `[SessionTopic]` (just for home screen) |
| Separate `startFrom` mutation | Gone — traversing from TopicRoot to a starting point IS a traverse |
| `Node.proposals` lazy field | Gone — proposals are just nodes with `state: PROPOSAL` in the graph |

### Why TopicGraph For Everything

Every mutation returns the full `TopicGraph` because the canvas needs
to re-render the whole picture after any action. This seems heavy
but the graph is small — a typical session has 5-20 visited nodes
and 3-10 proposal nodes. That's a tiny payload.

The alternative (returning deltas) is more complex and error-prone.
If performance becomes an issue later, we can add delta responses.

---

## How Each Screen Maps

### Home Screen

```
Query: myTopics

Returns: [
  { topicRoot: { id, name }, currentNodeId: "...", lastActive: "..." },
  ...
]

No graph data needed — just topic names and last position.
Clicking "Continue" navigates to canvas and fires topicGraph query.
Typing new interest fires enterTopic mutation.
```

### Canvas — Entering a New Topic

```
Mutation: enterTopic(interest: "Bansuri")

Backend flow:
  1. Embed interest → match/create TopicRoot
  2. AI Investigation Agent → generate 3-5 starting points
  3. Persist starting point nodes + STARTS_WITH edges in Memgraph

Returns TopicGraph:
  nodes: [
    { id: "root", title: "Bansuri", state: TOPIC_ROOT },
    { id: "sp1", title: "Holding & Breath Control", state: PROPOSAL, movement: null },
    { id: "sp2", title: "Understanding Sa Re Ga", state: PROPOSAL, movement: null },
    { id: "sp3", title: "The Bansuri in Indian Music", state: PROPOSAL, movement: null }
  ]
  edges: [
    { sourceId: "root", targetId: "sp1", movement: SUPPORTS, state: PROPOSAL },
    { sourceId: "root", targetId: "sp2", movement: SUPPORTS, state: PROPOSAL },
    { sourceId: "root", targetId: "sp3", movement: CONTEXTUALIZES, state: PROPOSAL }
  ]
  currentNodeId: "root"

Canvas renders: TopicRoot in center, 3 starting points fanning out.
```

### Canvas — Picking a Starting Point

```
Mutation: traverse(fromNodeId: "root", toNodeId: "sp1", movement: SUPPORTS)

Backend flow:
  1. Update edge root→sp1: state becomes TRAVERSED, increment traversals
  2. Set learner position to sp1
  3. Record in traversal_history
  4. Generate proposals from sp1 (AI investigation if cold)
  5. Assemble full graph: root + sp1 (now visited) + sp2/sp3 (still proposals
     from root, now faded) + new proposals from sp1

Returns TopicGraph:
  nodes: [
    { id: "root", title: "Bansuri", state: TOPIC_ROOT },
    { id: "sp1", title: "Holding & Breath Control", state: CURRENT },
    { id: "sp2", title: "Understanding Sa Re Ga", state: PROPOSAL },      ← still visible
    { id: "sp3", title: "The Bansuri in Indian Music", state: PROPOSAL },  ← still visible
    { id: "p1", title: "Finger Placement", state: PROPOSAL, movement: SUPPORTS },
    { id: "p2", title: "Diaphragmatic Breathing", state: PROPOSAL, movement: DEEPENS },
    { id: "p3", title: "Bamboo Flute Across Cultures", state: PROPOSAL, movement: CONTEXTUALIZES }
  ]
  edges: [
    { sourceId: "root", targetId: "sp1", state: TRAVERSED },   ← now solid
    { sourceId: "root", targetId: "sp2", state: PROPOSAL },    ← still dashed
    { sourceId: "root", targetId: "sp3", state: PROPOSAL },    ← still dashed
    { sourceId: "sp1", targetId: "p1", state: PROPOSAL },
    { sourceId: "sp1", targetId: "p2", state: PROPOSAL },
    { sourceId: "sp1", targetId: "p3", state: PROPOSAL }
  ]
  currentNodeId: "sp1"

Canvas: root connected to sp1 (solid line), sp1 highlighted as current,
3 new proposals fanning out from sp1. sp2/sp3 still visible but faded.
```

### Canvas — Traversing Deeper

```
Mutation: traverse(fromNodeId: "sp1", toNodeId: "p1", movement: SUPPORTS)

Backend flow:
  1. Update edge sp1→p1: TRAVERSED, increment traversals
  2. Set position to p1
  3. Record in traversal_history
  4. Generate proposals from p1

Returns TopicGraph:
  nodes: [
    { id: "root", state: TOPIC_ROOT },
    { id: "sp1", title: "Holding & Breath Control", state: VISITED },     ← was CURRENT
    { id: "sp2", state: PROPOSAL },
    { id: "sp3", state: PROPOSAL },
    { id: "p1", title: "Finger Placement", state: CURRENT },              ← now CURRENT
    { id: "p2", title: "Diaphragmatic Breathing", state: PROPOSAL },      ← still there
    { id: "p3", title: "Bamboo Flute", state: PROPOSAL },                 ← still there
    { id: "q1", title: "Saptak", state: PROPOSAL },                       ← new from p1
    { id: "q2", title: "Meend", state: PROPOSAL },                        ← new from p1
    { id: "q3", title: "Raga Yaman", state: PROPOSAL }                    ← new from p1
  ]
  edges: [
    { sourceId: "root", targetId: "sp1", state: TRAVERSED },
    { sourceId: "root", targetId: "sp2", state: PROPOSAL },
    { sourceId: "root", targetId: "sp3", state: PROPOSAL },
    { sourceId: "sp1", targetId: "p1", state: TRAVERSED },                ← now solid
    { sourceId: "sp1", targetId: "p2", state: PROPOSAL },
    { sourceId: "sp1", targetId: "p3", state: PROPOSAL },
    { sourceId: "p1", targetId: "q1", state: PROPOSAL },                  ← new
    { sourceId: "p1", targetId: "q2", state: PROPOSAL },                  ← new
    { sourceId: "p1", targetId: "q3", state: PROPOSAL }                   ← new
  ]
  currentNodeId: "p1"

Canvas: path root → sp1 → p1 in solid lines, p1 highlighted,
new proposals branching from p1, old proposals from sp1 and root
still visible but faded.
```

### Canvas — Show Me More

```
Mutation: showMore(nodeId: "p1")

Backend flow:
  1. Existing proposals from p1: [q1, q2, q3]
  2. Run AI investigation with exclusion context
  3. Generate 3 more proposals, persist to Memgraph

Returns TopicGraph:
  Same as before PLUS 3 new proposal nodes + edges from p1.
  Canvas: p1 now has 6 outgoing edges instead of 3.
  Graph gets wider at this node.
```

### Canvas — Back Up

```
Mutation: backUp

Backend flow:
  1. Current is p1, previous is sp1
  2. Downvote edge sp1→p1 (increment backups)
  3. Set position back to sp1
  4. Generate proposals for sp1 (deprioritize p1)

Returns TopicGraph:
  p1 state becomes VISITED (not removed — it's still in the graph)
  sp1 state becomes CURRENT again
  Edge sp1→p1 stays but weight drops
  sp1 gets fresh proposals (p1 deprioritized)
  currentNodeId: "sp1"

Canvas: camera pans back to sp1, p1 recedes, new proposals appear.
The traversed edge sp1→p1 is still visible but now visually
"dimmer" (lower weight).
```

---

## Backend: How topicGraph Assembles

The `topicGraph` query and every mutation that returns `TopicGraph`
all go through the same assembly function:

```rust
// learning-core/src/domains/graph/assembler.rs

pub async fn assemble_topic_graph(
    graph: &Graph,           // Memgraph client
    db: &PgPool,             // Postgres
    member_id: Uuid,
    topic_root_id: Uuid,
) -> Result<TopicGraph> {

    // 1. Get all nodes this learner can see
    //    = visited nodes + current proposals + topic root
    //
    //    Visited nodes: from traversal_history in Postgres
    //    Current proposals: outgoing edges from current node in Memgraph
    //    Topic root: always included

    let position = get_learner_position(db, member_id, topic_root_id).await?;
    let visited_ids = get_visited_node_ids(db, member_id, topic_root_id).await?;

    // 2. Fetch node data from Memgraph
    //    One query: get all visited nodes + their outgoing proposal edges

    let (nodes, edges) = fetch_visible_subgraph(
        graph,
        topic_root_id,
        &visited_ids,
        position.current_node_id,
    ).await?;

    // 3. Tag each node with its state
    //    TOPIC_ROOT if it's the root
    //    CURRENT if it matches position.current_node_id
    //    VISITED if it's in visited_ids
    //    PROPOSAL otherwise

    // 4. Tag each edge with its state
    //    TRAVERSED if both source and target are visited/current
    //    PROPOSAL otherwise

    // 5. Return assembled TopicGraph
}
```

### The Memgraph Query

One query to get the full visible subgraph:

```cypher
// Get all visited nodes + their data
MATCH (n:Node)
WHERE n.id IN $visited_ids OR n.id = $current_id
WITH collect(n) AS visited_nodes

// Get proposals from current node
MATCH (current:Node {id: $current_id})-[r]->(proposal:Node)
WITH visited_nodes, collect({node: proposal, edge: r, type: type(r)}) AS proposals

// Get the topic root
MATCH (root:TopicRoot {id: $topic_root_id})

// Get all edges between visited nodes
MATCH (a:Node)-[r]->(b:Node)
WHERE a.id IN $visited_ids AND b.id IN $visited_ids
WITH root, visited_nodes, proposals,
     collect({source: a.id, target: b.id, type: type(r), weight: r.weight}) AS traversed_edges

// Also get unvisited proposals from visited nodes (not just current)
// These are the "faded" proposals visible on the canvas
MATCH (v:Node)-[r]->(p:Node)
WHERE v.id IN $visited_ids
  AND NOT p.id IN $visited_ids
  AND NOT p.id = $current_id
WITH root, visited_nodes, proposals, traversed_edges,
     collect({source: v.id, target: p.id, node: p, type: type(r), weight: r.weight}) AS old_proposals

RETURN root, visited_nodes, proposals, traversed_edges, old_proposals
```

---

## Revised Module Structure

```
modules/learning-core/src/
  domains/
    auth/
      mod.rs
      models.rs
      jwt.rs
      identifier.rs
      otp.rs

    graph/
      mod.rs
      models.rs                # Node, TopicRoot, Movement, GraphNode, GraphEdge
      client.rs                # Memgraph connection wrapper
      queries.rs               # Cypher query builders
      assembler.rs             # ★ NEW: assemble TopicGraph from Memgraph + Postgres
      entry.rs                 # Topic matching (embed + LLM)
      traversal.rs             # Traverse, back up, update weights
      weights.rs               # Weight calculation
      dedup.rs                 # Node deduplication via embeddings

    ai/
      mod.rs
      agent.rs                 # Multi-turn investigation agent
      tools/
        mod.rs
        tavily.rs
        youtube_search.rs
        youtube_details.rs
        existing_nodes.rs
      proposer.rs              # Orchestrate: check graph → run agent if needed
      starting_points.rs       # Generate starting points for new topics
      prompts.rs               # System prompts

  error.rs
```

```
modules/learning-server/src/
  main.rs
  state.rs                     # AppState
  graphql/
    mod.rs
    schema.rs                  # Build schema
    query.rs                   # myTopics, topicGraph
    mutation.rs                # sendOtp, verifyOtp, enterTopic, traverse, backUp, showMore
    types.rs                   # GraphQL types (TopicGraph, GraphNode, GraphEdge, etc.)
    guard.rs                   # JWT auth guard
```

```
web/src/
  pages/
    Login.tsx
    Home.tsx
    Canvas.tsx                 # THE main experience

  components/
    canvas/
      GraphCanvas.tsx          # React Flow wrapper + layout
      CurrentNode.tsx          # Full detail node (description + video)
      VisitedNode.tsx          # Compact visited node
      ProposalNode.tsx         # Unvisited proposal with movement badge
      TopicRootNode.tsx        # Central hub
      MovementEdge.tsx         # Color-coded edge
      DetailPanel.tsx          # Bottom sheet for node detail
      ShowMoreButton.tsx       # Floating on canvas
    auth/
      LoginForm.tsx
      OtpInput.tsx
    home/
      TopicInput.tsx
      ContinueCard.tsx

  api/
    client.ts                  # GraphQL client
    operations.ts              # Typed queries + mutations
    types.ts                   # Generated or hand-written TypeScript types

  hooks/
    useTopicGraph.ts           # Manages graph state, converts to React Flow format
    useAuth.ts                 # Auth state
```

---

## Data Flow Summary

```
┌─────────────────────────────────────────────────────────────────┐
│                        REACT CANVAS                              │
│                                                                  │
│  useTopicGraph hook                                              │
│    │                                                             │
│    ├─ enterTopic("Bansuri")                                      │
│    │     └─► TopicGraph → toReactFlowElements() → render         │
│    │                                                             │
│    ├─ traverse(from, to, movement)                               │
│    │     └─► TopicGraph → toReactFlowElements() → render         │
│    │         (animate: pan to new current, new proposals fade in) │
│    │                                                             │
│    ├─ backUp()                                                   │
│    │     └─► TopicGraph → toReactFlowElements() → render         │
│    │         (animate: pan back to previous node)                │
│    │                                                             │
│    ├─ showMore(nodeId)                                           │
│    │     └─► TopicGraph → toReactFlowElements() → render         │
│    │         (animate: new proposals appear around node)          │
│    │                                                             │
│    └─ topicGraph(topicRootId)   // on resume                     │
│          └─► TopicGraph → toReactFlowElements() → render         │
│                                                                  │
└──────────────────────────┬───────────────────────────────────────┘
                           │
                      GraphQL POST /graphql
                           │
┌──────────────────────────▼───────────────────────────────────────┐
│                     AXUM + ASYNC-GRAPHQL                          │
│                                                                  │
│  Every mutation/query that returns TopicGraph calls:              │
│                                                                  │
│    graph::assembler::assemble_topic_graph(                        │
│      memgraph, postgres, member_id, topic_root_id                │
│    )                                                             │
│                                                                  │
│  Which:                                                          │
│    1. Gets learner position from Postgres                        │
│    2. Gets visited node IDs from Postgres                        │
│    3. Fetches visible subgraph from Memgraph (one query)         │
│    4. Tags nodes: CURRENT / VISITED / PROPOSAL / TOPIC_ROOT      │
│    5. Tags edges: TRAVERSED / PROPOSAL                           │
│    6. Returns TopicGraph                                         │
│                                                                  │
│  Before assembling, each mutation does its specific work:         │
│                                                                  │
│  enterTopic:                                                     │
│    → embed interest → match/create TopicRoot                     │
│    → AI investigation → generate starting points                 │
│    → persist nodes + edges in Memgraph                           │
│    → assemble_topic_graph()                                      │
│                                                                  │
│  traverse:                                                       │
│    → update edge weight in Memgraph (traversals++)               │
│    → record in Postgres traversal_history                        │
│    → update learner_position                                     │
│    → generate proposals for new current node                     │
│      (graph domain checks existing edges first,                  │
│       AI domain fills gaps if sparse)                            │
│    → assemble_topic_graph()                                      │
│                                                                  │
│  backUp:                                                         │
│    → look up previous from Postgres                              │
│    → downvote edge in Memgraph (backups++)                       │
│    → record backtrack in Postgres                                │
│    → update learner_position                                     │
│    → generate proposals for restored node                        │
│    → assemble_topic_graph()                                      │
│                                                                  │
│  showMore:                                                       │
│    → get existing proposals from Memgraph                        │
│    → AI investigation with exclusion context                     │
│    → persist new nodes + edges                                   │
│    → assemble_topic_graph()                                      │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

---

## What the React Flow Hook Does

```typescript
// hooks/useTopicGraph.ts

function useTopicGraph(topicRootId: string) {
  const [graph, setGraph] = useState<TopicGraph | null>(null);

  // Convert TopicGraph → React Flow elements
  const { nodes, edges } = useMemo(() => {
    if (!graph) return { nodes: [], edges: [] };
    return toReactFlowElements(graph);
  }, [graph]);

  // Layout: position nodes using force-directed or radial
  const positionedNodes = useAutoLayout(nodes, edges, graph.currentNodeId);

  // Mutations
  const enterTopic = async (interest: string) => {
    const result = await gqlClient.enterTopic({ interest });
    setGraph(result);
  };

  const traverse = async (fromId: string, toId: string, movement: Movement) => {
    const result = await gqlClient.traverse({ fromNodeId: fromId, toNodeId: toId, movement });
    setGraph(result);
    // React Flow animates the transition (pan to new current)
  };

  const backUp = async () => {
    const result = await gqlClient.backUp();
    setGraph(result);
  };

  const showMore = async (nodeId: string) => {
    const result = await gqlClient.showMore({ nodeId });
    setGraph(result);
  };

  return { nodes: positionedNodes, edges, currentNodeId: graph?.currentNodeId,
           enterTopic, traverse, backUp, showMore };
}
```

### toReactFlowElements

```typescript
function toReactFlowElements(graph: TopicGraph) {
  const nodes: RFNode[] = graph.nodes.map(n => ({
    id: n.id,
    type: nodeStateToType(n.state),  // 'current' | 'visited' | 'proposal' | 'topicRoot'
    data: {
      title: n.title,
      description: n.description,
      resources: n.resources,
      movement: n.movement,
      isWildcard: n.isWildcard,
      state: n.state,
    },
    position: { x: 0, y: 0 },  // layout engine positions these
  }));

  const edges: RFEdge[] = graph.edges.map(e => ({
    id: e.id,
    source: e.sourceId,
    target: e.targetId,
    type: 'movement',
    data: {
      movement: e.movement,
      state: e.state,
      weight: e.weight,
    },
    animated: e.state === 'PROPOSAL',   // dashed animation for proposals
    style: {
      stroke: movementColor(e.movement),
      strokeWidth: e.state === 'TRAVERSED' ? 3 : 1,
      strokeDasharray: e.state === 'PROPOSAL' ? '5 5' : undefined,
      opacity: e.state === 'PROPOSAL' ? 0.5 : 1,
    },
  }));

  return { nodes, edges };
}
```
