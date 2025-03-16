CREATE TABLE tracks (
    uuid TEXT NOT NULL PRIMARY KEY,
    app_data_id TEXT NOT NULL REFERENCES app_data(uuid),
    title TEXT NOT NULL,
    artist TEXT NOT NULL,
    album TEXT NOT NULL,
    cover_art TEXT NOT NULL,
    preview_url TEXT NOT NULL
)