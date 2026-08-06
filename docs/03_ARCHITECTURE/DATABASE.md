# Mnemos — Database

> Concrete schema and storage decisions, building on the conceptual model in ARCHITECTURE.md Section 3.
> This document owns: storage technology choice, schema, relationships, and migration strategy.

**Status:** Draft — v1

---

## 1. Storage Technology

**Choice: SQLite, accessed via a Rust ORM/query layer (e.g. `sqlx` or `rusqlite`).**

Reasoning, evaluated against PROJECT_BIBLE.md's Guiding Principles:
- **Local-first:** SQLite is a single file on disk, no server process — matches "player-owned, local data" exactly, with no infrastructure dependency.
- **Reliability over cleverness:** SQLite is mature, embedded, and doesn't introduce a moving part (a database server) that could fail independently of the app.
- **Simplicity:** No setup burden for the player or the contributor — the database is just a file that ships with the app's data directory.
- **Data portability (FEATURES.md 5.2):** A single `.db` file is trivially backed up, copied, or inspected, satisfying the portability feature without extra tooling.

Alternatives considered and rejected: a remote/server database (violates local-first outright), a plain flat-file format like JSON (would work for v1's scale but doesn't hold up once querying playtime aggregates, session history, and achievement joins gets non-trivial — SQLite gives us that for free without meaningfully more complexity than flat files).

---

## 2. Schema (v1)

### `installations`
The unit of detection identity (ARCHITECTURE.md Section 3). Deliberately *not* called `games` — this table represents a discovered executable, not a logical game, consistent with v1's installation-scoped identity decision.

| Column | Type | Notes |
|---|---|---|
| `id` | TEXT (UUID) | Primary key |
| `executable_path` | TEXT | Full path, used for process matching |
| `executable_name` | TEXT | Derived, kept for display/debugging |
| `install_directory` | TEXT | |
| `display_name` | TEXT | User-facing name; auto-derived at scan time, user-editable |
| `known_launcher` | TEXT, nullable | e.g. `"steam"`, `"gog"`, `null` if unknown — informational only, never required for tracking |
| `steam_app_id` | TEXT, nullable | Populated if the Steam provider resolves this installation; enables provider lookup without re-resolving each time |
| `manually_linked` | BOOLEAN | True if this entry was created/confirmed via manual linking rather than automatic scan |
| `created_at` | DATETIME | |
| `updated_at` | DATETIME | |

### `sessions`
| Column | Type | Notes |
|---|---|---|
| `id` | TEXT (UUID) | Primary key |
| `installation_id` | TEXT | Foreign key → `installations.id` |
| `started_at` | DATETIME | |
| `ended_at` | DATETIME, nullable | Null while session is active |
| `duration_seconds` | INTEGER, nullable | Derived/stored on session end, avoids recomputing on every read |
| `manually_edited` | BOOLEAN | True if the player corrected this session (FEATURES.md 2.3) |

### `achievement_definitions`
Provider-supplied, cached locally so the library view doesn't require a live API call.

| Column | Type | Notes |
|---|---|---|
| `id` | TEXT (UUID) | Primary key |
| `installation_id` | TEXT | Foreign key → `installations.id` |
| `provider` | TEXT | e.g. `"steam"` — identifies which provider sourced this |
| `provider_achievement_key` | TEXT | Provider's own identifier for the achievement (e.g. Steam's API name) |
| `name` | TEXT | |
| `description` | TEXT | |
| `icon_url` | TEXT, nullable | |
| `fetched_at` | DATETIME | Last successful sync from provider |

### `achievement_unlocks`
Separate from definitions so unlock state can be refreshed independently and history (when something unlocked) is preserved.

| Column | Type | Notes |
|---|---|---|
| `id` | TEXT (UUID) | Primary key |
| `achievement_definition_id` | TEXT | Foreign key → `achievement_definitions.id` |
| `unlocked_at` | DATETIME, nullable | Null if not yet unlocked |
| `notified` | BOOLEAN | Whether the unlock notification (FEATURES.md 3.4) has already been shown, preventing duplicate notifications on re-sync |

---

## 3. Relationships
installations (1) ──< (many) sessions
installations (1) ──< (many) achievement_definitions
achievement_definitions (1) ──< (1) achievement_unlocks

Every relationship anchors to `installations`, not to a higher-level game concept — consistent with ARCHITECTURE.md Section 3's note that Sessions and Achievements attach to Installation, deliberately leaving room for a future `games` table (Phase 2, Merge/Link Installations) to sit *above* this layer without requiring these foreign keys to change.

---

## 4. Forward Compatibility: Installation Merging (Phase 2 Preview)

Not implemented in v1, but the schema is shaped to accommodate it without migration pain later:

- A future `games` table would introduce `game_id`, with `installations.game_id` as a nullable foreign key added via migration.
- Because `sessions` and `achievement_definitions` key off `installation_id` (not directly off any game concept), merging installations under a game later only requires populating that new column — it doesn't require moving or rewriting session/achievement history.

This is the concrete database-level expression of PROJECT_BIBLE.md's "extensibility without redesign" principle — worth calling out explicitly since it's easy to state as a principle and harder to actually satisfy at the schema level.

---

## 5. Migration Strategy

- Schema changes are handled via versioned migration files (e.g. `sqlx migrate`), applied automatically on app startup if the local database is behind the current schema version.
- Migrations are additive wherever possible (new nullable columns/tables) to avoid destructive changes to a player's existing local history — consistent with the preservation goal in PROJECT_BIBLE.md Section 5, Goal 5.
- No migration in v1 should require the player to lose existing session or achievement history; this is treated as a hard constraint on how future migrations (like the Phase 2 `games` table) are written, not just a nice-to-have.

---

## 6. Indexing Notes (v1)

Kept minimal deliberately — premature indexing is exactly the kind of optimization PROJECT_BIBLE.md's "incremental progress over premature optimization" principle argues against.

- `installations.executable_path` — indexed; this is the hot path for process-match lookups on every detected process start.
- `sessions.installation_id` — indexed; needed for per-game session history and playtime aggregation.
- Additional indexes deferred until real usage patterns (or profiling) justify them.