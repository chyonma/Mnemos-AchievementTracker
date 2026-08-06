# Mnemos — Figma

> Describes how Mnemos's design work is organized in Figma, and how design and
> engineering stay in sync. This document does not contain design decisions itself —
> those live in UI_GUIDELINES.md and the Figma file. This is a pointer and process doc.

**Status:** Draft — v1

---

## 1. Purpose of This Document

Design tools and files change independently of the codebase — links break, file structure gets reorganized, new contributors need to find the current source of truth. This document exists so anyone joining the project knows *where* the design lives and *how* it's organized, without needing to ask.

---

## 2. Source of Truth

- **Figma file:** `[link to be added once the file exists]`
- The Figma file is the source of truth for visual design specifics (exact spacing values, component states, screen layouts). `UI_GUIDELINES.md` is the source of truth for the *principles* those specifics should follow — if a specific Figma frame seems to contradict UI_GUIDELINES.md, the guidelines take precedence and the Figma frame should be corrected.

---

## 3. File Organization (Convention)

To keep the file navigable as it grows, pages should follow a consistent structure:

- **Foundations** — type scale, color tokens, spacing scale, iconography. The Figma-native expression of UI_GUIDELINES.md Sections 4–6.
- **Components** — reusable UI components (buttons, cards, list items, navigation elements) as Figma components/variants, not one-off shapes duplicated per screen.
- **Screens — Library** — library view and its states (empty, populated, scanning).
- **Screens — Game Detail** — detail view and its states (with/without achievement provider, with/without session history).
- **Screens — Statistics** — aggregate stats views.
- **Screens — Settings / Manual Linking** — configuration and manual-linking flows.
- **Archive** — deprecated or superseded designs, kept for reference rather than deleted, so design decisions have a visible history.

---

## 4. Component-First Discipline

Screens should be assembled from the Components page wherever possible, rather than redrawn per screen. This mirrors the engineering principle in ARCHITECTURE.md of shared, reusable structure over duplicated one-offs — a design inconsistency (e.g. a card with different corner radii in two places) is the design-layer equivalent of the code duplication the architecture is trying to avoid.

---

## 5. Design–Engineering Sync

- New or changed components should be reflected in the frontend's component library in the same rough timeframe as the Figma change — the two should not drift silently over time.
- When a design decision meaningfully changes or clarifies a principle (not just a screen-specific choice), it should be reflected back into `UI_GUIDELINES.md`, not left implicit in Figma alone. Figma shows *what* a pattern looks like; UI_GUIDELINES.md records *why*, so future screens can be designed consistently without reverse-engineering intent from existing frames.

---

## 6. Status Tracking

Each screen/flow in the Figma file should carry a simple status to avoid ambiguity about what's ready to build:

- **Exploratory** — early direction, not ready for implementation.
- **Ready for dev** — finalized, safe to implement against.
- **Implemented** — built and shipped; kept in Figma for reference, not active iteration.

---

## 7. Non-Goals of This Document

- Does not contain actual design specifications — that's the Figma file itself.
- Does not duplicate UI_GUIDELINES.md content.
- Not a substitute for design review — it describes structure, not process/approval workflow (which can be added here later if the project's contributor model requires it).