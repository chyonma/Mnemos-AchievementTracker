CREATE TABLE watched_folders (
    id TEXT PRIMARY KEY NOT NULL,
    path TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL
);