-- This file should undo anything in `up.sql`
ALTER TABLE feeds
ADD COLUMN creator_id UUID;

ALTER TABLE feeds
ADD CONSTRAINT fk_creator FOREIGN KEY (creator_id) REFERENCES users (id);
