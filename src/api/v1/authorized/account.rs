use aide::axum::{routing::get_with, ApiRouter};
use axum::Extension;

use crate::{db::models::User, extractors::Json, state::AppState};

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
        .with_state(state)
}

async fn whoami(Extension(current_user): Extension<User>) -> Json<User> {
    tracing::error!("{:#?}", current_user);
    Json(current_user)
}
