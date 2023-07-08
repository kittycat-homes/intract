use clap::ValueEnum;
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

#[derive(Queryable, Selectable, Serialize, Deserialize, JsonSchema, Insertable, Default)]
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
}

#[derive(Queryable, Selectable, Serialize, Deserialize, JsonSchema, Insertable, Default)]
#[diesel(table_name= crate::schema::feed_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FeedItem {
    pub id: Uuid,
    #[serde(skip)]
    pub feed_url: String,
    pub link: Option<String>,
    pub guid: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub image_text: Option<String>,
    pub media_description: Option<String>,
    pub author_name: Option<String>,
    pub title: Option<String>,
}
