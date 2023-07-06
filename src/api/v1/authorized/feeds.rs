use aide::axum::{routing::post_with, ApiRouter};
use axum::Extension;
use hyper::StatusCode;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{
    db::models::User,
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
    Extension(_current_user): Extension<User>,
    Json(input): Json<FollowFeedInputData>,
) -> Result<Json<FeedData>, StatusCode> {
    let feed_data = feeds::parse_feed_from_url(&input.uri)
        .await
        .or(Err(StatusCode::BAD_REQUEST))?;
    Ok(Json(feed_data))
}
