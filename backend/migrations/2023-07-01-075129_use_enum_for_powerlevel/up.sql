-- Your SQL goes here
CREATE TYPE powerlevel AS ENUM ('unapproved', 'user', 'moderator', 'admin', 'owner');
ALTER TABLE users DROP COLUMN power_level;
ALTER TABLE users
ADD COLUMN powerlevel powerlevel NOT NULL;