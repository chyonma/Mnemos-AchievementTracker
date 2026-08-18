CREATE TABLE games (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE provider_game_records (
    id TEXT PRIMARY KEY NOT NULL,
    game_id TEXT NOT NULL,
    provider TEXT NOT NULL,
    source_id TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (game_id) REFERENCES games(id),
    UNIQUE (provider, source_id)
);

ALTER TABLE installations ADD COLUMN provider_game_record_id TEXT;

CREATE TABLE achievement_definitions (
    id TEXT PRIMARY KEY NOT NULL,
    provider_game_record_id TEXT NOT NULL,
    provider_achievement_key TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    icon_url TEXT,
    fetched_at TEXT NOT NULL,
    FOREIGN KEY (provider_game_record_id) REFERENCES provider_game_records(id)
);

CREATE TABLE achievement_unlocks (
    id TEXT PRIMARY KEY NOT NULL,
    achievement_definition_id TEXT NOT NULL,
    unlocked_at TEXT,
    notified BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY (achievement_definition_id) REFERENCES achievement_definitions(id)
);