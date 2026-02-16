---
date: 2026-02-15
topic: learning-app
---

# Learning App

## What We're Building

A personal, AI-powered learning app that generates recursive, explorable curricula. The curriculum is a multi-dimensional graph where every node offers three paths: dive deeper, more like this, or move on. YouTube videos are discovered, analyzed, and ranked by AI for each node. Resources are prefetched 2 levels ahead for smooth navigation.

## Core Concepts

- **Node** — a chapter or page with content + curated videos
- **Three actions at every node:**
  - **Dive deeper** — generates a sub-curriculum (vertical)
  - **More like this** — generates sibling nodes (horizontal)
  - **Move on** — presents branching next options (forward)
- Diving deeper never mutates the parent — it's a zoom, not an edit
- You can always pop back out to where you were
- The curriculum is a living, multi-dimensional choose-your-own-adventure graph
- Any node (chapter or page) can spawn its own subtree on demand

## Video Pipeline (Restate workflow)

1. AI extracts search queries from node content
2. Search YouTube
3. Analyze video metadata/content
4. Compare and rank candidates
5. Present best matches on the node

## Prefetching Strategy

- On visiting any node, prefetch 2 levels out in all directions (deeper, siblings, next paths)
- Lazy beyond that — only generate when the learner approaches
- This ensures navigation feels smooth and instant

## Tech Stack

- **Dioxus** — web frontend (Rust)
- **Rust + Restate** — backend server, durable workflows for curriculum generation + video discovery
- **Cargo workspace monorepo** — following taproot's `/modules/` pattern
- **Postgres** — curriculum graph storage
- **Docker Compose** — local dev

## Key Design Decisions

- **Personal tool** — no accounts, onboarding, or multi-user features
- **Recursive structure** — the curriculum is fractal; any node can become its own full curriculum
- **Non-destructive exploration** — diving deeper never mutates the parent curriculum
- **AI-curated resources** — not just search results, but analyzed, compared, and ranked videos
- **Choose your own adventure** — at every node, learner picks their path (dive, explore, advance)
- **Branching progression** — chapters lead to multiple possible next chapters at similar skill level

## Example: Learning Bansuri

1. Generate initial curriculum for "bansuri"
2. Chapters: Basics, Breath Control, Fingering, Meends, Ragas, ...
3. At "Meends" chapter → **Move on** offers paths to "Gamak", "Ragas", or "Taan"
4. At "Meends" chapter → **Dive deeper** generates sub-curriculum: Types of Meends, Meend Exercises, Meends in Raga Yaman, ...
5. At "Meends" chapter → **More like this** generates sibling content: additional meend lessons, different teaching perspectives
6. Each page has AI-curated YouTube videos ranked for quality and relevance

## Open Questions

- What AI provider for curriculum generation? (Anthropic, OpenAI, etc.)
- Do we need full-text content on pages or just titles + video collections to start?
- Any other resource types beyond YouTube eventually? (articles, books, etc.)

## Next Steps

- `/workflows:plan` for implementation details
