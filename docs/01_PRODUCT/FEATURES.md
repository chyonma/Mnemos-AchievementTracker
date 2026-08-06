# Mnemos — Features

> Defines what Mnemos does at a feature level for v1, and what's planned beyond it.
> Derived from PROJECT_BIBLE.md — every feature here maps to a goal defined there.

**Status:** v1 draft

---

## 1. Game Detection & Library

### 1.1 Process-Based Game Detection
Mnemos monitors running Windows processes and matches them against the local game library by executable path. When a match is found, Mnemos begins tracking a play session automatically. No launcher integration is required for this to work.

### 1.2 Library Scanning
Mnemos scans common install locations (Steam library folders, Epic/GOG install directories, user-specified custom folders) to discover installed games. Each discovered installation becomes a library entry with:
- Unique internal ID
- Executable path
- Executable name
- Install directory
- Known launcher (if detected — informational only, not required for tracking)

### 1.3 Manual Game Linking
When automatic detection fails (unrecognized executable, unusual install location, misidentified process), the player can manually link an executable to a library entry, or create a new one.

### 1.4 Installation-Scoped Identity (v1)
Each discovered install path is tracked as its own library entry. Multiple installs of the same game (e.g. Steam + GOG copies) appear as separate entries in v1. *(Merge/Link Installations is a post-v1 feature — see Roadmap.)*

---

## 2. Playtime & Session Tracking

### 2.1 Automatic Session Tracking
When a tracked game's process starts, Mnemos begins a session; when it exits, the session ends and is recorded. This works for every library entry, regardless of achievement provider availability.

### 2.2 Playtime History
Mnemos aggregates session data into total playtime per game, and retains individual session history (start time, end time, duration) rather than only a running total.

### 2.3 Manual Session Correction
Players can edit or delete incorrectly recorded sessions (e.g. a session left running accidentally) to keep their history accurate.

---

## 3. Achievements

### 3.1 Steam Achievement Provider (v1's official provider)
For games identified as Steam titles, Mnemos retrieves achievement definitions and unlock status via Steam's API and displays them alongside the game's tracked history.

### 3.2 Provider-Based Architecture
Achievements are sourced through an interchangeable provider interface. Steam is the only implemented provider in v1, but the system is built to support additional official providers and a future Mnemos-native provider without redesigning core achievement handling.

### 3.3 Graceful Handling of No Provider
Games with no available achievement provider are not marked "unsupported" — they simply show no achievement data, while playtime, sessions, and history remain fully functional.

---

## 4. Library & History View

### 4.1 Unified Library View
A single view listing all tracked games — regardless of launcher or source — showing playtime, last played, and achievement progress where available.

### 4.2 Game Detail View
Per-game view showing session history, total playtime, and achievement list/progress (if a provider exists for that game).

### 4.3 Basic Statistics
Aggregate stats across the library: total playtime, most played games, recently played, achievement completion overview.

---

## 5. Data & Storage

### 5.1 Local-First Storage
All library, session, and achievement data is stored locally on the player's machine. No account or cloud service is required to use Mnemos.

### 5.2 Data Portability
Player data is stored in a format that can reasonably be backed up or migrated by the player (exact format — e.g. local SQLite — to be finalized in ARCHITECTURE.md / DATABASE.md).

---

## Out of Scope for v1

(See PROJECT_BIBLE.md Section 6 for full rationale.)
- Social features (friends, leaderboards, chat)
- Cloud accounts / sync
- Mobile apps
- Store/purchasing integration
- Automatic achievement unlocking
- Linux/macOS support
- Merge/Link Installations (multi-install identity)
- Plugin system / Mnemos-native achievement engine

---

## Roadmap Pointer

Features planned beyond v1 (Linux support, Merge/Link Installations, plugin system, Mnemos-native achievement engine, additional official providers) are tracked in `ROADMAP.md`, not here — this document reflects committed v1 scope only.s