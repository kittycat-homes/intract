-- This file should undo anything in `up.sql`
ALTER TABLE users DROP COLUMN powerlevel;
ALTER TABLE users
ADD COLUMN power_level INT NOT NULL;
DROP TYPE powerlevel;