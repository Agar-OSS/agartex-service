use std::fmt::Debug;

use axum::{Extension, Json};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use cookie::time::OffsetDateTime;
use hyper::StatusCode;
use tracing::info;

use crate::{service::sessions::{SessionService, LoginError}, domain::users::Credentials, constants::SESSION_COOKIE_NAME};

#[tracing::instrument(skip_all, fields(email = credentials.email))]
pub async fn post_sessions<T: SessionService + Debug>(
    Extension(service): Extension<T>,
    jar: CookieJar,
    Json(credentials): Json<Credentials>
) -> Result<(CookieJar, StatusCode), StatusCode> {
    info!("Received login attempt");
    let session = match service.login(credentials).await {
        Err(LoginError::NoUser) =>  {
            return Err(StatusCode::UNAUTHORIZED);
        },
        Err(LoginError::Unknown) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        },
        Ok(session) => session
    };

    let cookie = Cookie::build(SESSION_COOKIE_NAME, session.id)
        .expires(OffsetDateTime::from_unix_timestamp(session.expires).unwrap())
        .http_only(true)
        // .secure(true) <-- add this when TLS is set up
        .finish();

    Ok((jar.add(cookie), StatusCode::CREATED))
}
