---
date: 2026-02-16
topic: system-data-flow-graph
---

# System Data Flow Graph

## Complete System Flow

```
                                    ┌──────────────┐
                                    │   LEARNER    │
                                    │  (browser)   │
                                    └──────┬───────┘
                                           │
                          ┌────────────────┼────────────────┐
                          │         React SPA               │
                          │                                 │
                          │  /login ──► /home ──► /start ──► /explore ◄─┐
                          │                                      │      │
                          │                              [pick] [back] [show more]
                          │                                      │      │
                          └──────────────────┬───────────────────┘──────┘
                                             │
                                        GraphQL POST /graphql
                                             │
                          ┌──────────────────▼───────────────────┐
                          │          AXUM SERVER                  │
                          │                                       │
                          │  ┌─── Cookie ──► JWT Guard ──► member_id
                          │  │                                    │
                          │  │    ┌──────────────────────────┐    │
                          │  │    │      MUTATIONS           │    │
                          │  │    │                          │    │
                          │  │    │  sendOtp ────────────────┼────┼──► Twilio
                          │  │    │  verifyOtp ─────────────┼────┼──► Twilio + JWT
                          │  │    │  enterTopic ────────────┼──┐ │
                          │  │    │  startFrom ─────────────┼──┤ │
                          │  │    │  traverse ──────────────┼──┤ │
                          │  │    │  backUp ────────────────┼──┤ │
                          │  │    │  showMore ──────────────┼──┤ │
                          │  │    │                          │  │ │
                          │  │    └──────────────────────────┘  │ │
                          │  │                                  │ │
                          └──┼──────────────────────────────────┼─┘
                             │                                  │
                             │         LEARNING-CORE            │
                             │                                  │
              ┌──────────────┼──────────────────────────────────┼──────────────┐
              │              │                                  │              │
              │   ┌──────────▼────────┐              ┌─────────▼──────────┐   │
              │   │   AUTH DOMAIN     │              │   GRAPH DOMAIN     │   │
              │   │                   │              │                    │   │
              │   │  otp.rs ──► Twilio│              │  entry.rs          │   │
              │   │  jwt.rs           │              │  traversal.rs      │   │
              │   │  identifier.rs    │              │  weights.rs        │   │
              │   │       │           │              │  dedup.rs          │   │
              │   │       ▼           │              │  client.rs         │   │
              │   │   Postgres        │              │       │            │   │
              │   │  (members,        │              │       ▼            │   │
              │   │   identifiers)    │              │   Memgraph         │   │
              │   └───────────────────┘              │  (nodes, edges,    │   │
              │                                      │   weights,         │   │
              │                                      │   embeddings)      │   │
              │                                      └────────▲───────────┘   │
              │                                               │               │
              │   ┌───────────────────────────────────────────┼───────────┐   │
              │   │            AI DOMAIN                      │           │   │
              │   │                                           │           │   │
              │   │   proposer.rs ─── "does graph have ──────►│           │   │
              │   │       │            enough edges?"                     │   │
              │   │       │                                              │   │
              │   │       │ NO (sparse/cold)     YES (rich graph)        │   │
              │   │       │                       │                      │   │
              │   │       ▼                       ▼                      │   │
              │   │   agent.rs              return existing              │   │
              │   │   (Investigation        edges ranked                 │   │
              │   │    Agent)               by weight                    │   │
              │   │       │                                              │   │
              │   │       │ multi-turn loop (up to 5 turns)              │   │
              │   │       │                                              │   │
              │   │       ├──► tool_call ──► tavily_search ──► Web       │   │
              │   │       │                     │                        │   │
              │   │       │                     ▼                        │   │
              │   │       │              articles, guides,               │   │
              │   │       │              learning resources              │   │
              │   │       │                                              │   │
              │   │       ├──► tool_call ──► youtube_search ──► YouTube  │   │
              │   │       │                     │              API       │   │
              │   │       │                     ▼                        │   │
              │   │       │              video list                      │   │
              │   │       │                                              │   │
              │   │       ├──► tool_call ──► youtube_details ──► YouTube │   │
              │   │       │                     │               API      │   │
              │   │       │                     ▼                        │   │
              │   │       │              full descriptions,              │   │
              │   │       │              AI reads & judges               │   │
              │   │       │              quality                         │   │
              │   │       │                                              │   │
              │   │       ├──► tool_call ──► existing_nodes ──► Memgraph │   │
              │   │       │                     │                        │   │
              │   │       │                     ▼                        │   │
              │   │       │              what already exists             │   │
              │   │       │              (avoid duplicates)              │   │
              │   │       │                                              │   │
              │   │       ▼                                              │   │
              │   │   proposals[] with resources                         │   │
              │   │       │                                              │   │
              │   │       ├──► embed each proposal                       │   │
              │   │       ├──► dedup against existing nodes              │   │
              │   │       ├──► persist new nodes to Memgraph             │   │
              │   │       └──► persist new edges to Memgraph             │   │
              │   │                                                      │   │
              │   └──────────────────────────────────────────────────────┘   │
              │                                                              │
              └──────────────────────────────────────────────────────────────┘
```

---

## Per-Mutation Data Flow

### enterTopic("Bansuri")

```
enterTopic("Bansuri")
    │
    ▼
┌─ GRAPH DOMAIN ──────────────────────────────────────────────────┐
│                                                                  │
│  1. Embed "Bansuri" ──► ai-client embedding call                 │
│                              │                                   │
│                              ▼                                   │
│  2. Vector search ──► Memgraph: CALL vector.search(              │
│                       'topic_embedding', $vec, 5)                │
│                              │                                   │
│                    ┌─────────┴──────────┐                        │
│                    │                    │                         │
│              score > 0.85         score < 0.85                   │
│              (match!)             (new topic)                    │
│                    │                    │                         │
│                    ▼                    ▼                         │
│           Use existing          Create TopicRoot                 │
│           TopicRoot             in Memgraph                      │
│                    │                    │                         │
│                    └────────┬───────────┘                        │
│                             │                                    │
│  3. Check existing ──► MATCH (t:TopicRoot {id: $id})             │
│     starting points        -[:STARTS_WITH]->(n:Node)             │
│                              │                                   │
│                    ┌─────────┴──────────┐                        │
│                    │                    │                         │
│              has 3+ nodes         has < 3 nodes                  │
│                    │                    │                         │
│                    ▼                    ▼                         │
│           Return existing        ──► AI DOMAIN                   │
│           starting points                                        │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
                                         │
                                         ▼
                          ┌─ AI DOMAIN ──────────────────────┐
                          │                                   │
                          │  Investigation Agent              │
                          │                                   │
                          │  Context:                         │
                          │    topic: "Bansuri"               │
                          │    task: "generate starting       │
                          │           points"                 │
                          │    existing: [any existing nodes] │
                          │                                   │
                          │  Agent loop:                      │
                          │    Turn 1: tavily_search(         │
                          │      "bansuri learning path       │
                          │       fundamentals beginner")     │
                          │           │                       │
                          │           ▼                       │
                          │    Reads results, decides what    │
                          │    to search on YouTube           │
                          │           │                       │
                          │    Turn 2: youtube_search(        │
                          │      "bansuri beginner tutorial") │
                          │           │                       │
                          │           ▼                       │
                          │    Gets video list, picks top     │
                          │    candidates to investigate      │
                          │           │                       │
                          │    Turn 3: youtube_details(       │
                          │      ["id1", "id2", "id3"])       │
                          │           │                       │
                          │           ▼                       │
                          │    Reads full descriptions,       │
                          │    judges quality, picks best     │
                          │           │                       │
                          │    Turn 4: existing_nodes(        │
                          │      topic_root_id)               │
                          │           │                       │
                          │           ▼                       │
                          │    Checks what exists, avoids     │
                          │    duplicates                     │
                          │           │                       │
                          │    Turn 5: Returns structured     │
                          │    output:                        │
                          │    [                              │
                          │      { title, description,        │
                          │        resources: [video, ...] }, │
                          │      { ... },                     │
                          │      { ... }                      │
                          │    ]                              │
                          │                                   │
                          └───────────────┬───────────────────┘
                                          │
                                          ▼
                          ┌─ PERSIST ────────────────────────┐
                          │                                   │
                          │  For each starting point:         │
                          │    1. Embed title+desc            │
                          │    2. Vector search existing      │
                          │       nodes (dedup)               │
                          │    3. If similar node exists:     │
                          │       reuse it                    │
                          │    4. If new: CREATE (:Node {     │
                          │       id, title, desc,            │
                          │       embedding, resources })     │
                          │    5. CREATE (topicRoot)           │
                          │       -[:STARTS_WITH]->(node)     │
                          │                                   │
                          └───────────────┬───────────────────┘
                                          │
                                          ▼
                          Return to frontend:
                          TopicEntry {
                            topicRoot: { id, name },
                            startingPoints: [
                              { node: { id, title, desc,
                                resources }, description }
                            ]
                          }
```

---

### traverse(fromNodeId, toNodeId, movement)

```
traverse(fromNodeId: "breath-control", toNodeId: "finger-placement", movement: SUPPORTS)
    │
    ▼
┌─ GRAPH DOMAIN ──────────────────────────────────────────────────┐
│                                                                  │
│  1. Persist/update edge in Memgraph                              │
│     MERGE (a:Node {id: $from})-[r:SUPPORTS]->(b:Node {id: $to}) │
│     SET r.traversals = r.traversals + 1                          │
│     SET r.weight = max(0, (r.traversals - 2 * r.backups))        │
│                    / max(1, r.traversals + r.backups)             │
│                                                                  │
│  2. Record in Postgres                                           │
│     INSERT INTO traversal_history                                │
│       (member_id, topic_root_id, node_id,                        │
│        previous_node_id, movement_type)                          │
│     UPDATE learner_position SET current_node_id = $to            │
│                                                                  │
│  3. Increment visit_count on target node                         │
│     SET b.visit_count = b.visit_count + 1                        │
│                                                                  │
└──────────────────────────┬───────────────────────────────────────┘
                           │
                           ▼
┌─ PROPOSAL GENERATION (for new current node) ────────────────────┐
│                                                                  │
│  proposer.rs orchestrates:                                       │
│                                                                  │
│  1. Query existing outgoing edges from "finger-placement"        │
│     MATCH (n:Node {id: $id})-[r]->(neighbor:Node)                │
│     RETURN neighbor, type(r), r.weight, r.traversals             │
│     ORDER BY r.weight DESC                                       │
│                                                                  │
│  2. Get learner's visited nodes (from Postgres)                  │
│     SELECT node_id FROM traversal_history                        │
│     WHERE member_id = $mid AND topic_root_id = $tid              │
│                                                                  │
│  3. Decision tree:                                               │
│                                                                  │
│     existing edges          existing edges          no existing  │
│     with traversal data     but unweighted          edges        │
│     ≥ 3                     1-2                     0            │
│        │                       │                       │         │
│        ▼                       ▼                       ▼         │
│     SERVE FROM GRAPH        MIX                     ALL AI       │
│     ┌──────────────┐     ┌──────────────┐       ┌────────────┐  │
│     │ slot 1: top  │     │ slot 1: best │       │ slot 1: AI │  │
│     │   weight     │     │   existing   │       │ slot 2: AI │  │
│     │ slot 2: 2nd  │     │ slot 2: AI   │       │ slot 3: AI │  │
│     │   weight     │     │ slot 3: AI   │       └─────┬──────┘  │
│     │ slot 3:      │     └──────┬───────┘             │         │
│     │  wildcard*   │            │                     │         │
│     └──────┬───────┘            │                     │         │
│            │                    │                     │         │
│            │          ┌─────────┴─────────────────────┘         │
│            │          │                                          │
│            │          ▼                                          │
│            │    AI DOMAIN: Investigation Agent                   │
│            │    (same multi-turn flow as above)                  │
│            │                                                     │
│            │    Context now includes:                             │
│            │    - current node: "Finger Placement & First Notes" │
│            │    - path so far: ["Breath Control" → "Finger       │
│            │      Placement"]                                    │
│            │    - existing neighbors: [any already connected]    │
│            │    - visited nodes: [breath-control,                │
│            │      finger-placement]                              │
│            │                                                     │
│            │    Agent uses this to:                               │
│            │    - Avoid proposing what learner already visited   │
│            │    - Avoid duplicating existing neighbors            │
│            │    - Build on the learner's trajectory              │
│            │    - Search for relevant next resources              │
│            │                                                     │
│     * wildcard = lowest-weight existing edge                     │
│       OR AI-generated if agent has something compelling          │
│                                                                  │
└──────────────────────────┬───────────────────────────────────────┘
                           │
                           ▼
                Return TraversalResult {
                  currentNode: { id, title, desc, resources },
                  proposals: [
                    { node, movement, isWildcard: false },
                    { node, movement, isWildcard: false },
                    { node, movement, isWildcard: true }
                  ]
                }
```

---

### showMore(nodeId)

```
showMore(nodeId: "finger-placement")
    │
    ▼
┌─ PROPOSAL GENERATION ──────────────────────────────────────────┐
│                                                                  │
│  1. Get ALL existing outgoing edges from this node               │
│     (not just top 3 — everything)                                │
│                                                                  │
│  2. Get what learner has already been SHOWN                      │
│     (current proposals on screen + any previous showMore)        │
│     From Postgres: recent proposal history for this node         │
│                                                                  │
│  3. Are there unshown existing edges?                            │
│     ┌──────────┴──────────┐                                      │
│     │                     │                                      │
│     YES                   NO                                     │
│     │                     │                                      │
│     ▼                     ▼                                      │
│   Serve unshown       AI Investigation Agent                     │
│   existing edges      with exclusion context:                    │
│   (ranked by weight)  "already proposed: [A, B, C, D, E, F]     │
│                        generate DIFFERENT directions"            │
│     │                     │                                      │
│     │                     ▼                                      │
│     │               Agent searches new angles                    │
│     │               Persists new nodes + edges                   │
│     │               Graph gets WIDER at this node                │
│     │                     │                                      │
│     └─────────┬───────────┘                                      │
│               │                                                  │
│               ▼                                                  │
│         Return 3 new proposals                                   │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

---

### backUp()

```
backUp()
    │
    ▼
┌─ GRAPH DOMAIN ──────────────────────────────────────────────────┐
│                                                                  │
│  1. Look up previous node from Postgres                          │
│     SELECT node_id, previous_node_id FROM traversal_history      │
│     WHERE member_id = $mid AND topic_root_id = $tid              │
│     AND is_backtrack = false                                     │
│     ORDER BY created_at DESC LIMIT 1                             │
│                                                                  │
│     previous = "breath-control"                                  │
│     abandoned = "finger-placement"                               │
│                                                                  │
│  2. Downvote abandoned edge in Memgraph                          │
│     MATCH (a:Node {id: $prev})-[r]->(b:Node {id: $abandoned})   │
│     SET r.backups = r.backups + 1                                │
│     SET r.weight = max(0, (r.traversals - 2 * r.backups))        │
│                    / max(1, r.traversals + r.backups)             │
│                                                                  │
│  3. Record backtrack in Postgres                                 │
│     INSERT INTO traversal_history                                │
│       (member_id, topic_root_id, node_id: $prev,                 │
│        previous_node_id: $abandoned, is_backtrack: true)         │
│     UPDATE learner_position SET current_node_id = $prev          │
│                                                                  │
│  4. Generate proposals for previous node                         │
│     (same proposal generation flow)                              │
│     Deprioritize the abandoned node in results                   │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

---

## The Investigation Agent's Decision Making

The agent isn't scripted. It decides what to search based on context.
Here's what a real multi-turn session might look like:

```
═══════════════════════════════════════════════════════════════
INVESTIGATION AGENT SESSION
═══════════════════════════════════════════════════════════════

SYSTEM PROMPT:
  You are investigating learning resources for a student.
  Current node: "Finger Placement & First Notes"
  Topic: Bansuri
  Path so far: Breath Control → Finger Placement
  Existing neighbors from this node: [none]
  Already visited: [Breath Control, Finger Placement]

  Find 3 next steps. Use your tools. Each proposal needs:
  - title, description, movement type, resources (videos/articles)
  Prefer diverse movement types.

───────────────────────────────────────────────────────────────
TURN 1 — Agent thinks: "They know breath control and finger
          placement. What logically comes next?"

  → tool_call: tavily_search(
      "bansuri learning after finger placement what next
       scales notes practice")

  ← results:
    - "After basic fingering, practice the saptak (octave)..."
    - "Meend (gliding between notes) is essential to bansuri..."
    - "Start with Raga Yaman — the most common first raga..."

───────────────────────────────────────────────────────────────
TURN 2 — Agent thinks: "Good — saptak, meend, and first raga
          are three natural directions. Let me find videos."

  → tool_call: youtube_search(
      "bansuri saptak practice basic scale")
  → tool_call: youtube_search(
      "bansuri meend technique tutorial")
  → tool_call: youtube_search(
      "bansuri raga yaman beginner lesson")

  ← results: ~10 videos per search

───────────────────────────────────────────────────────────────
TURN 3 — Agent thinks: "Let me look at the top candidates
          more closely to pick the best instructional ones."

  → tool_call: youtube_details(
      ["vid1", "vid2", "vid3", "vid4", "vid5", "vid6"])

  ← results: full descriptions, durations, channel info

  Agent reads descriptions:
    vid1: "In this lesson I cover all 7 notes of the saptak
           on a medium scale bansuri. We go note by note with
           close-up finger positions..." → GOOD. 14 min.

    vid2: "Bansuri vlog day 3 of my journey..." → BAD. Vlog.

    vid3: "Meend is the soul of bansuri. In this tutorial I
           show 5 meend exercises from Sa to Re, Re to Ga..."
           → GOOD. 18 min. Established channel.

    vid4: "Raga Yaman for absolute beginners. I break down
           the aroha, avroha, and pakad..."
           → GOOD. 22 min. 200K views.

───────────────────────────────────────────────────────────────
TURN 4 — Agent thinks: "I have what I need. Saptak is
          Foundational, Meend Intensifies the finger work,
          Raga Yaman is Creative (making music). Good spread."

  → Returns:
  [
    {
      title: "The Saptak — Your First Octave",
      description: "Practice all 7 notes (Sa Re Ga Ma Pa
        Dha Ni) ascending and descending on your bansuri.",
      movement: "Foundational",
      resources: [{
        type: "youtube", youtube_id: "vid1",
        title: "Bansuri Saptak Lesson - All 7 Notes",
        channel: "Flute Gurukul",
        reason: "Note-by-note walkthrough with close-up
          finger positions, 14 min, clear audio"
      }]
    },
    {
      title: "Meend — Gliding Between Notes",
      description: "The technique that gives bansuri its
        singing quality. Practice smooth transitions
        between adjacent notes.",
      movement: "Intensifies",
      resources: [{
        type: "youtube", youtube_id: "vid3",
        title: "5 Essential Meend Exercises for Bansuri",
        channel: "Bansuri Bliss",
        reason: "Structured exercises progressing from
          simple to complex, 18 min, great demonstrations"
      }]
    },
    {
      title: "Your First Raga — Yaman",
      description: "Put your notes to use by learning the
        most common beginner raga. Learn the ascending
        and descending patterns.",
      movement: "Creative",
      resources: [{
        type: "youtube", youtube_id: "vid4",
        title: "Raga Yaman - Complete Beginner Lesson",
        channel: "Indian Classical Hub",
        reason: "Breaks down aroha/avroha/pakad clearly,
          200K views, well-paced for beginners"
      }]
    }
  ]

═══════════════════════════════════════════════════════════════
```

---

## Graph Growth Over Time

```
PERSON A enters "Bansuri" (cold start — all AI)

  (TopicRoot: Bansuri)
       │
       ├── STARTS_WITH ──► [Breath Control] 🎥
       ├── STARTS_WITH ──► [Sa Re Ga] 🎥
       └── STARTS_WITH ──► [History of Bansuri] 🎥

  Person A picks Breath Control, then Finger Placement:

  (Bansuri)
       ├── STARTS_WITH ──► [Breath Control] ─── SUPPORTS (w:1.0) ──► [Finger Placement]
       ├── STARTS_WITH ──► [Sa Re Ga]
       └── STARTS_WITH ──► [History of Bansuri]

───────────────────────────────────────────────────────────────

PERSON B enters "Indian bamboo flute" (matches Bansuri, score 0.92)
  Sees existing starting points — NO AI call.
  Picks Sa Re Ga, then Raga Yaman:

  (Bansuri)
       ├── STARTS_WITH ──► [Breath Control] ─── SUPPORTS (w:1.0) ──► [Finger Placement]
       │                                             │
       │                                             ├── SUPPORTS (w:0) ──► [Saptak]
       │                                             ├── DEEPENS (w:0) ──► [Meend]
       │                                             └── APPLIES (w:0) ──► [Raga Yaman]
       │
       ├── STARTS_WITH ──► [Sa Re Ga] ─── APPLIES (w:1.0) ──► [Raga Yaman] ◄── (shared node!)
       └── STARTS_WITH ──► [History of Bansuri]

───────────────────────────────────────────────────────────────

PERSON C enters "bansuri" (exact match)
  Picks Breath Control (existing), then Saptak (existing edge, weight goes up):

  (Bansuri)
       ├── STARTS_WITH ──► [Breath Control] ─── SUPPORTS (w:1.0) ──► [Finger Placement]
       │                         │                   │
       │                    SUPPORTS (w:1.0)         ├── SUPPORTS (w:0.5) ──► [Saptak] ◄── weight increasing
       │                         │                   ├── DEEPENS (w:0) ──► [Meend]
       │                         ▼                   └── APPLIES (w:0) ──► [Raga Yaman]
       │                    [Finger Placement]
       │
       ├── STARTS_WITH ──► [Sa Re Ga] ─── APPLIES (w:1.0) ──► [Raga Yaman]
       └── STARTS_WITH ──► [History of Bansuri]

  Person C hits "show more" at Finger Placement:
  → AI investigates new directions
  → Adds: [Tonal Quality] (DEEPENS), [Western Flute Comparison] (RELATES_TO)
  → Graph gets wider at this node

───────────────────────────────────────────────────────────────

PERSON D enters "bansuri"
  At Finger Placement, now sees 5 existing edges ranked by weight:
  → Top 2 by weight (proven): Saptak, Raga Yaman
  → 1 wildcard: Meend (low weight) or Tonal Quality
  → NO AI call needed — graph is rich enough
```
