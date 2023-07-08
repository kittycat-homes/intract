use feed_rs::model::Image;
use schemars::JsonSchema;
use serde::Serialize;
use uuid::Uuid;

use crate::db::models::{Feed, FeedItem};

pub async fn parse_feed_from_url(uri: &str) -> Result<FeedData, Box<dyn std::error::Error>> {
    let response = reqwest::get(uri).await?;
    let mut parser_url = response.url().clone();
    parser_url.set_path("/");
    let bytes = &response.bytes().await?;
    let feed = feed_rs::parser::parse_with_uri(bytes.as_ref(), Some(parser_url.as_str()))?;
    tracing::debug!("got feed from {:#?}! {:#?}", parser_url.as_str(), feed);

    let img = get_image_from_feed(&feed);
    let the_feed = Feed {
        url: uri.into(),
        description: feed.clone().description.map(|v| v.content),
        link: feed.links.first().map(|v| v.clone().href),
        image_url: get_image_from_feed(&feed).map(|img| img.uri),
        image_text: match &img {
            None => None,
            Some(i) => get_image_text(&i),
        },
        title: feed.title.map(|title| title.content),
        ..Default::default()
    };

    Ok(FeedData {
        feed: the_feed,
        items: feed
            .entries
            .iter()
            .map(|entry| into_feed_item(uri, entry))
            .collect(),
    })
}

/// get an image from a feed, prefers big image over icon
fn get_image_from_feed(feed: &feed_rs::model::Feed) -> Option<Image> {
    feed.logo
        .clone()
        .map(|v| v.clone())
        .or(feed.icon.clone().map(|v| v.clone()))
}

/// get text of an image, prefers description over title
fn get_image_text(img: &feed_rs::model::Image) -> Option<String> {
    if let Some(description) = &img.description {
        return Some(description.into());
    }

    if let Some(title) = &img.title {
        return Some(title.into());
    }

    None
}

fn into_feed_item(feed_url: &str, item: &feed_rs::model::Entry) -> FeedItem {
    FeedItem {
        id: Uuid::now_v7(),
        feed_url: feed_url.to_string(),
        link: item.links.first().map(|link| link.href.clone()),
        guid: Some(item.id.clone()),
        description: item.summary.clone().map(|summary| summary.content),
        author_name: item.authors.first().map(|author| author.name.clone()),
        title: item.title.clone().map(|title| title.content),
        ..Default::default()
    }
}

#[derive(JsonSchema, Serialize, Default)]
pub struct FeedData {
    /// the feed
    feed: Feed,
    /// the items that belong to this feed
    items: Vec<FeedItem>,
}
