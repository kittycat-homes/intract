use std::sync::Arc;

pub type AppState = Arc<InnerAppState>;

#[derive(Clone)]
pub struct InnerAppState {
    /// pool of database connections
    pub pool: crate::db::Pool,
}
