CREATE TABLE IF NOT EXISTS analyses (
    id TEXT PRIMARY KEY,
    content_hash TEXT NOT NULL,
    content TEXT NOT NULL,
    platform TEXT NOT NULL,
    post_id TEXT,
    author TEXT,
    score INTEGER NOT NULL,
    confidence REAL NOT NULL,
    label TEXT NOT NULL,
    llm_score INTEGER,
    heuristic_score INTEGER,
    signals TEXT, -- JSON array
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_content_hash ON analyses(content_hash);
CREATE INDEX IF NOT EXISTS idx_created_at ON analyses(created_at);
