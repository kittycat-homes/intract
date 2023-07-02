-- This file should undo anything in `up.sql`
ALTER TABLE sessions
ALTER COLUMN expires_at DROP NOT NULL;