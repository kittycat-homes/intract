use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;
use diesel::prelude::*;

use crate::{
    db::models::{Powerlevel, Session, User},
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
    jar: CookieJar,
    mut request: Request<T>,
    next: Next<T>,
) -> Result<Response, StatusCode> {
    tracing::debug!("guard middleware triggered!");
    let token = jar
        .get("SessionID")
        .ok_or(StatusCode::UNAUTHORIZED)?
        .value();

    let mut conn = state
        .pool
        .get()
        .await
        .or(Err(StatusCode::SERVICE_UNAVAILABLE))?;

    let user: User = sessions::table
        .filter(secret.eq(token))
        .inner_join(users::table)
        .select(User::as_select())
        .first::<User>(&mut conn)
        .await
        .or(Err(StatusCode::UNAUTHORIZED))?;

    if user.powerlevel < Powerlevel::User {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let session: Session = sessions::table
        .select(Session::as_select())
        .first(&mut conn)
        .await
        .or(Err(StatusCode::UNAUTHORIZED))?;

    request.extensions_mut().insert(user);
    request.extensions_mut().insert(session);

    Ok(next.run(request).await)
}
