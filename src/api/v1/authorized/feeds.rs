use aide::axum::{
    routing::{get_with, post_with},
    ApiRouter, IntoApiResponse,
};
use axum::{
    extract::{Query, State},
    Extension,
};
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use hyper::StatusCode;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{
    db::models::{Feed, FeedItem, User, UsersFollowFeeds},
    extractors::Json,
    feeds::{self, FeedData},
    schema::users_follow_feeds,
    state::AppState,
};

pub fn routes(state: AppState) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/follow",
            post_with(add_feed, |docs| {
                docs.id("follow-feed")
                    .summary("follow a feed")
                    .description("follow a new feed")
                    .response_with::<201, (), _>(|docs| {
                        docs.description("successfully followed this feed")
                    })
                    .response_with::<400, (), _>(|docs| {
                        docs.description("failed to parse or get rss feed")
                    })
                    .tag("feed")
            }),
        )
        .api_route(
            "/show/",
            get_with(get_feeds, |docs| {
                docs.description(
                    "show feeds followed by the \
                authenticated user",
                )
                .tag("feed")
                .id("show-feeds")
                .summary("show feeds")
            }),
        )
        .with_state(state)
}

#[derive(JsonSchema, Deserialize)]
pub struct FollowFeedInputData {
    /// the uri for your feed
    pub uri: String,
    /// only show this feed and items from it
    /// if show hidden is enabled
    pub hide: bool,
}

async fn add_feed(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Json(input): Json<FollowFeedInputData>,
) -> Result<StatusCode, StatusCode> {
    use crate::schema::feed_items::dsl::*;
    use diesel_async::RunQueryDsl;

    let mut conn = state
        .pool
        .get()
        .await
        .or(Err(StatusCode::SERVICE_UNAVAILABLE))?;

    let items: Vec<FeedItem> = feed_items
        .filter(feed_url.eq(&input.uri))
        .select(FeedItem::as_select())
        .load(&mut conn)
        .await
        .or(Err(StatusCode::SERVICE_UNAVAILABLE))?;

    let feed_data = feeds::parse_feed_from_url(&input.uri)
        .await
        .or(Err(StatusCode::BAD_REQUEST))?;

    // take out all the ones we already have
    let new_items: Vec<&FeedItem> = feed_data
        .items
        .iter()
        .filter(|new_item| !&items.contains(&new_item))
        .collect();

    // insert or update the feed
    let result = diesel::insert_into(crate::schema::feeds::table)
        .values(&feed_data.feed)
        .on_conflict(crate::schema::feeds::dsl::url)
        .do_update()
        .set(&feed_data.feed)
        .execute(&mut conn)
        .await;

    if let Err(e) = result {
        tracing::error!("failed to insert feed: {:#?}", e);
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    }

    let follow_data = UsersFollowFeeds {
        feed_url: feed_data.feed.url.clone(),
        user_id: current_user.id.clone(),
        hidden: input.hide,
    };

    let result = diesel::insert_into(crate::schema::users_follow_feeds::table)
        .values(&follow_data)
        .on_conflict((
            crate::schema::users_follow_feeds::dsl::feed_url,
            crate::schema::users_follow_feeds::dsl::user_id,
        ))
        .do_update()
        .set(&follow_data)
        .execute(&mut conn)
        .await;

    if let Err(e) = result {
        tracing::error!("failed to follow feed: {:#?}", e);
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    }

    let result = diesel::insert_into(crate::schema::feed_items::table)
        .values(new_items)
        .execute(&mut conn)
        .await;

    if let Err(e) = result {
        tracing::error!("failed to insert feed items: {:#?}", e);
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    }

    Ok(StatusCode::CREATED)
}

#[derive(JsonSchema, Deserialize)]
pub struct FeedsInput {
    /// show feeds that you have hidden
    pub show_hidden: bool,
}

async fn get_feeds(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Query(input): Query<FeedsInput>,
) -> Result<Json<Vec<Feed>>, StatusCode> {
    use diesel_async::RunQueryDsl;

    let mut conn = state
        .pool
        .get()
        .await
        .or(Err(StatusCode::SERVICE_UNAVAILABLE))?;

    let feeds: Vec<Feed> = users_follow_feeds::table
        .filter(users_follow_feeds::user_id.eq(current_user.id))
        .filter(
            users_follow_feeds::dsl::hidden.eq_any(if input.show_hidden {
                vec![true, false]
            } else {
                vec![false]
            }),
        )
        .inner_join(crate::schema::feeds::table)
        .select(Feed::as_select())
        .load(&mut conn)
        .await
        .or(Err(StatusCode::SERVICE_UNAVAILABLE))?;

    Ok(Json(feeds))
}
