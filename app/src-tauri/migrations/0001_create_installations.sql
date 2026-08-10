CREATE TABLE installations (
    id TEXT PRIMARY KEY NOT NULL,
    executable_path TEXT NOT NULL,
    executable_name TEXT NOT NULL,
    install_directory TEXT NOT NULL,
    display_name TEXT NOT NULL,
    known_launcher TEXT,
    steam_app_id TEXT,
    manually_linked BOOLEAN NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
    );