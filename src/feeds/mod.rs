use std::time::SystemTime;

use feed_rs::model::{Image, MediaObject};
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

    let img = get_image_from_feed(&feed);
    let the_feed = Feed {
        url: uri.into(),
        description: feed.clone().description.map(|v| v.content),
        link: feed.links.first().map(|v| v.clone().href),
        image_url: get_image_from_feed(&feed).map(|img| img.uri),
        image_text: match &img {
            None => None,
            Some(i) => get_image_text(i),
        },
        title: feed.title.map(|title| title.content),
        last_checked: std::time::SystemTime::now(),
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
    feed.logo.clone().or(feed.icon.clone())
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
    let img = get_image_for_feed_entry(item);
    FeedItem {
        id: Uuid::now_v7(),
        feed_url: feed_url.to_string(),
        link: item.links.first().map(|link| link.href.clone()),
        guid: Some(item.id.clone()),
        description: item.summary.clone().map(|summary| summary.content),
        author_name: item.authors.first().map(|author| author.name.clone()),
        title: item.title.clone().map(|title| title.content),
        image_url: img.clone().map(|i| i.uri),
        image_text: img.and_then(|i| get_image_text(&i)),
        media_description: None,
        // use unix epoch as fallback, since the
        // feeds must have been updated / created
        // at some point in the past
        created_at: item
            .published
            .map(|time| time.into())
            .unwrap_or(SystemTime::UNIX_EPOCH),
        updated_at: item
            .updated
            .map(|time| time.into())
            .unwrap_or(SystemTime::UNIX_EPOCH),
        synced_at: SystemTime::now(),
    }
}

fn get_image_for_feed_entry(entry: &feed_rs::model::Entry) -> Option<Image> {
    if let Some(from_thumbnail) = entry
        // first try first media item with a thumbnail
        .media
        .iter()
        .filter(|media_content| !media_content.thumbnails.is_empty())
        .collect::<Vec<&MediaObject>>()
        .first()
        .and_then(|media_item| media_item.thumbnails.first().map(|item| &item.image))
        .cloned()
    {
        return Some(from_thumbnail);
    }

    // get first media object that is also an image
    for media_object in entry.media.iter() {
        for media_content in media_object.content.iter() {
            if media_content
                .content_type
                .clone()
                .map(|mime| mime.type_() == "image")
                .unwrap_or(false)
            {
                if let Some(url) = media_content.clone().url {
                    return Some(Image {
                        uri: url.to_string(),
                        description: media_object.description.clone().map(|desc| desc.content),
                        height: None,
                        title: None,
                        link: None,
                        width: None,
                    });
                }
            }
        }
    }
    None
}

#[derive(JsonSchema, Serialize)]
pub struct FeedData {
    /// the feed
    pub feed: Feed,
    /// the items that belong to this feed
    pub items: Vec<FeedItem>,
}
