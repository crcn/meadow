---
date: 2026-02-16
topic: canvas-ui-duolingo-style
---

# Canvas UI — Duolingo-Style Learning Map

## The Idea

Instead of a card stack, the learner sees their knowledge graph as a visual
canvas — a map they're building as they learn. Nodes they've visited are
filled in. The current node is highlighted. Unvisited proposals glow at the
edges, inviting exploration. The graph grows visually as they traverse it.

Like Duolingo's skill tree, but it's a real graph — not a linear path.
It branches, connects, and widens based on what the learner explores.

---

## The Canvas — First Visit (Just Started)

The learner just picked "Holding & Breath Control" as their starting point.
The canvas is mostly empty — just the starting node and its proposals.

```
┌─────────────────────────────────────────────────────────────────────────┐
│                                                                         │
│  ← Home                                              Bansuri  🎵        │
│                                                                         │
│                                                                         │
│                                                                         │
│                                                                         │
│                                                                         │
│                         ╭───────────────────╮                           │
│             ◆ ·········│  Finger Placement  │                           │
│               SUPPORTS │  & First Notes     │                           │
│              ·         ╰───────────────────╯                           │
│             ·              (unvisited)                                   │
│            ·                                                            │
│           ·                                                             │
│      ╔═══════════════════════╗                                          │
│      ║                       ║                                          │
│      ║  Holding & Breath     ║  ◄── YOU ARE HERE                        │
│      ║  Control              ║      (highlighted, pulsing border)       │
│      ║                       ║                                          │
│      ║  🎥 Bansuri Basics    ║                                          │
│      ║     Flute Gurukul     ║                                          │
│      ║                       ║                                          │
│      ╚═══════════════════════╝                                          │
│            ·           ·                                                │
│           ·             ·                                               │
│          ·               ·                                              │
│         ·                 ·                                             │
│   ▲ ···                    ··· ○                                        │
│   DEEPENS                     CONTEXTUALIZES                           │
│      ╭──────────────────╮        ╭──────────────────────╮              │
│      │  Diaphragmatic   │        │  The Bamboo Flute    │              │
│      │  Breathing       │        │  Across Cultures     │  ✦           │
│      ╰──────────────────╯        ╰──────────────────────╯              │
│         (unvisited)                 (unvisited, wildcard)               │
│                                                                         │
│                                                                         │
│                              ┌───────────────┐                          │
│                              │ Show me more  │                          │
│                              └───────────────┘                          │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### Node Visual States

```
  ╔═══════════════╗
  ║  Current Node ║     Double border, highlighted/pulsing
  ║               ║     Full details visible (description + video)
  ╚═══════════════╝

  ╭───────────────╮
  │  Unvisited    │     Dashed/soft border, muted colors
  │  Proposal     │     Title + short description visible
  ╰───────────────╯     Clickable — leads to traversal

  ┌───────────────┐
  │  ● Visited    │     Solid border, filled/colored background
  │    Node       │     Shows title, small checkmark or dot
  └───────────────┘     Clickable — can revisit

  Edge lines:
  ·········  untraversed (dashed/dotted, muted)
  ─────────  traversed (solid, colored by movement type)
```

---

## After a Few Steps — The Map Grows

The learner went: Breath Control → Finger Placement → Saptak.
The canvas shows their journey and the frontier of proposals.

```
┌─────────────────────────────────────────────────────────────────────────┐
│                                                                         │
│  ← Home                                              Bansuri  🎵        │
│                                                                         │
│                                                                         │
│                                                                         │
│                                                                         │
│                                      ╭──────────────────╮              │
│                          ◆ ·········│  Meend — Gliding  │              │
│                            SUPPORTS │  Between Notes    │              │
│                           ·         ╰──────────────────╯              │
│                          ·                                              │
│                         ·                                               │
│      ╔═══════════════════════╗                                          │
│      ║                       ║                                          │
│      ║  The Saptak — Your    ║  ◄── YOU ARE HERE                        │
│      ║  First Octave         ║                                          │
│      ║                       ║                                          │
│      ║  🎥 Bansuri Saptak    ║                                          │
│      ║     Flute Gurukul     ║                                          │
│      ║                       ║                                          │
│      ╚═══════════════════════╝                                          │
│          │         ·           ·                                         │
│          │        ·             ·                                        │
│     ◆    │       ·               ·                                      │
│  SUPPORTS│      ·                 ·                                     │
│          │ ★ ···                    ··· ○                                │
│          │ APPLIES                     CONTEXTUALIZES                   │
│          │    ╭──────────────────╮        ╭──────────────────────╮      │
│          │    │  Your First Raga │        │  The Scales of       │      │
│          │    │  — Yaman         │        │  Hindustani Music    │  ✦   │
│          │    ╰──────────────────╯        ╰──────────────────────╯      │
│          │       (unvisited)                 (unvisited, wildcard)       │
│          │                                                              │
│     ┌────┴──────────────┐                                               │
│     │  ● Finger         │     ╭──────────────────╮                      │
│     │    Placement      │     │  Diaphragmatic   │                      │
│     │    & First Notes  │     │  Breathing       │                      │
│     └───────┬───────────┘     ╰──────────────────╯                      │
│             │                    (unvisited — still there                │
│        ◆    │                     from before, faded)                    │
│     SUPPORTS│                                                           │
│             │                                                           │
│     ┌───────┴───────────┐                                               │
│     │  ● Holding &      │                                               │
│     │    Breath Control  │                                               │
│     └───────────────────┘                                               │
│                                                                         │
│                              ┌───────────────┐                          │
│                              │ Show me more  │                          │
│                              └───────────────┘                          │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### What You See

- **Your path** glows as solid lines: Breath Control → Finger Placement → Saptak
- **Visited nodes** (●) are filled in, recede to the background
- **Current node** is front and center with full details + video
- **Proposals** fan out from current node as unvisited nodes with dashed edges
- **Old unvisited proposals** (like "Diaphragmatic Breathing") are still visible but faded — you can still go back and explore them
- **The canvas auto-centers** on the current node, with gentle pan/zoom

---

## Deep Into the Graph — A Real Session

After 30 minutes, the learner has explored several branches.
The canvas zooms out a bit to show the shape of their journey.

```
┌─────────────────────────────────────────────────────────────────────────┐
│                                                                         │
│  ← Home                                    Bansuri  🎵    [−] [+] zoom │
│                                                                         │
│                                                                         │
│          ● Raga Yaman ──── ★ ···· ╭─ Improvise over ─╮                 │
│          │                 APPLIES│  Yaman            │                 │
│     ★    │                        ╰───────────────────╯                 │
│  APPLIES │                                                              │
│          │                                                              │
│     ● Sa Re Ga ──── ◆ ──── ● Saptak ═══════════╗                      │
│                   SUPPORTS        │              ║                      │
│                                   │         ╔════╝                      │
│                              ◆    │         ║                           │
│                           SUPPORTS│    ╔═══════════════╗                │
│                                   │    ║  Meend —      ║               │
│                              ● Finger  ║  Gliding      ║ ◄── HERE     │
│                                Placement║  Between      ║               │
│                                   │    ║  Notes        ║               │
│                              ◆    │    ║               ║               │
│                           SUPPORTS│    ║  🎥 5 Meend   ║               │
│                                   │    ║   Exercises   ║               │
│                              ● Breath  ╚═══════════════╝               │
│                                Control      ·         ·                │
│                                            ·           ·               │
│                                      ▲ ···               ··· ◇         │
│                                   DEEPENS              RELATES_TO      │
│                              ╭──────────────╮    ╭──────────────╮      │
│                              │  Gamak —     │    │  Meend in    │      │
│                              │  Ornamental  │    │  Vocal       │  ✦   │
│                              │  Shaking     │    │  Tradition   │      │
│                              ╰──────────────╯    ╰──────────────╯      │
│                                                                         │
│                              ┌───────────────┐                          │
│                              │ Show me more  │                          │
│                              └───────────────┘                          │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### Canvas Behaviors

- **Auto-center**: Camera follows current node, smooth pan animation
- **Pinch/scroll zoom**: Zoom out to see the full map, zoom in for detail
- **Tap visited node**: Pan to it, show brief info. Tap again to revisit (traverse back)
- **Tap proposal node**: Traverse to it (same as picking a card)
- **Edge colors**: Match movement type colors
  - ◆ Blue (Foundational/SUPPORTS)
  - ▲ Amber (Intensifies/DEEPENS)
  - ◇ Green (Lateral/RELATES_TO)
  - ★ Purple (Creative/APPLIES)
  - ○ Gray (Contextual/CONTEXTUALIZES)
- **Solid edges**: Traversed path (your journey)
- **Dotted edges**: Untraversed proposals (the frontier)

---

## Node Detail Panel

When you tap the current node or any visited node, a detail panel
slides up from the bottom (mobile) or appears as an expanded card.

```
┌─────────────────────────────────────────────────────────────────────────┐
│                                                                         │
│  [dimmed canvas visible above]                                          │
│                                                                         │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  (drag handle)              │
│                                                                         │
│  Meend — Gliding Between Notes                                          │
│                                                                         │
│  The technique that gives bansuri its singing quality.                   │
│  Practice smooth transitions between adjacent notes.                    │
│                                                                         │
│  ┌───────────────────────────────────────────────────────────────────┐  │
│  │                                                                   │  │
│  │              ┌─────────────────────────┐                          │  │
│  │              │      ▶  YouTube         │                          │  │
│  │              │      Embed / Thumbnail  │                          │  │
│  │              │                         │                          │  │
│  │              └─────────────────────────┘                          │  │
│  │                                                                   │  │
│  │  5 Essential Meend Exercises for Bansuri                          │  │
│  │  Bansuri Bliss · 18 min · 67K views                               │  │
│  │                                                                   │  │
│  │  "Structured exercises progressing from simple to complex,        │  │
│  │   great demonstrations of meend technique."                       │  │
│  │                                                                   │  │
│  └───────────────────────────────────────────────────────────────────┘  │
│                                                                         │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Starting Points — Canvas Style

Instead of a list of cards, the starting points fan out from the
topic root on the canvas. Feels like you're choosing your first
step on a map.

```
┌─────────────────────────────────────────────────────────────────────────┐
│                                                                         │
│  ← Home                                              Bansuri  🎵        │
│                                                                         │
│                    Where would you like to start?                        │
│                                                                         │
│                                                                         │
│                                                                         │
│          ╭─────────────────────╮                                        │
│          │  Understanding      │                                        │
│          │  Sa Re Ga           │                                        │
│          │                     │                                        │
│          │  🎥 Sa Re Ga Ma     │                                        │
│          │     on Bansuri      │                                        │
│          ╰──────────┬──────────╯                                        │
│                      ·                                                  │
│                       ·                                                 │
│                        ·                                                │
│                    ┌────────┐                                           │
│                    │ Bansuri│                                           │
│                    │   🎵   │                                           │
│                    └────────┘                                           │
│                   ·          ·                                           │
│                  ·            ·                                          │
│                 ·              ·                                         │
│     ╭──────────┴───────────╮   ╭┴─────────────────────╮                │
│     │  Holding & Breath    │   │  The Bansuri in       │                │
│     │  Control             │   │  Indian Music         │                │
│     │                      │   │                       │                │
│     │  🎥 Bansuri Basics   │   │  🎥 History of the    │                │
│     │     Flute Gurukul    │   │     Bansuri           │                │
│     ╰──────────────────────╯   ╰───────────────────────╯                │
│                                                                         │
│                                                                         │
│                              ┌───────────────┐                          │
│                              │ Show me more  │                          │
│                              └───────────────┘                          │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Canvas Implementation Notes

### Layout Algorithm

The graph needs to be laid out automatically. Options:

1. **Force-directed layout** (d3-force, @antv/layout)
   - Nodes repel each other, edges pull connected nodes together
   - Organic feel, good for exploration
   - Can be jittery — needs careful damping

2. **Dagre/hierarchical layout** (dagre, elkjs)
   - Top-to-bottom or left-to-right flow
   - More structured, like a skill tree
   - Might feel too rigid for a "choose your own adventure" graph

3. **Radial layout**
   - Current node at center, proposals radiate outward
   - Visited nodes form a trail behind
   - Natural focus on "where I am now"

**Recommendation for V1: Radial/force hybrid.**
- Current node is always centered
- Proposals fan out around it
- Visited path trails behind naturally
- Use force simulation to avoid overlap but with strong centering

### Canvas Library

For React:

- **React Flow** (reactflow.dev) — built for node-based UIs, handles
  pan/zoom/edges natively, good performance, customizable nodes
- **@xyflow/react** (v12) — same team, latest version
- **Canvas API / Pixi.js** — lower level, more control, more work
- **SVG + d3** — flexible but more manual

**Recommendation: React Flow.** It handles:
- Pan and zoom natively
- Custom node rendering (our styled cards)
- Edge rendering with labels and colors
- Automatic layout (with dagre plugin)
- Mobile touch support
- Minimap (optional, for zoomed-out overview)

### React Flow Node Types

```typescript
// Custom node types for our graph
const nodeTypes = {
  current:    CurrentNodeComponent,    // full detail, video, highlighted
  visited:    VisitedNodeComponent,    // compact, filled, clickable
  proposal:   ProposalNodeComponent,   // soft, movement badge, clickable
  topicRoot:  TopicRootComponent,      // small, central hub
};

// Custom edge types
const edgeTypes = {
  traversed:  TraversedEdge,    // solid, colored by movement
  proposal:   ProposalEdge,     // dashed, muted, colored by movement
};
```

### Data → React Flow Conversion

The GraphQL response needs to be converted to React Flow's format:

```typescript
// From GraphQL
type TraversalResult = {
  currentNode: Node;
  proposals: Proposal[];
};

// To React Flow
function toReactFlowElements(
  currentNode: Node,
  proposals: Proposal[],
  visitedNodes: Map<string, VisitedNode>,
  traversalHistory: TraversalEdge[]
): { nodes: RFNode[], edges: RFEdge[] } {
  // 1. Place current node at center
  // 2. Place proposals in radial positions around center
  // 3. Place visited nodes using force-directed positions (cached)
  // 4. Create traversed edges (solid) from history
  // 5. Create proposal edges (dashed) from current to proposals
}
```

---

## Screen Flow (Revised for Canvas)

```
┌────────┐     ┌────────┐     ┌────────────────────────────────────┐
│ Login  │────►│  Home  │────►│            Canvas                  │
│        │     │        │     │                                    │
└────────┘     └────────┘     │  Starting Points → Explore (same  │
                              │  canvas, graph grows as you go)    │
                              │                                    │
                              └────────────────────────────────────┘
```

Starting points and explore are the SAME screen now. You enter the
canvas when you pick a topic. It starts with just the topic root and
starting points fanned out. Once you pick one, the graph starts
growing and you keep exploring on the same canvas.

No separate "starting points" page needed. It's all one continuous
canvas experience.

---

## Revised Page Structure

```
web/
  src/
    pages/
      Login.tsx                 # Phone OTP flow
      Home.tsx                  # "What do you want to learn?" + continue
      Canvas.tsx                # THE main experience — graph canvas

    components/
      canvas/
        GraphCanvas.tsx         # React Flow wrapper, layout logic
        CurrentNode.tsx         # Full detail node (video, description)
        VisitedNode.tsx         # Compact visited node
        ProposalNode.tsx        # Unvisited proposal with movement badge
        TopicRootNode.tsx       # Central hub node
        MovementEdge.tsx        # Color-coded edge component
        DetailPanel.tsx         # Slide-up panel for node detail + video
        ShowMoreButton.tsx      # Floating button on canvas
      auth/
        LoginForm.tsx
        OtpInput.tsx
      home/
        TopicInput.tsx
        ContinueCard.tsx
```

---

## Mobile Canvas

React Flow supports touch gestures natively:
- **One finger drag**: Pan the canvas
- **Pinch**: Zoom in/out
- **Tap node**: Select → show detail panel
- **Tap proposal**: Traverse to it

The detail panel slides up as a bottom sheet on mobile:

```
┌──────────────────────────┐
│ ← Home       Bansuri  🎵 │
│                          │
│                          │
│    ● Breath ──── ● Finger│
│      Control       │     │
│                    │     │
│               ╔════╧═══╗ │
│               ║ Saptak ║ │
│               ║  ◄HERE ║ │
│               ╚════╤═══╝ │
│              ·     │   · │
│            ·       │     ·│
│     ╭─────╮   ╭───┴─╮  ╭┴────╮
│     │Meend│   │Raga │  │Scale│
│     │     │   │Yaman│  │Hist │
│     ╰─────╯   ╰─────╯  ╰─────╯
│                          │
│ ┌────────────────────────┤
│ │ Show me more           │
│ └────────────────────────┤
│                          │
│ ━━━━━━━━━━━━━━━━━━━━━━━━ │
│                          │
│ The Saptak — Your First  │
│ Octave                   │
│                          │
│ Practice all 7 notes     │
│ ascending and descending │
│                          │
│ ┌──────────────────────┐ │
│ │  ▶ Bansuri Saptak    │ │
│ │    Flute Gurukul     │ │
│ │    14 min            │ │
│ └──────────────────────┘ │
│                          │
└──────────────────────────┘
```
