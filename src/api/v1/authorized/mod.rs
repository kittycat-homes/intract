use aide::axum::ApiRouter;
use axum::middleware;

use crate::{middleware::guard::guard_user, state::AppState};

mod account;
mod feeds;

pub fn routes(state: AppState) -> ApiRouter {
    ApiRouter::new()
        .nest_api_service("/account", account::routes(state.clone()))
        .nest_api_service("/feed", feeds::routes(state.clone()))
        .layer(middleware::from_fn_with_state(state.clone(), guard_user))
        .with_state(state)
        .with_path_items(|docs| docs.security_requirement("SessionID"))
}
