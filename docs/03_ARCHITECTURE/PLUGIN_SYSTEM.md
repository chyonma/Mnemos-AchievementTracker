# Mnemos — Plugin System

> Describes the long-term plugin architecture Mnemos is designed to grow into.
> This is a forward-looking design document, not a v1 implementation spec — v1 ships
> no plugin system. Its purpose is to ensure v1's provider interface (ARCHITECTURE.md
> Section 5) is shaped correctly today so this system can be built later without
> redesigning that interface.

**Status:** Conceptual — not scheduled before ROADMAP.md Phase 5

---

## 1. Why This Document Exists Now, Before Any Plugin Code

Per PROJECT_BIBLE.md's "incremental progress over premature optimization" principle, Mnemos should not build plugin infrastructure before it's needed. But it *should* understand, directionally, what that infrastructure will eventually require — otherwise v1's provider interface risks being shaped around Steam's specifics rather than around what a general plugin needs, and Phase 5 (ROADMAP.md) would then require a redesign instead of an extension.

This document exists to de-risk that specific failure mode. It does not authorize building any of this now.

---

## 2. Relationship to the Provider System

The achievement provider interface (ARCHITECTURE.md Section 5) is the plugin system's first and narrowest instance — a "plugin" that's currently limited to one capability (achievements) and hardcoded into the backend rather than dynamically loaded. The plugin system, when built, generalizes this pattern in two directions:

1. **Broader capability surface** — not just achievement providers, but potentially other extension points (future data sources, integrations) as the product grows.
2. **Dynamic loading** — v1's Steam provider is compiled into the app. A true plugin system allows third-party code to be added without modifying or recompiling Mnemos itself.

Everything below describes the target for direction (2). Direction (1) — what else becomes pluggable — is intentionally left open; scoping it prematurely would be exactly the kind of premature abstraction the Guiding Principles warn against.

---

## 3. Two Categories of Plugin

Distinguishing these early matters because they carry very different trust and technical requirements:

### 3.1 Official/Verified Providers
Additional platform integrations (e.g. Epic, GOG) that Mnemos or trusted maintainers build and ship, following the same pattern as the Steam provider. These are functionally similar to v1's Steam provider — API-based, read-only, low trust risk — just implemented as loadable plugins rather than compiled-in.

### 3.2 Community/Mnemos-Native Achievement Packs
Achievement definitions and unlock logic authored by the community or by Mnemos itself for games with no official achievement system (PROJECT_BIBLE.md Section 8). This category is fundamentally different and higher-risk:
- Unlock detection likely requires inspecting game state directly — file contents, save data, possibly memory — rather than calling a trusted API.
- Packs are data/logic authored by parties Mnemos doesn't control, for games Mnemos doesn't control either.
- This is the category that actually requires a trust and moderation model; official providers largely don't.

**This distinction should carry through to implementation:** official providers and achievement packs will likely need different loading mechanisms and different levels of sandboxing, not one uniform "plugin" pathway.

---

## 4. Design Requirements This Places on the Provider Interface (v1-relevant)

This is the part that actually matters for what we build now. For the provider interface designed in ARCHITECTURE.md Section 5 to remain valid once the plugin system arrives, it must already satisfy:

- **No assumption of API access.** The interface must not presume a provider talks to a remote API — a future achievement-pack provider may derive unlock state entirely from local file/save inspection. *(Already satisfied — ARCHITECTURE.md 5.4 notes this explicitly.)*
- **Self-contained recognition logic.** Each provider decides for itself whether it applies to a given Installation, rather than core dictating recognition rules. *(Already satisfied — ARCHITECTURE.md 5.2.)*
- **No compiled-in assumption baked into core.** Core code should depend on the *interface*, not on "Steam" specifically, so swapping compiled-in for dynamically-loaded later doesn't touch core logic. *(Should be verified during v1 implementation, not just assumed from the design doc.)*

No new v1 work is implied by this section — it's a checklist to validate against during implementation, not a new feature.

---

## 5. Anticipated Trust & Safety Model (Directional Only)

Not designed in detail here, but the shape of the eventual problem is worth naming so it's not a surprise later:

- Official providers ship through Mnemos's own review (similar trust level to core code).
- Community achievement packs will need some review or sandboxing layer before being loaded, given they involve inspecting arbitrary game state — this is a meaningfully larger engineering and moderation commitment than anything in v1, and is explicitly called out in ROADMAP.md Phase 5 rather than assumed to be solved by the interface alone.

---

## 6. Non-Goals of This Document

- Does not define a plugin manifest format, loading mechanism, or API surface — those are Phase 5 implementation decisions, not v1-adjacent ones.
- Does not commit to a specific sandboxing technology.
- Does not expand v1 scope in any way. If anything in this document seems to suggest v1 work, that's a documentation error, not an intended addition to FEATURES.md.