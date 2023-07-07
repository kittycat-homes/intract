-- Your SQL goes here
ALTER TABLE feeds
DROP CONSTRAINT fk_creator;

ALTER TABLE feeds
DROP COLUMN creator_id;