---
date: 2026-02-16
topic: data-flow-ai-investigation
---

# Data Flow: Frontend → Graph → AI Investigation

## The Full Picture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           REACT SPA                                     │
│                                                                         │
│  Login → Home → StartingPoints → Explore (loop)                        │
│                                   ▲                                     │
│                                   │ proposals[]                         │
│                                   │                                     │
└───────────────────────────────────┼─────────────────────────────────────┘
                                    │ GraphQL
┌───────────────────────────────────┼─────────────────────────────────────┐
│                        AXUM + ASYNC-GRAPHQL                             │
│                                   │                                     │
│  ┌────────────────────────────────▼──────────────────────────────────┐  │
│  │                     LEARNING-CORE                                  │  │
│  │                                                                    │  │
│  │  ┌──────────┐    ┌──────────────┐    ┌─────────────────────────┐  │  │
│  │  │  Auth     │    │  Graph       │    │  AI Investigation      │  │  │
│  │  │  Domain   │    │  Domain      │    │  Domain                │  │  │
│  │  │          │    │              │    │                         │  │  │
│  │  │ JWT      │    │ Memgraph    │◄───│  Investigative Agent   │  │  │
│  │  │ OTP      │    │ Traversal   │    │  ┌───────────────────┐ │  │  │
│  │  │ Identity │    │ Weights     │    │  │ Tool: Tavily      │ │  │  │
│  │  │          │    │ Dedup       │    │  │ Tool: YouTube API │ │  │  │
│  │  │          │    │ Entry       │    │  │ Tool: Reddit?     │ │  │  │
│  │  └──────────┘    └──────────────┘    │  └───────────────────┘ │  │  │
│  │                                      └─────────────────────────┘  │  │
│  └───────────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Flow 1: "I want to learn Bansuri" → Starting Points

```
User types "Bansuri" → enterTopic mutation
│
▼
1. TOPIC MATCHING
   ├─ Embed "Bansuri" → vector
   ├─ Search Memgraph TopicRoot embeddings
   ├─ Match found? → Use existing TopicRoot
   └─ No match? → Create new TopicRoot
│
▼
2. AI INVESTIGATION AGENT (for starting points)
   │
   │  The AI doesn't just make up starting points from its training data.
   │  It runs an investigative loop with tool calls:
   │
   │  ┌─────────────────────────────────────────────────────┐
   │  │  System: "You are generating starting points for    │
   │  │  someone who wants to learn Bansuri. Use your tools │
   │  │  to research what the best entry points are."       │
   │  │                                                     │
   │  │  TURN 1: AI decides to search                       │
   │  │  → tool_call: tavily_search("bansuri beginner       │
   │  │    learning path fundamentals")                     │
   │  │  → result: articles about bansuri basics, common    │
   │  │    first steps, technique guides                    │
   │  │                                                     │
   │  │  TURN 2: AI decides to find videos                  │
   │  │  → tool_call: youtube_search("bansuri tutorial      │
   │  │    beginner")                                       │
   │  │  → result: top 10 videos with titles + descriptions │
   │  │  → tool_call: youtube_video_details(video_ids)      │
   │  │  → result: full descriptions, view counts, likes    │
   │  │  → AI reads descriptions, picks best quality ones   │
   │  │                                                     │
   │  │  TURN 3: AI synthesizes findings                    │
   │  │  → Returns 3-5 starting points with:                │
   │  │    - title                                          │
   │  │    - description (informed by real resources)        │
   │  │    - resource_urls (videos/articles it found)        │
   │  └─────────────────────────────────────────────────────┘
   │
   │  Example output:
   │  [
   │    {
   │      title: "Holding & Breath Control",
   │      description: "How to hold the Bansuri and produce
   │        your first clear note through proper breath support",
   │      resources: [
   │        { type: "youtube", id: "abc123",
   │          title: "Bansuri Basics - First Sounds",
   │          reason: "Clear demonstration of embouchure..." }
   │      ]
   │    },
   │    {
   │      title: "Understanding Sa Re Ga",
   │      description: "The foundational notes of Indian
   │        classical music and how to find them on Bansuri",
   │      resources: [...]
   │    },
   │    {
   │      title: "The Bansuri in Indian Classical Music",
   │      description: "Context and history — why this
   │        instrument matters and what makes it unique",
   │      resources: [...]
   │    }
   │  ]
│
▼
3. PERSIST TO MEMGRAPH
   ├─ For each starting point:
   │   ├─ Embed title+description → check dedup against existing nodes
   │   ├─ Create Node (or reuse existing)
   │   ├─ Store resource URLs as node properties
   │   └─ Create STARTS_WITH edge from TopicRoot
   └─ Return TopicEntry { topicRoot, startingPoints[] }
│
▼
4. REACT: StartingPoints.tsx
   ├─ Display 3-5 cards, each with title + description
   ├─ Optionally show resource previews (video thumbnails)
   └─ User picks one → startFrom mutation
```

---

## Flow 2: User picks "Holding & Breath Control" → Start Swimming

```
User clicks starting point → startFrom mutation
│
▼
1. SET POSITION
   ├─ Record in Postgres: learner_position (member_id, topic_root_id, node_id)
   ├─ Record in Postgres: traversal_history (first entry)
   └─ Increment visit_count on node in Memgraph
│
▼
2. GENERATE PROPOSALS (the core loop — this happens every time)
   │
   │  First: check what already exists in the graph
   │  ┌──────────────────────────────────────────────────┐
   │  │  MATCH (n:Node {id: $current})-[r]->(neighbor)   │
   │  │  RETURN neighbor, type(r) as movement, r.weight   │
   │  │  ORDER BY r.weight DESC                           │
   │  └──────────────────────────────────────────────────┘
   │
   │  CASE A: Rich graph (3+ existing edges with traversal data)
   │  ├─ Pick top 2 by weight (proven paths)
   │  ├─ Pick 1 wildcard (low-weight existing OR new AI-generated)
   │  └─ Return immediately — no AI call needed for proven paths
   │
   │  CASE B: Sparse graph (< 3 existing edges)
   │  ├─ Use whatever existing edges we have
   │  └─ Fill remaining slots with AI Investigation Agent
   │
   │  CASE C: Empty (no existing edges — cold start)
   │  └─ All 3 slots from AI Investigation Agent
   │
   │  ─── AI INVESTIGATION AGENT (for proposals) ───
   │
   │  Context provided to the agent:
   │  - Current node: { title, description, resources }
   │  - Existing neighbors: [{ title, movement }] (to avoid duplicates)
   │  - Learner's path so far: [node titles in order]
   │  - Topic: "Bansuri"
   │
   │  ┌─────────────────────────────────────────────────────┐
   │  │  System: "The learner is at 'Holding & Breath       │
   │  │  Control' in their Bansuri journey. They've visited: │
   │  │  [just started]. Propose next steps. Use your tools  │
   │  │  to find real resources. Each proposal needs a       │
   │  │  movement type: Foundational, Intensifies, Lateral,  │
   │  │  Creative, or Contextual."                           │
   │  │                                                      │
   │  │  TURN 1: AI searches for what comes next             │
   │  │  → tool_call: tavily_search("bansuri after learning  │
   │  │    breath control what to learn next")                │
   │  │  → result: articles suggesting finger placement,     │
   │  │    basic scales, first ragas                          │
   │  │                                                      │
   │  │  TURN 2: AI finds supporting videos                  │
   │  │  → tool_call: youtube_search("bansuri finger         │
   │  │    placement technique")                              │
   │  │  → tool_call: youtube_search("bansuri first raga     │
   │  │    beginner")                                         │
   │  │  → Reads descriptions, picks best ones               │
   │  │                                                      │
   │  │  TURN 3: AI returns proposals                        │
   │  │  → [                                                  │
   │  │      { title: "Finger Placement & First Notes",      │
   │  │        description: "...",                            │
   │  │        movement: "Foundational",                     │
   │  │        resources: [{ youtube video about fingering }]│
   │  │      },                                              │
   │  │      { title: "Diaphragmatic Breathing for Bansuri", │
   │  │        description: "...",                            │
   │  │        movement: "Intensifies",                      │
   │  │        resources: [{ article about breath support }] │
   │  │      },                                              │
   │  │      { title: "The Bamboo Flute Across Cultures",    │
   │  │        description: "...",                            │
   │  │        movement: "Contextual",                       │
   │  │        resources: [{ youtube documentary }]          │
   │  │      }                                               │
   │  │    ]                                                  │
   │  └─────────────────────────────────────────────────────┘
   │
   │  For each AI-generated proposal:
   │  ├─ Embed title+description
   │  ├─ Dedup check against existing nodes in topic graph
   │  ├─ Create or reuse Node in Memgraph
   │  ├─ Create edge: current_node -[MOVEMENT_TYPE]-> proposal_node
   │  └─ Edge starts with weight 0, traversals 0, backups 0
│
▼
3. RETURN TO FRONTEND
   TraversalResult {
     currentNode: { id, title, description, resources },
     proposals: [
       { node: {...}, movement: SUPPORTS, isWildcard: false },
       { node: {...}, movement: DEEPENS, isWildcard: false },
       { node: {...}, movement: CONTEXTUALIZES, isWildcard: true }
     ]
   }
│
▼
4. REACT: Explore.tsx
   ┌─────────────────────────────────────────────┐
   │                                             │
   │  ◄ Back                                     │
   │                                             │
   │  ┌───────────────────────────────────────┐  │
   │  │  Holding & Breath Control              │  │
   │  │                                       │  │
   │  │  How to hold the Bansuri and produce   │  │
   │  │  your first clear note through proper  │  │
   │  │  breath support.                       │  │
   │  │                                       │  │
   │  │  🎥 "Bansuri Basics - First Sounds"    │  │
   │  │     by [channel] · 45K views           │  │
   │  └───────────────────────────────────────┘  │
   │                                             │
   │  Where to next?                             │
   │                                             │
   │  ┌─────────────────────────────┐            │
   │  │ ◆ Foundational              │            │
   │  │ Finger Placement & First    │            │
   │  │ Notes                       │            │
   │  │ Learn proper finger...      │            │
   │  └─────────────────────────────┘            │
   │                                             │
   │  ┌─────────────────────────────┐            │
   │  │ ▲ Intensifies               │            │
   │  │ Diaphragmatic Breathing     │            │
   │  │ for Bansuri                 │            │
   │  │ Go deeper into breath...    │            │
   │  └─────────────────────────────┘            │
   │                                             │
   │  ┌─────────────────────────────┐            │
   │  │ ○ Contextual    ✦ wildcard  │            │
   │  │ The Bamboo Flute Across     │            │
   │  │ Cultures                    │            │
   │  │ Explore the bamboo flute... │            │
   │  └─────────────────────────────┘            │
   │                                             │
   └─────────────────────────────────────────────┘
```

---

## Flow 3: User picks "Finger Placement" → Continue Swimming

```
User clicks "Finger Placement & First Notes" → traverse mutation
│
▼
1. UPDATE GRAPH WEIGHTS
   ├─ MERGE edge current_node -[SUPPORTS]-> finger_placement_node
   ├─ Increment traversals count on that edge
   ├─ Recalculate weight: max(0, (traversals - 2*backups)) / max(1, traversals + backups)
   └─ The fact that this learner chose Foundational over Intensifies
      is itself a signal about what path is valuable from "Breath Control"
│
▼
2. UPDATE POSTGRES
   ├─ Insert into traversal_history (member_id, topic_root_id, node_id,
   │   previous_node_id, movement_type: "SUPPORTS", is_backtrack: false)
   └─ Update learner_position to new node
│
▼
3. GENERATE PROPOSALS (same process as Flow 2, step 2)
   ├─ Check existing outgoing edges from "Finger Placement" node
   ├─ Apply 2+1 rule (or AI-generate if sparse)
   ├─ AI agent now has more context:
   │   "Learner is at 'Finger Placement & First Notes'.
   │    They came from 'Holding & Breath Control'.
   │    Path so far: [Breath Control → Finger Placement]"
   └─ This context helps the agent make better proposals
│
▼
4. RETURN TraversalResult → Explore.tsx re-renders with new node + proposals
│
▼
5. LOOP CONTINUES — user keeps picking, graph keeps growing
```

---

## Flow 4: User hits Back

```
User clicks ◄ Back → backUp mutation
│
▼
1. LOOK UP PREVIOUS NODE
   └─ Query traversal_history: most recent non-backtrack entry for this member+topic
│
▼
2. DOWNVOTE ABANDONED EDGE
   ├─ Find the edge: previous_node → current_node
   ├─ Increment backups count
   ├─ Recalculate weight (backups penalized 2x)
   └─ Signal: this path was not valuable from the previous node
│
▼
3. UPDATE POSTGRES
   ├─ Insert into traversal_history with is_backtrack: true
   └─ Update learner_position back to previous node
│
▼
4. GENERATE PROPOSALS for previous node
   ├─ Deprioritize the node they just backed up from
   └─ Otherwise same process
│
▼
5. RETURN TraversalResult → Explore.tsx shows previous node with new proposals
```

---

## The AI Investigation Agent — Detailed Design

This is NOT a simple prompt-in/text-out LLM call. It's a **multi-turn agent with tools**.

### Agent Architecture

```
┌──────────────────────────────────────────────────────────────┐
│                  AI INVESTIGATION AGENT                       │
│                                                              │
│  Uses: ai-client module (Agent trait + Tool trait)            │
│  Provider: Claude / OpenAI (configurable)                    │
│  Mode: multi_turn (agent loops until it has enough info)     │
│                                                              │
│  Tools available:                                            │
│  ┌────────────────────────────────────────────────────────┐  │
│  │                                                        │  │
│  │  tavily_search(query: String) -> SearchResults         │  │
│  │    Web search for articles, guides, learning resources │  │
│  │                                                        │  │
│  │  youtube_search(query: String) -> VideoList            │  │
│  │    Search YouTube for relevant videos                  │  │
│  │                                                        │  │
│  │  youtube_video_details(video_ids: Vec<String>)         │  │
│  │    -> Vec<VideoDetail>                                 │  │
│  │    Get full descriptions, stats, channel info          │  │
│  │    Agent reads descriptions to judge quality/relevance │  │
│  │                                                        │  │
│  │  get_existing_nodes(topic_root_id: String)             │  │
│  │    -> Vec<Node>                                        │  │
│  │    Check what already exists in the graph to avoid     │  │
│  │    duplicates                                          │  │
│  │                                                        │  │
│  └────────────────────────────────────────────────────────┘  │
│                                                              │
│  The agent decides:                                          │
│  - WHAT to search for (based on current node + path)         │
│  - HOW MANY searches to do (1-3 turns typically)             │
│  - WHICH videos/articles are actually good (reads descs)     │
│  - WHAT movement types make sense for the proposals          │
│  - WHEN it has enough information to return proposals        │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

### How It Maps to ai-client

The existing `ai-client` module already has the `Agent` trait with `.tool()` and `.prompt()` + multi-turn support. The investigation agent is built using this:

```rust
// Pseudocode — how it fits into the existing ai-client pattern

let agent = ai_client
    .tool(TavilySearchTool::new(tavily_api_key))
    .tool(YoutubeSearchTool::new(youtube_api_key))
    .tool(YoutubeDetailsTool::new(youtube_api_key))
    .tool(GetExistingNodesTool::new(graph_client.clone()));

let result = agent
    .prompt(format!(
        "The learner is at '{}' in their {} journey. \
         Path so far: [{}]. \
         Propose {} next steps. \
         Use your tools to find real resources. \
         Each proposal needs a movement type.",
        current_node.title,
        topic_name,
        path_titles.join(" → "),
        num_proposals
    ))
    .preamble(INVESTIGATION_SYSTEM_PROMPT)
    .multi_turn(5) // max 5 tool-call rounds
    .send()
    .await?;

// Parse structured output into Vec<ProposalWithResources>
```

### System Prompt (Investigation Agent)

```
You are a learning path investigator. Your job is to find the best
next steps for someone learning a topic.

You have tools to search the web and YouTube. USE THEM. Do not rely
solely on your training data — search for real, current resources.

When evaluating YouTube videos:
- Read the full description to judge quality
- Prefer videos with clear instructional content over vlogs
- Prefer videos from established educators/channels
- Note the video length — 5-20 min is ideal for learning nodes

For each proposal, provide:
- title: concise name for this learning step
- description: 1-2 sentences about what the learner will gain
- movement: one of Foundational, Intensifies, Lateral, Creative, Contextual
- resources: the best video/article you found for this topic

Try to offer diverse movement types — not all Foundational.
Consider what the learner has already covered (their path) to avoid
repetition and suggest meaningful next directions.
```

### Output Format (Structured)

The agent returns JSON (via structured output or function calling):

```json
{
  "proposals": [
    {
      "title": "Finger Placement & First Notes",
      "description": "Learn proper finger positioning on the six holes to produce clean notes Sa through Pa.",
      "movement": "Foundational",
      "resources": [
        {
          "type": "youtube",
          "youtube_id": "abc123",
          "title": "Bansuri Fingering Guide for Beginners",
          "channel": "Flute Gurukul",
          "reason": "Clear close-up shots of finger placement, covers all basic notes in 12 minutes"
        }
      ]
    },
    ...
  ]
}
```

---

## Node Data Model (Updated with Resources)

Resources found by the AI agent are stored on the node:

```cypher
(:Node {
  id: "uuid",
  title: "Finger Placement & First Notes",
  description: "Learn proper finger positioning...",
  topic_root_id: "uuid",
  embedding: [float],
  visit_count: 0,
  resources: [                          // JSON array stored as string
    {
      "type": "youtube",
      "youtube_id": "abc123",
      "title": "Bansuri Fingering Guide",
      "channel": "Flute Gurukul",
      "reason": "Clear close-up shots..."
    }
  ],
  created_at: "..."
})
```

---

## When Does the AI Agent Run?

NOT on every request. Only when the graph needs new content:

| Scenario | AI Agent Runs? | Why |
|---|---|---|
| enterTopic (new topic) | YES | Need starting points from scratch |
| enterTopic (existing topic) | MAYBE | Only if existing starting points are stale/few |
| Proposals: rich graph (3+ weighted edges) | NO | Use existing graph edges, ranked by weight |
| Proposals: sparse graph (< 3 edges) | YES | Need to fill remaining slots |
| Proposals: wildcard slot | SOMETIMES | Can generate new node OR use low-weight existing |
| startFrom | NO | Just setting position, proposals generated separately |
| traverse | NO (for traversal itself) | Just updating weights + position |
| backUp | NO | Just updating weights + position |

**Key insight:** The AI agent is expensive (multiple API calls + tool use). It runs only when the graph can't serve the request from existing data. As the graph fills up, AI calls decrease. A mature graph for "Bansuri" might serve most requests entirely from existing weighted edges.

---

## GraphQL Schema Update (Resources on Nodes)

```graphql
type Resource {
  type: String!              # "youtube", "article", "guide"
  youtubeId: String           # for youtube resources
  url: String                 # for web resources
  title: String!
  channel: String             # for youtube
  reason: String!             # AI's rationale for picking this resource
}

type Node {
  id: ID!
  title: String!
  description: String!
  resources: [Resource!]!     # curated resources found by AI
  proposals(sessionId: ID!): [Proposal!]!
}
```

---

## Second Learner Experience

```
Person B types "Indian flute" → enterTopic
│
▼
1. Embed "Indian flute" → vector search → matches "Bansuri" TopicRoot (0.92 similarity)
2. Existing TopicRoot has 3 starting points already (from Person A's traversal)
3. Return those starting points immediately — NO AI agent call needed
│
▼
Person B picks "Understanding Sa Re Ga" → startFrom
│
▼
1. This node already has 2 outgoing edges (from Person A):
   - SUPPORTS → "Playing Sa Re Ga on Bansuri" (weight: 0.8, traversals: 4)
   - DEEPENS → "Raga Theory Basics" (weight: 0.5, traversals: 2)
2. Need 1 more slot → AI agent generates 1 wildcard
3. Proposals: 2 proven + 1 wildcard (AI-generated with investigation)
│
▼
Person B's choices now also update edge weights,
further refining the graph for Person C...
```
