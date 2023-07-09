-- Your SQL goes here
ALTER TABLE feeds
DROP COLUMN update_frequency;

ALTER TABLE feeds
ADD COLUMN last_checked TIMESTAMP NOT NULL;