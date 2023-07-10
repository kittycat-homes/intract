use aide::axum::{
    routing::{delete_with, get_with},
    ApiRouter, IntoApiResponse,
};
use axum::{extract::State, Extension};
use axum_extra::extract::CookieJar;
use cookie::Cookie;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use hyper::StatusCode;

use crate::{
    db::models::{Session, User},
    extractors::Json,
    schema::sessions,
    state::AppState,
};

pub fn routes(state: AppState) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/whoami",
            get_with(whoami, |docs| {
                docs.id("whoami")
                    .tag("account")
                    .description(
                        "tells you information about your account. \
                        one useful thing you can do with this is checking if you are logged in.",
                    )
                    .summary("who am i???")
            }),
        )
        .api_route(
            "/logout",
            delete_with(logout, |docs| {
                docs.id("logout")
                    .response_with::<202, (), _>(|docs| {
                        docs.description(
                            "logged out successfully! \
                    your session was deleted from the db and \
                    your cookie was deleted from your browser",
                        )
                    })
                    .tag("account")
            }),
        )
        .with_state(state)
}

async fn whoami(Extension(current_user): Extension<User>) -> Json<User> {
    tracing::error!("{:#?}", current_user);
    Json(current_user)
}

async fn logout(
    state: State<AppState>,
    Extension(session): Extension<Session>,
    jar: CookieJar,
) -> Result<(StatusCode, CookieJar), StatusCode> {
    let mut conn = state
        .pool
        .get()
        .await
        .or(Err(StatusCode::SERVICE_UNAVAILABLE))?;

    diesel::delete(sessions::dsl::sessions.filter(sessions::dsl::secret.eq(session.secret)))
        .execute(&mut conn)
        .await
        .or(Err(StatusCode::SERVICE_UNAVAILABLE))?;

    let new_jar = jar.remove(Cookie::named("SessionID"));
    Ok((StatusCode::ACCEPTED, new_jar))
}
