use std::sync::Arc;

use crate::{config::CONFIG, extractors::Json, state::AppState};
use aide::{
    axum::{
        routing::{get, get_with},
        ApiRouter, IntoApiResponse,
    },
    openapi::{Contact, OpenApi, Server},
    redoc::Redoc,
    transform::TransformOpenApi,
};
use axum::{response::IntoResponse, Extension};
use clap::{crate_description, crate_name, crate_version};
use indexmap::IndexMap;
use serde_json::Value;

pub fn docs_routes(_state: AppState) -> ApiRouter {
    // look for api responses
    aide::gen::infer_responses(true);

    ApiRouter::new()
        .route(
            "/redoc",
            get(Redoc::new("/docs/openapi.json")
                .with_title("intract redoc")
                .axum_handler()),
        )
        .route("/openapi.json", get(serve_docs))
}

/// shows the openapi specification
async fn serve_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    Json(api).into_response()
}

/// add additional info to api docs
pub fn add_api_docs(api: TransformOpenApi) -> TransformOpenApi {
    let security_extensions: IndexMap<String, Value> = IndexMap::new();

    api.title(crate_name!())
        .contact(Contact {
            url: Some(String::from("https://github.com/kittycat-homes/intract")),
            ..Default::default()
        })
        .version(crate_version!())
        .description(crate_description!())
        .server(Server {
            url: CONFIG.server.url.clone(),
            ..Default::default()
        })
        .security_scheme(
            "ApiKey",
            aide::openapi::SecurityScheme::ApiKey {
                location: aide::openapi::ApiKeyLocation::Header,
                name: "Key".into(),
                description: Some("an api key for session auth".into()),
                extensions: security_extensions,
            },
        )
}
