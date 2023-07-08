// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "powerlevel"))]
    pub struct Powerlevel;
}

diesel::table! {
    categories (name) {
        name -> Text,
    }
}

diesel::table! {
    feed_items (id) {
        id -> Uuid,
        feed_url -> Text,
        link -> Nullable<Text>,
        guid -> Nullable<Text>,
        description -> Nullable<Text>,
        image_url -> Nullable<Text>,
        image_text -> Nullable<Text>,
        media_description -> Nullable<Text>,
        author_name -> Nullable<Text>,
        author_link -> Nullable<Text>,
        title -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        synced_at -> Timestamp,
    }
}

diesel::table! {
    feed_items_have_categories (category_name, feed_item_id) {
        category_name -> Text,
        feed_item_id -> Uuid,
    }
}

diesel::table! {
    feeds (url) {
        url -> Text,
        title -> Nullable<Text>,
        description -> Nullable<Text>,
        link -> Nullable<Text>,
        image_url -> Nullable<Text>,
        image_text -> Nullable<Text>,
        last_update -> Timestamp,
    }
}

diesel::table! {
    feeds_have_categories (category_name, feed_url) {
        category_name -> Text,
        feed_url -> Text,
    }
}

diesel::table! {
    sessions (secret) {
        user_id -> Uuid,
        secret -> Text,
        description -> Nullable<Text>,
        expires_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Powerlevel;

    users (id) {
        id -> Uuid,
        display_name -> Nullable<Text>,
        username -> Text,
        signup_reason -> Nullable<Text>,
        pronoun -> Nullable<Text>,
        bio -> Nullable<Text>,
        powerlevel -> Powerlevel,
        salt -> Text,
        hash -> Text,
    }
}

diesel::table! {
    users_follow_feeds (user_id, feed_url) {
        user_id -> Uuid,
        feed_url -> Text,
    }
}

diesel::joinable!(feed_items -> feeds (feed_url));
diesel::joinable!(feed_items_have_categories -> categories (category_name));
diesel::joinable!(feed_items_have_categories -> feed_items (feed_item_id));
diesel::joinable!(feeds_have_categories -> categories (category_name));
diesel::joinable!(feeds_have_categories -> feeds (feed_url));
diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(users_follow_feeds -> feeds (feed_url));
diesel::joinable!(users_follow_feeds -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    feed_items,
    feed_items_have_categories,
    feeds,
    feeds_have_categories,
    sessions,
    users,
    users_follow_feeds,
);
