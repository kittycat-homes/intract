-- Your SQL goes here
ALTER TABLE users
ADD COLUMN salt text NOT NULL;