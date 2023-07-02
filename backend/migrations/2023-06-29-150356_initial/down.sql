-- This file should undo anything in `up.sql`
DROP TABLE feed_items_have_categories;
DROP TABLE feeds_have_categories;
DROP TABLE users_follow_feeds;
DROP TABLE categories;
DROP TABLE feed_items;
DROP TABLE feeds;
DROP TABLE sessions;
DROP TABLE users;
DROP EXTENSION "uuid-ossp";