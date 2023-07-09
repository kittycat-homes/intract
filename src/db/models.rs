use std::time::SystemTime;

use clap::ValueEnum;
use derivative::Derivative;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    diesel_derive_enum::DbEnum,
    Debug,
    Serialize,
    JsonSchema,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Copy,
    Clone,
    ValueEnum,
)]
#[ExistingTypePath = "crate::schema::sql_types::Powerlevel"]
pub enum Powerlevel {
    /// this user has no power and
    /// cannot do anything
    Unapproved,
    /// a regular user.
    /// they are allowed to use the service.
    User,
    /// a moderator can control other people's
    /// profiles
    Moderator,
    /// an admin has control over the server itself,
    /// as well as being able to control moderator
    /// and lower
    Admin,
    /// server owner can do anything they want :3
    Owner,
}

#[derive(Queryable, Selectable, Serialize, JsonSchema, Insertable, Clone)]
#[diesel(table_name= crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    /**
     * unique id for a user,
     * uses uuid v7.
     * this is based on time and makes ordering them
     * in the databse faster.
     */
    pub id: Uuid,
    /// how much this user is allowed to do
    pub powerlevel: Powerlevel,
    /**
     * username for logging in
     */
    pub username: String,
    /**
     * should be more prominently featured than username
     */
    pub display_name: Option<String>,
    #[serde(skip)]
    pub signup_reason: Option<String>,
    /// pronouns of this user, can be set back to none
    pub pronoun: Option<String>,
    /// bio of this user
    pub bio: Option<String>,
    #[serde(skip)]
    pub hash: String,
    #[serde(skip)]
    pub salt: String,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, JsonSchema, Insertable)]
#[diesel(table_name= crate::schema::sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Session {
    /**
     * unique id for a user,
     * uses uuid v7.
     * this is based on time and makes ordering them
     * in the databse faster.
     */
    pub user_id: Uuid,
    /**
     * session secret!
     * put this in the header called 'Key'
     */
    pub secret: String,
    /**
     * set during login.
     *
     * description for a session.
     * clients can set this to whatever they want,
     * or let the user set it themselves.
     *
     * something like time and client name can be useful.
     * just make sure they are human readable!
     */
    pub description: Option<String>,
    /**
     * timestamp when the token expires.
     * it will not be usable anymore
     * once the expiry date has passed.
     */
    pub expires_at: std::time::SystemTime,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, JsonSchema, Insertable)]
#[diesel(table_name= crate::schema::feeds)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Feed {
    /// url of the feed
    pub url: String,
    /// title of the feed
    pub title: Option<String>,
    /// description of the feed
    pub description: Option<String>,
    /// link to the feed
    pub link: Option<String>,
    /// link to an image specified for this feed
    pub image_url: Option<String>,
    /// title of that image, can be used for alt text
    pub image_text: Option<String>,
    /// when the feed was last checked
    pub last_checked: std::time::SystemTime,
}

#[derive(Derivative, Queryable, Selectable, Serialize, Deserialize, JsonSchema, Insertable)]
#[diesel(table_name= crate::schema::feed_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derivative(PartialEq)]
pub struct FeedItem {
    pub id: Uuid,
    #[serde(skip)]
    /// the feed that this item is a part of
    /// it's also the primary key for that
    pub feed_url: String,
    /// the link that leads to the thing this item
    /// is talking about
    pub link: Option<String>,
    /// should be unique but is not guaranteed!
    /// that is why we skip it for partial eq
    #[derivative(PartialEq = "ignore")]
    pub guid: Option<String>,
    /// short summary of the feed item
    pub description: Option<String>,
    /// link to an image
    pub image_url: Option<String>,
    /// used for alt text on images.
    /// description > title > None
    pub image_text: Option<String>,
    pub media_description: Option<String>,
    /// displays who wrote this
    pub author_name: Option<String>,
    /// title of this item
    pub title: Option<String>,
    /// when this item was last updated
    pub updated_at: std::time::SystemTime,
    /// when this item was created
    pub created_at: SystemTime,
    /// when this item was fetched from the server
    #[derivative(PartialEq = "ignore")]
    pub synced_at: SystemTime,
}
