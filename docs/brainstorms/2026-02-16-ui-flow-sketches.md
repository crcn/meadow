---
date: 2026-02-16
topic: ui-flow-sketches
---

# UI Flow Sketches

## Screen Flow

```
┌────────┐     ┌────────┐     ┌────────────┐     ┌──────────┐
│ Login  │────►│  Home  │────►│  Starting  │────►│ Explore  │◄─┐
│        │     │        │     │  Points    │     │          │  │
└────────┘     └────────┘     └────────────┘     └──────────┘  │
                                                    │    │     │
                                                    │    └─────┘
                                                    │   pick / show more
                                                    │
                                                    ▼
                                               ┌──────────┐
                                               │  Back to  │
                                               │  Home     │
                                               │ (new topic│
                                               └──────────┘
```

---

## Screen 1: Login

Simple. Phone number → code. Nothing else.

```
┌─────────────────────────────────────────────┐
│                                             │
│                                             │
│                                             │
│                                             │
│              ┌─────────────────┐            │
│              │                 │            │
│              │    🎵  Learn    │            │
│              │                 │            │
│              └─────────────────┘            │
│                                             │
│                                             │
│         ┌───────────────────────────┐       │
│         │  +1  (555) 123-4567       │       │
│         └───────────────────────────┘       │
│                                             │
│         ┌───────────────────────────┐       │
│         │       Send Code →         │       │
│         └───────────────────────────┘       │
│                                             │
│                                             │
│                                             │
└─────────────────────────────────────────────┘


After code sent:

┌─────────────────────────────────────────────┐
│                                             │
│                                             │
│                                             │
│         We sent a code to                   │
│         +1 (555) 123-4567                   │
│                                             │
│         ┌──┐ ┌──┐ ┌──┐ ┌──┐ ┌──┐ ┌──┐     │
│         │ 4│ │ 2│ │ 8│ │  │ │  │ │  │     │
│         └──┘ └──┘ └──┘ └──┘ └──┘ └──┘     │
│                                             │
│         ┌───────────────────────────┐       │
│         │       Verify →            │       │
│         └───────────────────────────┘       │
│                                             │
│         Didn't get it? Resend              │
│                                             │
│                                             │
└─────────────────────────────────────────────┘
```

---

## Screen 2: Home

The entry point. One question: what do you want to learn?

If returning learner has active topics, show those too.

```
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│                                                             │
│                                                             │
│                                                             │
│                                                             │
│                  What do you want to learn?                  │
│                                                             │
│         ┌─────────────────────────────────────────┐         │
│         │  Bansuri                             →  │         │
│         └─────────────────────────────────────────┘         │
│                                                             │
│                                                             │
│                                                             │
│                                                             │
│                                                             │
│                                                             │
│                                                             │
│                                                             │
│                                                             │
│                                                             │
└─────────────────────────────────────────────────────────────┘


Returning learner with active topics:

┌─────────────────────────────────────────────────────────────┐
│                                                             │
│                                                             │
│                  What do you want to learn?                  │
│                                                             │
│         ┌─────────────────────────────────────────┐         │
│         │                                      →  │         │
│         └─────────────────────────────────────────┘         │
│                                                             │
│                                                             │
│         ─── Continue where you left off ───                 │
│                                                             │
│         ┌─────────────────────────────────────────┐         │
│         │                                         │         │
│         │  🎵  Bansuri                             │         │
│         │  Currently at: Finger Placement          │         │
│         │  Last visited 2 hours ago                │         │
│         │                                         │         │
│         └─────────────────────────────────────────┘         │
│                                                             │
│         ┌─────────────────────────────────────────┐         │
│         │                                         │         │
│         │  🦀  Rust                                │         │
│         │  Currently at: Ownership Basics          │         │
│         │  Last visited 3 days ago                 │         │
│         │                                         │         │
│         └─────────────────────────────────────────┘         │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## Screen 3: Starting Points

After entering a topic, the AI investigates and returns starting points.
This screen has a loading state while the agent works.

```
Loading state (agent is investigating):

┌─────────────────────────────────────────────────────────────┐
│                                                             │
│  ← Back                                                     │
│                                                             │
│                                                             │
│                                                             │
│                                                             │
│                  Bansuri                                     │
│                                                             │
│                  Finding the best starting                   │
│                  points for you...                           │
│                                                             │
│                  ◌ Searching learning resources              │
│                  ◌ Finding videos                            │
│                  ◌ Building your path                        │
│                                                             │
│                                                             │
│                                                             │
│                                                             │
│                                                             │
└─────────────────────────────────────────────────────────────┘


Loaded:

┌─────────────────────────────────────────────────────────────┐
│                                                             │
│  ← Back                                                     │
│                                                             │
│                                                             │
│         Bansuri                                              │
│         Where would you like to start?                       │
│                                                             │
│                                                             │
│         ┌─────────────────────────────────────────┐         │
│         │                                         │         │
│         │  Holding & Breath Control                │         │
│         │                                         │         │
│         │  How to hold the Bansuri and produce     │         │
│         │  your first clear note through proper    │         │
│         │  breath support.                         │         │
│         │                                         │         │
│         │  🎥  Bansuri Basics - First Sounds       │         │
│         │      Flute Gurukul · 45K views           │         │
│         │                                         │         │
│         └─────────────────────────────────────────┘         │
│                                                             │
│         ┌─────────────────────────────────────────┐         │
│         │                                         │         │
│         │  Understanding Sa Re Ga                  │         │
│         │                                         │         │
│         │  The foundational notes of Indian        │         │
│         │  classical music and how to find them    │         │
│         │  on Bansuri.                             │         │
│         │                                         │         │
│         │  🎥  Sa Re Ga Ma on Bansuri              │         │
│         │      Bansuri Bliss · 120K views          │         │
│         │                                         │         │
│         └─────────────────────────────────────────┘         │
│                                                             │
│         ┌─────────────────────────────────────────┐         │
│         │                                         │         │
│         │  The Bansuri in Indian Music             │         │
│         │                                         │         │
│         │  Context and history — why this          │         │
│         │  instrument matters and what makes       │         │
│         │  it unique.                              │         │
│         │                                         │         │
│         │  🎥  History of the Bansuri              │         │
│         │      Indian Classical Hub · 89K views    │         │
│         │                                         │         │
│         └─────────────────────────────────────────┘         │
│                                                             │
│                                                             │
│         ┌─────────────────────────────────────────┐         │
│         │         Show me more options             │         │
│         └─────────────────────────────────────────┘         │
│                                                             │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## Screen 4: Explore (The Main Loop)

This is where the learner spends most of their time. They're at a node,
they see what's here (title, description, resources), and they see
where they can go next.

```
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│  ← Back                                        Bansuri  🎵  │
│                                                             │
│  ┌───────────────────────────────────────────────────────┐  │
│  │                                                       │  │
│  │                                                       │  │
│  │   Holding & Breath Control                            │  │
│  │                                                       │  │
│  │   How to hold the Bansuri and produce your first      │  │
│  │   clear note through proper breath support.           │  │
│  │                                                       │  │
│  │   ┌───────────────────────────────────────────────┐   │  │
│  │   │  ▶  Bansuri Basics - First Sounds             │   │  │
│  │   │     Flute Gurukul · 14 min · 45K views        │   │  │
│  │   │                                               │   │  │
│  │   │     Clear demonstration of how to hold the    │   │  │
│  │   │     bansuri and produce your first tone.       │   │  │
│  │   └───────────────────────────────────────────────┘   │  │
│  │                                                       │  │
│  └───────────────────────────────────────────────────────┘  │
│                                                             │
│                                                             │
│  Where to next?                                             │
│                                                             │
│  ┌───────────────────────────────────────────────────────┐  │
│  │                                                       │  │
│  │   ◆ Foundational                                      │  │
│  │                                                       │  │
│  │   Finger Placement & First Notes                      │  │
│  │                                                       │  │
│  │   Learn proper finger positioning on the six holes    │  │
│  │   to produce clean notes Sa through Pa.               │  │
│  │                                                       │  │
│  │   🎥  Bansuri Fingering Guide · Flute Gurukul         │  │
│  │                                                       │  │
│  └───────────────────────────────────────────────────────┘  │
│                                                             │
│  ┌───────────────────────────────────────────────────────┐  │
│  │                                                       │  │
│  │   ▲ Intensifies                                       │  │
│  │                                                       │  │
│  │   Diaphragmatic Breathing for Bansuri                 │  │
│  │                                                       │  │
│  │   Go deeper into breath support — learn to sustain    │  │
│  │   long phrases and control volume with your           │  │
│  │   diaphragm.                                          │  │
│  │                                                       │  │
│  │   🎥  Advanced Breath Control · Bansuri Bliss         │  │
│  │                                                       │  │
│  └───────────────────────────────────────────────────────┘  │
│                                                             │
│  ┌───────────────────────────────────────────────────────┐  │
│  │                                                 ✦     │  │
│  │   ○ Contextual                                        │  │
│  │                                                       │  │
│  │   The Bamboo Flute Across Cultures                    │  │
│  │                                                       │  │
│  │   Explore how bamboo flutes appear in traditions      │  │
│  │   from India to China to South America.               │  │
│  │                                                       │  │
│  │   🎥  Bamboo Flutes of the World · World Music TV     │  │
│  │                                                       │  │
│  └───────────────────────────────────────────────────────┘  │
│                                                             │
│                                                             │
│  ┌───────────────────────────────────────────────────────┐  │
│  │              Show me more options                      │  │
│  └───────────────────────────────────────────────────────┘  │
│                                                             │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## Movement Type Visual Language

Each movement type has a distinct symbol + color so learners develop
an intuitive feel for the direction of each step.

```
  ◆ Foundational     — solid, grounding         — blue/indigo
  ▲ Intensifies      — rising, building         — amber/orange
  ◇ Lateral          — open, adjacent           — green/teal
  ★ Creative         — expressive, making       — purple/magenta
  ○ Contextual       — encompassing, background — gray/slate

  ✦ wildcard indicator (small, top-right of card)
```

---

## Interaction States

### Picking a proposal (click/tap)

```
  ┌───────────────────────────────────────────────────────┐
  │                                                       │
  │   ◆ Foundational                                      │
  │                                                       │░░
  │   Finger Placement & First Notes                      │░░
  │                                                       │░░  ← pressed state
  │   Learn proper finger positioning on the six holes    │░░    (slight scale
  │   to produce clean notes Sa through Pa.               │░░     + shadow)
  │                                                       │░░
  │   🎥  Bansuri Fingering Guide · Flute Gurukul         │░░
  │                                                       │░░
  └───────────────────────────────────────────────────────┘░░
    ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
```

### Loading proposals (after picking or show more)

```
  Where to next?

  ┌───────────────────────────────────────────────────────┐
  │                                                       │
  │   ░░░░░░░░░░░░░░░░                                    │
  │                                                       │
  │   ░░░░░░░░░░░░░░░░░░░░░░░░░░                          │
  │                                                       │
  │   ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░              │
  │   ░░░░░░░░░░░░░░░░░░░░░░                              │
  │                                                       │
  └───────────────────────────────────────────────────────┘

  ┌───────────────────────────────────────────────────────┐
  │   ░░░░░░░░░░░░░░░░░░░                                 │
  │   ...                                                 │
  └───────────────────────────────────────────────────────┘

  ┌───────────────────────────────────────────────────────┐
  │   ░░░░░░░░░░░░░░                                      │
  │   ...                                                 │
  └───────────────────────────────────────────────────────┘
```

### Back button pressed (explore screen)

```
  ← Back

  The "back" action is explicit. It means "this wasn't useful."
  The UI could show a subtle confirmation or just transition
  immediately to the previous node with new proposals.

  Option A: Immediate — just go back, new proposals appear
  Option B: Micro-feedback — brief "Got it" toast before transition
```

### Show me more (loading + results)

```
  Clicking "Show me more" replaces the current proposal cards
  with a loading state, then shows 3 new cards. The current
  node stays in place — only the proposals below change.

  Previous proposals fade out → skeleton loading → new proposals fade in.

  The learner can keep hitting "show me more" to widen the graph.
```

---

## Video Resource Interaction

When a proposal card shows a video, tapping the video could:

```
  Option A: Inline expand (stay on page)

  ┌───────────────────────────────────────────────────────┐
  │                                                       │
  │   ◆ Foundational                                      │
  │                                                       │
  │   Finger Placement & First Notes                      │
  │                                                       │
  │   Learn proper finger positioning on the six holes    │
  │   to produce clean notes Sa through Pa.               │
  │                                                       │
  │   ┌───────────────────────────────────────────────┐   │
  │   │                                               │   │
  │   │           ┌─────────────────────┐             │   │
  │   │           │    ▶  YouTube       │             │   │
  │   │           │    Embed            │             │   │
  │   │           │                     │             │   │
  │   │           └─────────────────────┘             │   │
  │   │                                               │   │
  │   │  Bansuri Fingering Guide for Beginners        │   │
  │   │  Flute Gurukul · 14 min                       │   │
  │   │                                               │   │
  │   └───────────────────────────────────────────────┘   │
  │                                                       │
  └───────────────────────────────────────────────────────┘


  Option B: Open in new tab (simpler for V1)

  🎥  Bansuri Fingering Guide · Flute Gurukul  ↗
      Clicking opens YouTube in new tab
```

---

## Mobile Considerations

The layout is naturally mobile-friendly — it's a vertical stack
of cards. On mobile:

```
┌──────────────────────────┐
│ ← Back         Bansuri 🎵│
│                          │
│ ┌────────────────────┐   │
│ │                    │   │
│ │ Holding & Breath   │   │
│ │ Control            │   │
│ │                    │   │
│ │ How to hold the    │   │
│ │ Bansuri and...     │   │
│ │                    │   │
│ │ 🎥 Bansuri Basics  │   │
│ │    Flute Gurukul   │   │
│ │                    │   │
│ └────────────────────┘   │
│                          │
│ Where to next?           │
│                          │
│ ┌────────────────────┐   │
│ │ ◆ Foundational     │   │
│ │                    │   │
│ │ Finger Placement   │   │
│ │ & First Notes      │   │
│ │                    │   │
│ │ Learn proper...    │   │
│ │                    │   │
│ │ 🎥 Fingering Guide │   │
│ └────────────────────┘   │
│                          │
│ ┌────────────────────┐   │
│ │ ▲ Intensifies      │   │
│ │                    │   │
│ │ Diaphragmatic      │   │
│ │ Breathing          │   │
│ │                    │   │
│ │ Go deeper into...  │   │
│ │                    │   │
│ │ 🎥 Advanced Breath │   │
│ └────────────────────┘   │
│                          │
│ ┌────────────────────┐   │
│ │ ○ Contextual    ✦  │   │
│ │                    │   │
│ │ The Bamboo Flute   │   │
│ │ Across Cultures    │   │
│ │                    │   │
│ │ Explore how...     │   │
│ │                    │   │
│ │ 🎥 Bamboo Flutes   │   │
│ └────────────────────┘   │
│                          │
│ ┌────────────────────┐   │
│ │  Show me more      │   │
│ └────────────────────┘   │
│                          │
└──────────────────────────┘
```

---

## Full User Journey (Happy Path)

```
1. OPEN APP
   └─► See login screen

2. SIGN IN
   └─► Enter phone → get code → verify
   └─► Redirected to Home

3. HOME
   └─► Type "Bansuri" → press enter
   └─► Loading: "Finding the best starting points..."

4. STARTING POINTS
   └─► See 3-5 starting points with video previews
   └─► Scan options...
   └─► Pick "Holding & Breath Control"

5. EXPLORE: Holding & Breath Control
   └─► Read description
   └─► Watch the video (opens YouTube or inline embed)
   └─► See 3 proposals: Foundational, Intensifies, Contextual
   └─► Pick "◆ Finger Placement & First Notes"

6. EXPLORE: Finger Placement & First Notes
   └─► Read description
   └─► Watch the fingering video
   └─► See 3 proposals: Saptak, Meend, Raga Yaman
   └─► Hmm, none of these feel right...
   └─► Click "Show me more"
   └─► Loading... new proposals appear
   └─► New proposals: Tonal Quality, Embouchure Fine-Tuning, Playing with a Tanpura
   └─► Pick "★ Playing with a Tanpura" (Creative)

7. EXPLORE: Playing with a Tanpura
   └─► Read description
   └─► Watch video of playing along with a tanpura drone
   └─► See 3 proposals
   └─► Actually, this was too advanced. Hit ← Back
   └─► Back at "Finger Placement" with new proposals
       (the edge to "Playing with a Tanpura" now has lower weight)
   └─► Pick "◆ The Saptak — Your First Octave"

8. EXPLORE: The Saptak — Your First Octave
   └─► Continue swimming...

9. LATER: COME BACK
   └─► Open app → Home
   └─► See "Continue where you left off: Bansuri — The Saptak"
   └─► Click → land right where they were
   └─► Keep going
```
