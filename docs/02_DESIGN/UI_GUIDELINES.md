# Mnemos — UI Guidelines

> Defines the design principles, visual language, and interaction standards for Mnemos.
> This document governs *how* the UI should look, feel, and behave across the product —
> not what any individual screen contains. Screen-level design belongs in FIGMA.md
> and eventual screen specs.

**Status:** Draft — v1

---

## 1. Design Principles

These translate PROJECT_BIBLE.md's Guiding Principles into UI-specific terms:

- **Clarity over decoration.** The UI's job is to make a player's gaming history legible at a glance — playtime, progress, achievements. Visual flourish that doesn't serve that legibility is deprioritized.
- **Consistency over novelty.** The same kind of information (a game entry, a stat, a timestamp) should look and behave identically everywhere it appears. A player shouldn't have to relearn a pattern between the library view and the detail view.
- **Calm, not urgent.** Mnemos is a background companion and a personal record, not an engagement-driven product. Nothing in the UI should compete for attention the way a live-service or social app might — no aggressive animation, no artificial urgency, no red badges demanding action.
- **Respect the data, not the chrome.** Given the product is fundamentally about a player's history, the UI should foreground actual data (real playtime, real achievements) over ornamental UI elements.
- **Predictability over surprise.** Interactions behave the way their visual affordances suggest. No hidden gestures, no non-obvious ways to trigger destructive actions (like deleting session history).

---

## 2. Layout & Structure

- **Consistent shell.** A persistent navigation structure (library, statistics, settings) frames all views, so the player is never disoriented about where they are in the app.
- **Content-first hierarchy.** Primary content (game library, stats, achievement lists) occupies the majority of visual space; navigation and chrome are secondary and compact.
- **Predictable information density.** Each view type (list, detail, stats) uses a consistent density standard — list views are scannable and compact, detail views can be more spacious — but density shouldn't vary arbitrarily between similar views.
- **Responsive to window resizing**, since Mnemos is a desktop app players may run in varied window sizes, including as a smaller background companion window.

---

## 3. Navigation

- **Shallow hierarchy.** Core content (library → game detail) should be reachable in as few steps as possible. Avoid deep nested navigation for primary flows.
- **Persistent access to the library.** Since the unified library view is the core value proposition, it should always be one action away from anywhere in the app.
- **Back-navigation consistency.** Every drill-down view (e.g. game detail) has a clear, consistent way back, not just an implicit reliance on OS-level back gestures.
- **No modal-heavy flows for core browsing.** Modals are reserved for focused actions (manual linking, editing a session) — not for primary navigation between content.

---

## 4. Typography

- **A single primary typeface family**, chosen for readability at small sizes (list-heavy UI) and legibility for numeric data (playtime durations, percentages, dates).
- **A limited, deliberate type scale** (e.g. 4–5 sizes covering: page title, section header, primary content, secondary/metadata text, captions) rather than ad hoc sizing per screen.
- **Numeric data given visual priority** where it's the point of the UI (playtime totals, completion percentages) — typically through weight or size, not color alone, to remain accessible.
- **Consistent treatment of secondary text** (timestamps, install paths, metadata) — visually de-emphasized but always legible, never relying on low contrast alone to indicate hierarchy.

---

## 5. Color

- **Dark-first design.** Mnemos's primary interface is dark-themed by default, consistent with its identity as a background companion app used alongside games (which are typically played in dark or low-light environments, and often full-screen/borderless).
- **A light theme is a v1-or-near-term consideration, not a redesign risk** — color usage should be defined through semantic tokens (background, surface, text-primary, text-secondary, accent, success, warning) rather than hardcoded values, so a light theme can be introduced as an alternate token set later.
- **A single accent color** used deliberately and sparingly — primary actions, active states, achievement-unlock highlights — not scattered decoratively across the UI.
- **Semantic color use only where it aids understanding** (e.g. a completion indicator), never as the sole means of conveying meaning (accessibility — see Section 8).
- **Avoid launcher-specific branding colors** (Steam blue, Epic black, etc.) as UI accents — Mnemos's visual identity should not visually imply favoritism toward, or dependency on, any single platform, consistent with its platform-independence principle.

---

## 6. Spacing & Visual Rhythm

- **A consistent spacing scale** (e.g. a base unit and its multiples) applied uniformly across padding, margins, and gaps, rather than freeform per-component spacing.
- **Consistent alignment grid** across list items, cards, and detail views, so scanning the library feels uniform regardless of how much data an individual game entry has (e.g. a game with achievements vs. one without should still align cleanly).
- **Generous whitespace in detail views**, tighter density in list/library views — reflecting their different purposes (scanning vs. focused reading).

---

## 7. Animation & Motion

- **Purposeful, not decorative.** Motion should communicate state change (a session starting, an achievement unlocking) — not exist for visual polish alone.
- **Short, subtle durations.** Consistent with the "calm, not urgent" principle — transitions should feel smooth and quick, never showy or attention-grabbing.
- **Achievement unlock notifications get slightly more visual presence** than other UI feedback, since that's a moment of genuine player accomplishment worth acknowledging — but still restrained relative to how games themselves present unlocks (Mnemos is a record-keeper, not the game).
- **Respect reduced-motion preferences** at the OS level — animations should degrade to instant/minimal transitions when the player has requested reduced motion.

---

## 8. Accessibility

- **Color is never the sole indicator of meaning** — status, progress, and achievement state must also be conveyed through text, icons, or shape.
- **Sufficient contrast ratios** (targeting WCAG AA at minimum) across all text/background combinations, including secondary/metadata text.
- **Full keyboard navigability** for core flows — library browsing, game detail viewing, settings — not just mouse-driven interaction.
- **Legible at default OS scaling**, and functional (not broken) at larger accessibility-driven text/UI scaling settings.
- **Notifications are not the only channel for important state** — an achievement unlock should always also be visible in-app (e.g. in the achievement list), not rely solely on a transient OS notification the player might miss.

---

## 9. Overall Experience Standard

Taken together, these guidelines exist to make Mnemos feel like **trustworthy infrastructure, not another app competing for attention** — consistent, calm, legible, and consistent with its identity as a permanent record rather than a live, engagement-driven product. Any UI decision that trades this for visual novelty should be questioned, the same way FEATURES.md questions scope creep against PROJECT_BIBLE.md 