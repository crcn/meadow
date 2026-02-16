---
date: 2026-02-16
topic: future-canvas-features
---

# Future Canvas Features (Post-MVP)

## Movement Filter

Toggle which movement types are visible on the canvas. Filter edges by type to focus exploration.

- "Show me only Foundational paths" — see the core learning backbone
- "Show me Lateral connections" — discover what's adjacent
- "Show me Creative/Applied" — find hands-on projects

Implementation: opacity toggles on edges + connected proposal nodes. Simple UI — row of movement-colored toggle pills at the top of the canvas.

Could also be useful for "I've been going deep, now show me what's wide" moments.

---

## Live Mode

Go live on the meadow. Broadcast that you're here, see who else is exploring the same topic, and connect in real-time.

### The Vision

You're on the "Raga Yaman" node. You toggle Live Mode on. Your avatar glows on the canvas. Someone else learning Bansuri sees you — they're on the "Meend Technique" node two hops away. They tap your avatar. Now you're talking, maybe sharing screens, maybe picking up your instruments and practicing together.

This is the "practice partner" problem solved spatially. You don't search a directory. You just happen to be in the same meadow at the same time.

### Scoping

- **Global** — anyone on this topic graph can see you're live
- **Group** — create/join a named group (e.g., "Tuesday evening Bansuri circle"), only group members see each other

Groups could be invite-link based. No admin complexity. Just a shared meadow within the meadow.

### What "Live" Means

Spectrum of intensity:
1. **Presence only** — avatar visible on the node you're at, no communication (lightest)
2. **Chat** — text chat with others on the same topic graph
3. **Voice** — audio call (WebRTC), like a study room
4. **Video** — full video, screen sharing, practice together

MVP of live mode could be just presence + chat. Voice/video is a bigger lift (WebRTC, TURN servers, etc.)

### How It Maps to the Graph

- Your avatar appears on the node you're currently at
- Other live learners' avatars appear on their current nodes
- You can see movement — "oh, they just traversed to Saptak from Meend"
- Tapping an avatar opens a connection (chat → voice → video)
- The graph becomes a live map of who's learning what right now

### Technical Considerations

- WebSocket layer for real-time presence (probably a separate service)
- WebRTC for voice/video (or a service like LiveKit / Daily.co)
- Presence data is ephemeral — not stored, just broadcast
- Group membership stored in Postgres (simple join table)
- Rate of real-time updates is low (people traverse slowly) so scale isn't a concern early

### The Magic

This turns Our Meadow from a solo exploration tool into a *place*. A meadow you can actually bump into people in. The graph isn't just shared structurally — it's shared *in time*.
