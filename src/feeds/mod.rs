use feed_rs::model::Image;
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
        image_text: get_image_text(&feed),
        ..Default::default()
    };

    Ok(FeedData {
        feed: the_feed,
        items: vec![],
    })
}

/// get an image from a feed, prefers big image over icon
fn get_image(feed: &feed_rs::model::Feed) -> Option<Image> {
    feed.logo
        .clone()
        .map(|v| v.clone())
        .or(feed.icon.clone().map(|v| v.clone()))
}

/// get image url for a feed
fn get_image_url(feed: &feed_rs::model::Feed) -> Option<String> {
    get_image(feed).map(|img| img.uri)
}

/// get text of an image, prefers description over title
fn get_image_text(feed: &feed_rs::model::Feed) -> Option<String> {
    let img = get_image(feed);
    if img.is_none() {
        return None;
    }
    let img = img.unwrap();

    if let Some(description) = &img.description {
        return Some(description.into());
    }

    if let Some(title) = &img.title {
        return Some(title.into());
    }

    None
}

#[derive(JsonSchema, Serialize, Default)]
pub struct FeedData {
    feed: Feed,
    items: Vec<FeedItem>,
}
