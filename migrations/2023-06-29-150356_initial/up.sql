-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id UUID DEFAULT uuid_generate_v4 (),
    power_level INT NOT NULL,
    display_name TEXT,
    username TEXT NOT NULL,
    signup_reason TEXT,
    pronoun TEXT,
    bio TEXT,
    hash BYTEA NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE sessions (
    user_id UUID NOT NULL,
    secret text PRIMARY KEY,
    description TEXT,
    expires_at TIMESTAMP,

    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE TABLE feeds (
    url TEXT PRIMARY KEY,
    title TEXT,
    description TEXT,
    link TEXT,
    creator_id UUID,
    image_url text,
    image_text text,

    CONSTRAINT fk_creator FOREIGN KEY (creator_id) REFERENCES users (id)
);

CREATE TABLE feed_items (
    id UUID DEFAULT uuid_generate_v4 (),
    feed_url TEXT NOT NULL,
    link    text,
    guid    text,
    description text,
    image_url   text,
    image_text  text,
    media_description text,
    author_name text,
    author_link text,
    title text,

    PRIMARY KEY (id),
    CONSTRAINT fk_item_of FOREIGN KEY (feed_url) REFERENCES feeds (url)
);

CREATE TABLE categories (
    name text PRIMARY KEY
);

CREATE TABLE users_follow_feeds(
    user_id UUID NOT NULL,
    feed_url text NOT NULL,

    CONSTRAINT pk_users_follow_feeds PRIMARY KEY (user_id, feed_url),
    CONSTRAINT fk_follow_feed FOREIGN KEY (feed_url) REFERENCES feeds (url),
    CONSTRAINT fk_user_follows FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE TABLE feeds_have_categories(
    category_name text NOT NULL,
    feed_url text NOT NULL,

    CONSTRAINT pk_feeds_have_categories PRIMARY KEY (category_name, feed_url),
    CONSTRAINT fk_categorized_feed FOREIGN KEY (feed_url) REFERENCES feeds (url),
    CONSTRAINT fk_category_for_feed FOREIGN KEY (category_name) REFERENCES categories (name)
);

CREATE TABLE feed_items_have_categories(
    category_name text NOT NULL,
    feed_item_id UUID NOT NULL,

    CONSTRAINT pk_feed_items_have_categories PRIMARY KEY (category_name, feed_item_id),
    CONSTRAINT fk_category_for_feed_item FOREIGN KEY (category_name) REFERENCES categories (name),
    CONSTRAINT fk_categorized_feed_item FOREIGN KEY (feed_item_id) REFERENCES feed_items (id)
);