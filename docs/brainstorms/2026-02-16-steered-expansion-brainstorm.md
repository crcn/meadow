---
date: 2026-02-16
topic: steered-expansion
---

# Steered Expansion: Smarter "Show Me More"

## What We're Building

Evolve the existing `showMore` flow from a rigid "generate 3 proposals (1 deeper, 1 practice, 1 broader/foundation)" into a two-step steered expansion:

1. **Suggest**: Given a node, generate 3 contextual expansion prompts the user can pick from (lightweight LLM call, no tool-calling).
2. **Expand**: The user picks a prompt (or writes their own), and that prompt drives the existing investigation pipeline to generate proposals.

This lets learners say things like "more ragas at this level" or "songs I can play with my current skills" instead of always getting the fixed movement-type formula.

## Why This Approach

We considered:

- **Generic expand with configurable count/movement**: Too mechanical. Doesn't capture what the learner actually wants.
- **Fixed template prompts**: Simpler but misses the contextual magic. "More at this level" means very different things for a raga node vs a fingering technique node.
- **AI-generated contextual prompts**: Best of both worlds. The AI knows the node context and can suggest genuinely useful directions. Small additional cost (one lightweight LLM call with no tool-calling).

## Key Decisions

- **Two new operations, not one**: Suggestion generation is a separate cheap call from the full expansion. This keeps the UI responsive — show suggestions instantly, then run the heavy pipeline only when the user commits.
- **Reuse existing proposer pipeline**: The expand step is the same `agent::investigate()` → structured extraction flow, just with a user-steered prompt instead of the hardcoded movement-slot rules.
- **Remove the rigid 3-movement constraint for steered expansions**: When the user says "more ragas at this level," all 3 proposals should be ragas at that level — not forced into 1 deeper / 1 practice / 1 broader.
- **Keep the original `showMore` behavior as default**: If no steering prompt is provided (e.g., first visit to a node), fall back to the current 1/1/1 formula. Steered expansion is an enhancement, not a replacement.
- **Skill level comes from traversal path + depth**: No new skill-tracking infrastructure. The AI already receives the learner's traversal history and current depth, which is a strong proxy for skill level.
- **Remove the < 3 edge sparsity cap for steered expansions**: The current `showMore` skips AI if ≥ 3 edges exist. For steered expansions, always generate — the user explicitly asked for more.

## How It Works

### Step 1: Expansion Suggestions (New)

**GraphQL:**
```graphql
query ExpansionSuggestions($topicRootId: ID!, $nodeId: ID!): [String!]!
```

**Backend:**
- Build context: node title, description, depth, learner's traversal path
- Single LLM call (no tools) with a prompt like:

```
You are helping a learner decide what to explore next from "{node_title}" (depth {depth}).

Their learning path so far:
{traversal_path_context}

Suggest exactly 3 short, specific expansion directions the learner might want.
Each should be a natural phrase a learner would say, like:
- "More ragas at this difficulty"
- "Songs that use this technique"
- "Easier alternatives to practice first"

Return a JSON array of 3 strings.
```

- Returns 3 strings to display as buttons/chips in the UI.

### Step 2: Steered Expand (Evolved `showMore`)

**GraphQL:**
```graphql
mutation ShowMore($topicRootId: ID!, $nodeId: ID!, $prompt: String): TopicGraph!
```

The `prompt` parameter is optional:
- **If null**: Use the existing hardcoded investigation prompt (1 deeper, 1 practice, 1 broader/foundation). Backward compatible.
- **If provided**: Use a steered investigation prompt that incorporates the user's chosen direction.

**Steered prompt template:**
```
You are an AI learning investigation agent for Our Meadow.

The learner is at "{node_title}" (depth {depth}) and wants to explore:
"{user_prompt}"

Their learning path so far:
{traversal_path_context}

Investigate and propose exactly 3 learning nodes that match what the learner asked for.
All 3 should directly address the learner's request. Calibrate difficulty
to match the learner's current depth ({depth}) and learning history.

For each node, search for real resources (articles, videos, guides) and verify
they exist. Each node needs: title, description, movement type, and resources.

Movement types to use:
- DEEPER: if the proposed node is harder than the current one
- BROADER: if it's at a similar level
- FOUNDATION: if it's a prerequisite
- PRACTICE: if it's a hands-on exercise

Pick the movement that best fits each proposal — do NOT force a specific distribution.
```

The rest of the pipeline (tool-calling investigation, structured extraction, deduplication, depth calculation, graph assembly) stays exactly the same.

## What Changes

| Layer | Change |
|-------|--------|
| **Prompts** (`prompts.rs`) | Add `EXPANSION_SUGGESTIONS_PROMPT` and `STEERED_INVESTIGATION_PROMPT` templates |
| **Proposer** (`proposer.rs`) | Accept optional `prompt: Option<String>` in `generate_proposals()`. If present, use steered prompt and skip the < 3 edge cap. |
| **Agent** (`agent.rs`) | Add a lightweight `suggest()` function (single LLM call, no tools, returns `Vec<String>`) |
| **GraphQL mutation** (`mutation.rs`) | Add optional `prompt` param to `showMore`. Add `expansionSuggestions` query. |
| **GraphQL types** (`types.rs`) | No changes needed — suggestions are just `[String!]!` |
| **Frontend** (later) | `showMore` button → shows 3 suggestion chips + freeform input → picks one → calls `showMore(prompt)` |
| **Graph models** | No changes |
| **Assembler** | No changes |

## Open Questions

- Should we cache expansion suggestions per node, or regenerate each time? (Leaning toward regenerate — it's cheap and context-dependent on the learner's path)
- Should steered expansions count toward the sparsity heuristic? (Leaning no — let the graph grow organically when the learner is actively exploring)

## Next Steps

→ Implement backend changes (prompts, proposer, agent, GraphQL)
→ UI design for suggestion chips + freeform input (separate pass)
