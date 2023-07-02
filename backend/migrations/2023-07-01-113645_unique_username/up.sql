-- Your SQL goes here
ALTER TABLE users
ADD CONSTRAINT uc_unique_username UNIQUE (username);
