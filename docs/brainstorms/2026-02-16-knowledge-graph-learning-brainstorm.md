---
date: 2026-02-16
topic: knowledge-graph-learning
---

# Knowledge Graph Learning System

## What We're Building

A learning system where the knowledge graph is generated on-demand by AI and grows through learner traversal. There is no pre-built curriculum. You express an interest, the AI proposes a starting point, and from there you navigate through a graph of concepts — each step offering meaningful directions to go next. The graph persists in Neo4j and grows organically as people learn.

The graph is the shared artifact. Everyone interested in the same topic traverses the same graph. No explicit sharing or collaboration features — the social layer is just using the same graph. Your traversal extends it, the next person benefits from what's already been mapped.

## Core Insight

Learning is not linear and not bucketed (beginner/intermediate/advanced). It's a gradient. A node doesn't have a level — the learner's relationship to the node is what has a level. The same graph serves different learners because their personal gradient determines what's relevant.

Some connections are "strongly recommended before" (like proper technique in guitar/Bansuri to avoid bad habits), but these are weighted edges, not hard gates. The graph expresses "most people benefit from going here first" without locking anyone into a rigid sequence.

## The System Is a Sense-Making Amplifier, Not a Teacher

The AI proposes, the learner decides, and collective traversal patterns become the real intelligence. The AI doesn't need to be a domain expert — it just needs to generate plausible next steps. Humans correct it through their choices over time.

The first person through a topic might miss fundamentals. That's okay. The next person who *knows* about fundamentals asks for them, which informs the AI and extends the graph. The path blazed by people informs the next ones about what the "best path" is — not the AI alone.

## The Interaction Loop

1. Learner expresses an interest ("Bansuri")
2. AI proposes a starting node
3. From any node, AI proposes several next steps (3 proposals: 2 high-weight, 1 wildcard)
4. Learner chooses where to go
5. Choice gets persisted as a real edge in Neo4j
6. Repeat

### What a Node Is

A node is lightweight — a **proposal**, not a container:
- **Title** — what this concept/skill is
- **Short description** — what it's about (for Creative nodes, this is a *prompt* rather than an explanation)
- **Direction** — the character of this step relative to where you came from

### Movements of Learning (Edge Types)

The core abstraction of the system. These are not content categories — they are **universal movements through knowledge**, domain-agnostic, applicable whether you're learning Bansuri, Rust, or French cooking.

| Movement | Meaning | Example (Bansuri) | Example (Rust) |
|---|---|---|---|
| **Foundational** | what supports this | breath control before playing | stack vs heap before ownership |
| **Intensifies** | more of this — deeper or harder | advanced breath techniques / faster ornamental passages | ownership edge cases / harder borrow checker puzzles |
| **Lateral** | what's adjacent | raga theory from technique | concurrency from ownership |
| **Creative** | make something with it | improvise over a raga | build a project |
| **Contextual** | why does this exist | Bansuri's role in Hindustani music | why Rust was created |

Five movements. "Deeper" and "Harder" merged into **Intensifies** — the learner doesn't care about the distinction. Both mean "more of this." The AI describes the flavor in the node's description without needing it as a separate edge type.

The taxonomy is about the **direction of movement**, not the content. Nodes are just topics. Edges carry the intelligence — they describe *how* one topic relates to another in the context of learning. This means the same graph engine works for any domain with zero configuration.

### Relationship Edges (Graph-Stored)

Movements describe the learner's experience. Relationship edges are what's actually stored in Neo4j. Each movement maps to a concrete relationship type:

| Movement (learner-facing) | Relationship Edge (graph-stored) | Direction |
|---|---|---|
| Foundational | `SUPPORTS` | A supports B |
| Intensifies | `DEEPENS` | B deepens A |
| Lateral | `RELATES_TO` | A relates to B |
| Creative | `APPLIES` | B applies A |
| Contextual | `CONTEXTUALIZES` | B contextualizes A |

No structural edges (`PART_OF`, `CONTRASTS_WITH`). These model encyclopedic relationships, not learning ones. The five movement edges are sufficient. If hierarchy is needed later, it can be derived from graph structure (nodes that support many of the same children form natural clusters).

## Graph Entry: Matching Learners to Existing Graphs

When a learner expresses an interest ("Indian bamboo flute"), the system uses **embeddings + LLM judgment** to find the right graph:

1. Embed the learner's query
2. Find nearest existing graph root by vector similarity
3. If close enough (e.g. "Indian bamboo flute" → Bansuri graph), attach to existing graph
4. If genuinely different, LLM decides: create new root or merge
5. This prevents fragmented parallel graphs ("Bansuri", "bansuri flute", "Indian bamboo flute") while keeping distinct topics separate

## Learner Modes

Different moments call for different modes of interaction:
- **Wander mode** — "where am I, what's near me" — local traversal, curiosity-driven
- **Goal mode** — "I want to learn X, show me a path" — pathfinding across the graph
- **Guide mode** — "just teach me the next thing" — AI recommends based on your gradient + aggregate patterns

The AI layer makes mode transitions feel natural through conversation rather than UI buttons.

## The Graph Grows

- The graph is lazy-loaded and learner-driven
- AI generates nodes/edges on demand as learners traverse
- Traversed paths get persisted as real graph data
- New learners inherit the existing graph — AI proposes paths through mapped territory and extends into unmapped territory
- Over time, traversal data reveals patterns: which paths lead to understanding, where people get stuck, which nodes turn out to be essential

### Keeping the Graph Coherent

As many learners traverse and extend the graph:
- **AI proposes paths through existing graph first** before generating new nodes
- **Edge weights from traversal signals** — not just visit counts, but *continuation patterns*: if a learner traverses A → B → C and keeps going, that path gets an implicit endorsement. If they traverse A → B and back up or restart, B gets an implicit downvote. Continuation = signal of value. This avoids self-reporting and doesn't require the AI to assess understanding.
- **Exploration budget** — when proposing next steps, the AI doesn't only recommend high-weight paths. It reserves one slot for a low-weight or brand-new proposal (2 proven + 1 wildcard). This prevents the graph from ossifying into a de facto linear curriculum and keeps the long tail alive.
- Potential use of external sources (Tavily, YouTube, Reddit searches) to ground the AI's proposals in real-world learning resources

## Key Decisions

- **Graph is generated, not hand-crafted**: AI proposes the graph on demand based on the learner's interest and current position
- **Single shared graph per topic**: No per-user graph creation. Everyone on "Bansuri" uses the same graph. Matching via embeddings + LLM.
- **No difficulty buckets**: Learning is a gradient, not beginner/intermediate/advanced
- **Nodes are lightweight**: Title + description + direction. Not full lessons or resource collections. Creative nodes use prompts as descriptions.
- **Five movements of learning are the core abstraction**: Foundational, Intensifies, Lateral, Creative, Contextual. No structural edges.
- **The learner sets their own gradient**: The person is the sense-making machine. AI proposes, humans decide.
- **Implicit weight signals**: Continuation = endorsement, backing up = downvote. No self-reporting needed.
- **Exploration budget in proposals**: 2 proven paths + 1 wildcard to prevent graph ossification.
- **Neo4j for persistence**: Graph database is a natural fit for the data model
- **Single-player first**: Social patterns emerge from shared graph usage, no collaboration features needed in v1

## First Version Scope

- Single-player
- AI-generated graph (using existing ai-client module)
- One domain to prove it out: Bansuri
- Core loop: express interest -> get proposals -> choose -> traverse -> graph grows

## Open Questions

- What does the UI look like? Graph visualization? Simple list of proposals? Conversational?
- How to deduplicate nodes as the graph grows (same concept, slightly different titles)? Embeddings could help here too.

## Next Steps

-> `/workflows:plan` for implementation details
