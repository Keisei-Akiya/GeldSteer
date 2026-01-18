-- Add migration script here
CREATE TABLE asset_master (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    ticker_symbol TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER IF NOT EXISTS update_asset_master_modtime
AFTER UPDATE ON asset_master
BEGIN
    UPDATE asset_master SET updated_at = CURRENT_TIMESTAMP WHERE id = old.id;
END;
