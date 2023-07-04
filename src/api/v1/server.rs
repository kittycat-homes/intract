use aide::axum::{routing::get_with, ApiRouter};
use schemars::JsonSchema;
use serde::Serialize;

use crate::{config::CONFIG, extractors::Json, state::AppState};

pub fn routes(state: AppState) -> ApiRouter {
    ApiRouter::new().api_route(
        "/info",
        get_with(info, |docs| {
            docs.description("get info about the server")
                .tag("server")
                .id("server_info")
                .summary("server info")
                .description(
                    "this is for getting information about the server. \
                instances will be configured differently and these are the configurable \
                parameters that are important for clients.",
                )
        })
        .with_state(state),
    )
}

#[derive(JsonSchema, Serialize)]
struct ServerInfo {
    /// lowest length a password can be
    min_password_length: u8,
    /// the url the server is running on,
    /// something like `https://example.com`
    url: String,
    /// name of the server, this can be anything
    name: String,
}

async fn info() -> Json<ServerInfo> {
    Json(ServerInfo {
        min_password_length: CONFIG.server.min_password_size,
        url: CONFIG.server.url.clone(),
        name: CONFIG.server.name.clone(),
    })
}
