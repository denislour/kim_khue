CREATE TABLE IF NOT EXISTS chapters (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    slug TEXT NOT NULL UNIQUE,
    story_slug TEXT NOT NULL,
    position INTEGER NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    UNIQUE (story_slug, position)
);
