use std::time::{Duration, SystemTime};

use crate::{
    db::models::{Powerlevel, Session},
    pass::hash_password,
    schema::{sessions, users::dsl::*},
};
use aide::{
    axum::{routing::post_with, ApiRouter, IntoApiResponse},
    gen::infer_responses,
};
use axum::{
    extract::State, headers::AccessControlAllowCredentials, http::StatusCode,
    response::IntoResponse, TypedHeader,
};

use ::cookie::time::OffsetDateTime;
use axum_extra::extract::{
    cookie::{self, Cookie},
    CookieJar, PrivateCookieJar,
};
use diesel::{dsl::IntervalDsl, ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use hyper::{header::SET_COOKIE, HeaderMap};
use rand::distributions::{Alphanumeric, DistString};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    config::CONFIG, db::models::User, extractors::Json, pass::hash_password_and_generate_salt,
    schema::users, state::AppState,
};

/// tag used for routes in this module
static TAG: &str = "account";

pub fn routes(state: AppState) -> ApiRouter {
    infer_responses(true);
    ApiRouter::new()
        .api_route(
            "/register",
            post_with(register, |docs| {
                docs.response_with::<500, (), _>(|docs| {
                    docs.description("cannot create a user who is like this?!")
                })
                .response_with::<503, (), _>(|docs| {
                    docs.description("something went wrong with the database")
                })
                .response_with::<400, (), _>(|docs| {
                    docs.description("check if your username and password fit the requirements")
                })
                .description(
                    "use this if you want a new account. \
                it tries to create a new user with the password + username you gave. \
                by default unapproved users have no permission to access any locked api routes\
                or the login route",
                )
                .tag(TAG)
                .id("register")
                .summary("apply for account")
            }),
        )
        .api_route(
            "/login",
            post_with(login, |docs| {
                docs.tag(TAG)
                    .id("login")
                    .description(
                        "provide username and password to get a session token. keep it safe!",
                    )
                    .summary("log in")
                    .response_with::<200, (), _>(|docs| docs.description("session token has been set"))
                    .response_with::<503, (), _>(|docs| docs.description("database did a fucky wucky"))
                    .response_with::<404, (), _>(|docs| docs.description("this user does not exist"))
                    .response_with::<418, (), _>(|docs| docs.description("your application has not been processed yet! wait until the moderators make their decision"))
                    .response_with::<401, (), _>(|docs| docs.description("you're not allowed!!! :PPPPP"))
            }),
        )
        .with_state(state)
}

/// data needed to sign up for an account
#[derive(Deserialize, JsonSchema, Serialize)]
pub struct RegisterData {
    /// username to use, you can change this later. this is used for logging in.
    pub username: String,
    /// password to use. it has to be
    /// at least as long as the password set in the
    /// server config.
    pub password: String,
    /// give a bit of context as to why you want to join.
    /// links to social media and a little bit about youself can provide
    /// good reasons for an admin to let you join.
    pub join_reason: Option<String>,
}

async fn register(
    State(state): State<AppState>,
    Json(form): Json<RegisterData>,
) -> impl IntoApiResponse {
    if form.password.chars().count() < CONFIG.server.min_password_size as usize
        || form.username.chars().count() < CONFIG.server.min_username_length as usize
    {
        return Err(StatusCode::BAD_REQUEST);
    }

    // get db connection
    let mut conn = match state.pool.get().await {
        Err(e) => {
            tracing::log::error!("{}", e);
            return Err(StatusCode::SERVICE_UNAVAILABLE);
        }
        Ok(v) => v,
    };

    // hash and salt password
    let hash_and_salt = match hash_password_and_generate_salt(&form.password) {
        Err(e) => {
            tracing::log::error!("{}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        Ok(v) => v,
    };

    // generate user
    let user = User {
        id: Uuid::now_v7(),
        powerlevel: crate::db::models::Powerlevel::Unapproved,
        username: form.username,
        display_name: None,
        signup_reason: form.join_reason,
        pronoun: None,
        bio: None,
        hash: hash_and_salt.hash,
        salt: hash_and_salt.salt,
    };

    // try storing user in db
    let result = match diesel::insert_into(users::table)
        .values(&user)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await
    {
        Err(e) => {
            tracing::log::error!("{}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        Ok(v) => v,
    };

    Ok(Json(result))
}

/// data used to log a user in
#[derive(Deserialize, JsonSchema, Serialize)]
pub struct LoginData {
    /// username for the account you want to log in
    pub username: String,
    /// password for the user you want to log in
    pub password: String,
    /**
     * description for a session.
     * clients can set this to whatever they want,
     * or let the user set it themselves.
     *
     * something like time and client name can be useful.
     * just make sure they are human readable!
     */
    pub description: Option<String>,
}

async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(login_data): Json<LoginData>,
) -> impl IntoApiResponse {
    let mut conn = state
        .pool
        .get()
        .await
        .or(Err(StatusCode::SERVICE_UNAVAILABLE))?;

    // get user with this username from the database
    let known_user: User = users
        .filter(username.eq(login_data.username))
        .select(User::as_select())
        .first(&mut conn)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;

    // unapproved user should not be able to log in
    if known_user.powerlevel == Powerlevel::Unapproved {
        return Err(StatusCode::IM_A_TEAPOT);
    }

    // check if the hash is the same as the one stored in the database
    if hash_password(&login_data.password, &known_user.salt)
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?
        != known_user.hash
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // try creating a new session for this user
    let session = Session {
        description: login_data.description,
        user_id: known_user.id,
        secret: Alphanumeric.sample_string(&mut rand::thread_rng(), 64),
        expires_at: std::time::SystemTime::now()
            .checked_add(Duration::from_secs(
                604800, // this is how many seconds there are in a week
            ))
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?,
    };

    diesel::insert_into(sessions::table)
        .values(&session)
        .execute(&mut conn)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    let cookie = Cookie::build("SessionID", session.secret)
        .path("/")
        .secure(true)
        .expires(OffsetDateTime::now_utc().checked_add(time::Duration::days(7)))
        .http_only(true);

    let cookie = cookie.same_site(axum_extra::extract::cookie::SameSite::Strict);

    #[cfg(debug_assertions)]
    let cookie = cookie.same_site(axum_extra::extract::cookie::SameSite::None);

    let cookie = cookie.finish();
    tracing::error!("{:#?}", cookie);

    let updated_jar = jar.add(cookie);

    // return the secret
    Ok((updated_jar, StatusCode::OK))
}

#[cfg(test)]
mod test {
    use hyper::{Body, Request, StatusCode};
    use tower::ServiceExt;

    use crate::{api::v1::account::LoginData, generate_server};

    #[tokio::test]
    async fn register_empty_pass() {
        let b = serde_json::json!(LoginData {
            username: String::from("meowmeowmewmeow"),
            password: String::new(),
            description: None
        })
        .to_string();

        let app = generate_server().await.unwrap();
        let result = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/account/register")
                    .body(Body::from(b))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(result.status(), StatusCode::BAD_REQUEST);
    }
}
