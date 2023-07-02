use aide::axum::ApiRouter;

use crate::state::AppState;

mod account;
mod server;

/// for routes that require an authorized user with regular user permissions
mod authorized;

pub fn routes(state: AppState) -> ApiRouter {
    aide::gen::infer_responses(true);
    ApiRouter::new()
        .nest_api_service("/account", account::routes(state.clone()))
        .nest_api_service("/server", server::routes(state.clone()))
        .nest_api_service("/authorized",  authorized::routes(state.clone()))
        .with_state(state)
}
