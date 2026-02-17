---
date: 2026-02-16
topic: inspire-movement
---

# Inspire Movement: Contextual Immersive Content Discovery

## What We're Building

Add a fifth movement type — **Inspire** — that surfaces contextual consumption content (artists, performances, real-world examples, communities) relevant to the learner's current position in the graph. Unlike the other four movements which answer "what should I learn next?", Inspire answers "who's doing this beautifully?"

Inspire nodes are fully traversable. A learner can discover Pannalal Ghosh via an Inspire node from "Bhoopali Raga," then go deeper into his discography, broader into his contemporaries, or back to Practice with "try playing this Ghosh phrase." This mirrors real immersive learning: see something beautiful → want to understand it → want to try it.

## Why This Approach

We tested the "traversable vs leaf node" question against real examples across domains (Bansuri, painting, Rust, baking) and in every case, traversable Inspire nodes create a natural "consume → understand → practice" loop. Leaf nodes would break flow by forcing the learner back to their lesson.

The key insight: **inspiration is contextual to where you are in the graph.** Learning Bhoopali? Find people playing Bhoopali. Not generic "famous Bansuri players."

## Key Decisions

- **Fully traversable**: Inspire nodes participate in the graph like any other node. All five movements work from them.
- **Fifth movement type**: Added to the Movement enum alongside Deeper/Broader/Foundation/Practice.
- **Depth = same as parent**: Inspire nodes sit at the same depth (like Broader/Practice). They're not harder or easier — they're a different axis.
- **Included in the default proposal mix**: Change the 3-proposal formula from "1 deeper, 1 practice, 1 broader/foundation" to "1 deeper, 1 practice, 1 broader/foundation/inspire." The third slot now rotates between broader, foundation, and inspire.
- **Resource-heavy**: Inspire nodes emphasize consumption resources (performances, talks, portfolios, galleries, codebases) rather than tutorials.
- **Works with steered expansion**: "Find me artists playing Bhoopali" is a natural steering prompt that would generate 3 Inspire nodes.

## Open Questions

- Should starting points include an Inspire node? (Leaning yes — a great onboarding moment)

## Next Steps

-> Plan implementation across backend + frontend
