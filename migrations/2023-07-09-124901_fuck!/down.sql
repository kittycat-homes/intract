-- This file should undo anything in `up.sql`
ALTER TABLE feeds
ADD COLUMN last_update TIMESTAMP NOT NULL;