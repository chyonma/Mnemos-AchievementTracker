# Mnemos — Roadmap

> High-level sequencing of Mnemos's development, from v1 through long-term vision.
> Derived from PROJECT_BIBLE.md (Sections 5, 8) and FEATURES.md.
> This is a directional roadmap, not a scheduled one — no dates are committed.

**Status:** Draft

---

## Phase 0 — Foundation (Pre-v1)

Not a feature phase — this is the engineering groundwork that has to exist before v1 features can be built.

- Finalize product documentation (PROJECT_BIBLE.md, FEATURES.md, ROADMAP.md, ARCHITECTURE.md, DATABASE.md, PLUGIN_SYSTEM.md)
- Set up Tauri + Rust + TypeScript development environment
- Establish core architectural boundaries: OS-abstraction interface, achievement provider interface, data layer
- Stand up an empty, running Tauri app as a known-good baseline

---

## Phase 1 — v1: Core Tracking + Steam

The scope defined in FEATURES.md. Goal: prove the core value proposition — a unified, platform-independent view of a player's gaming journey — for Windows, with Steam as the reference achievement provider.

- Process-based game detection
- Library scanning and manual linking
- Playtime and session tracking
- Steam achievement provider + unlock notifications
- Unified library view, game detail view, basic statistics
- Local-first data storage

**Exit criteria:** matches the v1 Success Criteria defined in PROJECT_BIBLE.md Section 9.

---

## Phase 2 — Stabilization & Installation Identity

Once v1 is functional, the priority shifts to correctness and the first real architectural extension, rather than new platforms.

- Merge/Link Installations — allow multiple installation entries to be linked into a single logical game identity, as scoped in PROJECT_BIBLE.md Section 3 (Identity Model)
- Hardening of process detection edge cases (shared executables, renamed binaries across updates)
- Refinement of manual linking UX based on real usage

---

## Phase 3 — Provider Expansion

Extends the achievement system beyond Steam, testing whether the provider architecture actually holds up to a second implementation — a direct validation of the Long-Term Success Criteria in PROJECT_BIBLE.md Section 9.

- Evaluate and implement additional official achievement providers where technically feasible (e.g. Epic, GOG — subject to API access realities)
- Begin design of the Mnemos-native achievement engine (community/self-authored achievement packs for games with no official provider)

---

## Phase 4 — Linux Support

- Implement the Linux side of the OS-abstraction layer established in Phase 0/v1 (process detection, game discovery, system integration)
- Validate that no core redesign is required — this is the direct test of the "extensibility without redesign" principle
- Steam Deck / Proton considerations addressed as part of this phase, given the target audience overlap

---

## Phase 5 — Plugin Ecosystem

- Formalize the provider interface into a broader plugin system, open to third-party developers
- Documentation and tooling for building/distributing plugins
- Governance/trust model for community-contributed plugins (particularly relevant for Mnemos-native achievement packs, which may involve file or memory inspection — a larger trust surface than read-only API integrations)

---

## Explicitly Not Scheduled

Consistent with PROJECT_BIBLE.md Section 6, the following are not on this roadmap at any phase:
- macOS support
- Social features, cloud sync, mobile apps
- Store/purchasing integration
- Any form of automatic achievement unlocking

---

## Sequencing Rationale

Phases are ordered by dependency and risk, not by feature appeal:
1. **v1 first** proves the core hypothesis (unified tracking has value) with the least architectural risk.
2. **Installation identity** comes early because it's a known v1 simplification with a defined future fix — better to resolve it before more systems are built on top of the current model.
3. **Provider expansion** comes before **Linux support** because it's a lower-risk test of the same underlying question (does the abstraction hold?) — validating the provider boundary is cheaper than validating the OS boundary.
4. **Plugin ecosystem** comes last because it depends on patterns proven in Phase 3 (a second achievement provider) and Phase 4 (a second OS) — opening the architecture to third parties before it's been tested internally would be premature.