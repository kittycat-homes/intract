-- This file should undo anything in `up.sql`
ALTER TABLE feeds
ADD COLUMN update_frequency TIMESTAMP NOT NULL;

ALTER TABLE feeds
DROP COLUMN last_checked;