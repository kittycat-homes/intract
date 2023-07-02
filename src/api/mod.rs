use aide::axum::ApiRouter;

use crate::state::AppState;

pub mod v1;

pub fn routes(state: AppState) -> ApiRouter {
    aide::gen::infer_responses(true);
    
    ApiRouter::new()
        .nest_api_service("/v1", v1::routes(state.clone()))
        .with_state(state)
}
