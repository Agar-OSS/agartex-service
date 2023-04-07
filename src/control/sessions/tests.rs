use mockall::predicate;
use sqlx::types::chrono::Utc;

use crate::{service::sessions::MockSessionService, domain::{sessions::Session, users::User}};

use super::*;

fn mock_email() -> String {
    String::from("email")
}

fn mock_password() -> String {
    String::from("password")
}

fn mock_credentials() -> Credentials {
    Credentials {
        email: mock_email(),
        password: mock_password()
    }
}

fn mock_session_id() -> String {
    String::from("session_id")
}

fn mock_user() -> User {
    User {
        id: 1, 
        email: mock_email(),
        password_hash: mock_password()
    }
}

fn mock_session() -> Session {
    Session {
        id: mock_session_id(),
        user: mock_user(),
        expires: Utc::now().timestamp()
    }
}

#[tokio::test]
async fn post_sessions_normal() {
    let mut session_service = MockSessionService::new();

    session_service
        .expect_login()
        .with(predicate::eq(mock_credentials()))
        .times(1)
        .returning(|_| Ok(mock_session()));

    let (jar, status) = post_sessions(Extension(session_service), CookieJar::new(), Json(mock_credentials())).await.unwrap();
    assert_eq!(StatusCode::CREATED, status);

    let cookie = jar.get(SESSION_COOKIE_NAME).unwrap();
    assert_eq!(mock_session().id, cookie.value());
    assert_eq!(mock_session().expires, cookie.expires().unwrap().datetime().unwrap().unix_timestamp());
    assert!(cookie.http_only().unwrap());
}

#[tokio::test]
async fn post_sessions_no_user_error() {
    let mut session_service = MockSessionService::new();

    session_service
        .expect_login()
        .with(predicate::eq(mock_credentials()))
        .times(1)
        .returning(|_| Err(LoginError::NoUser));

    assert_eq!(StatusCode::UNAUTHORIZED, post_sessions(Extension(session_service), CookieJar::new(), Json(mock_credentials())).await.err().unwrap())
}

#[tokio::test]
async fn post_sessions_unknown_error() {
    let mut session_service = MockSessionService::new();

    session_service
        .expect_login()
        .with(predicate::eq(mock_credentials()))
        .times(1)
        .returning(|_| Err(LoginError::Unknown));

    assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, post_sessions(Extension(session_service), CookieJar::new(), Json(mock_credentials())).await.err().unwrap())
}
