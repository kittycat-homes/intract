use aide::axum::{routing::post_with, ApiRouter};
use axum::{extract::State, Extension};
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use hyper::StatusCode;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{
    db::models::{FeedItem, User, UsersFollowFeeds},
    extractors::Json,
    feeds::{self, FeedData},
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
                    .response_with::<400, (), _>(|docs| {
                        docs.description("failed to parse or get rss feed")
                    })
                    .tag("feed")
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
) -> Result<Json<FeedData>, StatusCode> {
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

    Ok(Json(feed_data))
}
