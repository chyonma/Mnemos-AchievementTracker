CREATE TABLE sessions (
    if TEXT PRIMARY KEY NOT NULL,
    installation_id TEXT NOT NULL,
    started_at TEXT NOT NULL,
    ended-at TEXT,
    duration_seconds INTEGER,
    manually_edited BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY (installation_id) REFERENCES installations(id)

);