-- This file should undo anything in `up.sql`
ALTER TABLE users
DROP CONSTRAINT uc_unique_username;