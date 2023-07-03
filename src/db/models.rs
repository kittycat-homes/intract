use clap::ValueEnum;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
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
