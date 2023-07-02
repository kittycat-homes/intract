-- This file should undo anything in `up.sql`
ALTER TABLE users DROP COLUMN hash;
ALTER TABLE users
ADD COLUMN hash BYTEA NOT NULL;