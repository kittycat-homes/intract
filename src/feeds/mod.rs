use schemars::JsonSchema;
use serde::Serialize;

use crate::db::models::{Feed, FeedItem};

pub async fn parse_feed_from_url(uri: &str) -> Result<FeedData, Box<dyn std::error::Error>> {
    let response = reqwest::get(uri).await?;
    let mut parser_url = response.url().clone();
    parser_url.set_path("/");
    let bytes = &response.bytes().await?;
    let feed = feed_rs::parser::parse_with_uri(bytes.as_ref(), Some(parser_url.as_str()))?;
    tracing::debug!("got feed from {:#?}! {:#?}", parser_url.as_str(), feed);

    let the_feed = Feed {
        url: uri.into(),
        description: feed.clone().description.map(|v| v.content),
        link: feed.links.first().map(|v| v.clone().href),
        image_url: get_image_url(&feed),
        ..Default::default()
    };

    Ok(FeedData {
        feed: the_feed,
        items: vec![],
    })
}

fn get_image_url(feed: &feed_rs::model::Feed) -> Option<String> {
    if let Some(image) = feed.logo.clone().or(feed.icon.clone()) {
        return Some(image.uri);
    }
    None
}

#[derive(JsonSchema, Serialize, Default)]
pub struct FeedData {
    feed: Feed,
    items: Vec<FeedItem>,
}
