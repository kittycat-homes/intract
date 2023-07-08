-- This file should undo anything in `up.sql`
-- Your SQL goes here
ALTER TABLE feed_items
DROP COLUMN created_at;

ALTER TABLE feed_items
DROP COLUMN updated_at;

ALTER TABLE feed_items
DROP COLUMN synced_at;

ALTER TABLE feeds
DROP COLUMN last_update;