
-- Add migration script here
CREATE TABLE asset_categories (
    id TEXT PRIMARY KEY NOT NULL,
    account_id TEXT NOT NULL,
    name TEXT NOT NULL,
    target_ratio TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (account_id) REFERENCES accounts(id)
);

CREATE TRIGGER IF NOT EXISTS update_asset_categories_modtime
AFTER UPDATE ON asset_categories
BEGIN
    UPDATE asset_categories SET updated_at = CURRENT_TIMESTAMP WHERE id = old.id;
END;

CREATE TABLE user_asset_groupings (
    id TEXT PRIMARY KEY NOT NULL,
    account_id TEXT NOT NULL,
    asset_master_id TEXT NOT NULL,
    category_id TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (account_id) REFERENCES accounts(id),
    FOREIGN KEY (asset_master_id) REFERENCES asset_master(id),
    FOREIGN KEY (category_id) REFERENCES asset_categories(id)
);

CREATE TRIGGER IF NOT EXISTS update_user_asset_groupings_modtime
AFTER UPDATE ON user_asset_groupings
BEGIN
    UPDATE user_asset_groupings SET updated_at = CURRENT_TIMESTAMP WHERE id = old.id;
END;

CREATE TABLE assets (
    id TEXT PRIMARY KEY NOT NULL,
    account_id TEXT NOT NULL,
    asset_master_id TEXT NOT NULL,
    current_amount TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (account_id) REFERENCES accounts(id),
    FOREIGN KEY (asset_master_id) REFERENCES asset_master(id)
);

CREATE TRIGGER IF NOT EXISTS update_assets_modtime
AFTER UPDATE ON assets
BEGIN
    UPDATE assets SET updated_at = CURRENT_TIMESTAMP WHERE id = old.id;
END;
