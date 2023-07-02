use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use diesel::prelude::*;

use crate::{
    db::models::{Powerlevel, User},
    schema::{
        sessions::{self, secret},
        users,
    },
    state::AppState,
};
use diesel_async::RunQueryDsl;

/// makes sure that you need at least a powerlevel of user to access these routes
pub async fn guard_user<T>(
    State(state): State<AppState>,
    mut request: Request<T>,
    next: Next<T>,
) -> Result<Response, StatusCode> {
    tracing::debug!("guard middleware triggered!");
    let token = request
        .headers()
        .get("Key")
        .ok_or(StatusCode::UNAUTHORIZED)?
        .to_str()
        .or(Err(StatusCode::UNAUTHORIZED))?;

    let mut conn = state
        .pool
        .get()
        .await
        .or(Err(StatusCode::SERVICE_UNAVAILABLE))?;

    let user: User = sessions::table
        .select(secret.eq(token))
        .inner_join(users::table)
        .select(User::as_select())
        .first::<User>(&mut conn)
        .await
        .or(Err(StatusCode::UNAUTHORIZED))?;

    if user.powerlevel < Powerlevel::User {
        return Err(StatusCode::UNAUTHORIZED);
    }

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
