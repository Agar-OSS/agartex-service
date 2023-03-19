use anyhow::Result;
use axum::async_trait;
use axum_database_sessions::{SessionStore, SessionPgPool, SessionConfig, SessionMode};
use axum_sessions_auth::{AuthConfig, Authentication};
use sqlx::{PgPool, query_as};

use crate::{constants, domain::users::User};

#[derive(Clone, Debug)]
pub enum AuthUser {
    Guest,
    Known(User)
}

#[async_trait]
impl Authentication<Self, i32, PgPool> for AuthUser {
    async fn load_user(userid: i32, pool: Option<&PgPool>) -> Result<Self> {
        let pool = match pool {
            Some(pool) => pool,
            None => return Err(anyhow::Error::msg("Can't authenticate user: no connection to database!"))
        };

        let result = query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(userid)
            .fetch_optional(pool)
            .await;
        match result {
            Ok(Some(user)) => Ok(Self::Known(user)),
            Ok(None) => Ok(Self::Guest),
            Err(err) => Err(err.into())
        }
    }

    fn is_authenticated(&self) -> bool {
        match self {
            Self::Guest => false,
            Self::Known(_) => true
        }
    }

    fn is_active(&self) -> bool {
        self.is_authenticated()
    }

    fn is_anonymous(&self) -> bool {
        !self.is_authenticated()
    }
}

pub async fn auth_setup(pool: &PgPool) -> (AuthConfig<i32>, SessionStore<SessionPgPool>) {
    let session_config = SessionConfig::default()
        .with_http_only(true)
        // .with_secure(true)
        .with_table_name(constants::SESSIONS_TABLE)
        .with_cookie_name(constants::SESSION_ID)
        .with_mode(SessionMode::Storable);

    let auth_config = AuthConfig::default().set_cache(true);

    let session_store = SessionStore::<SessionPgPool>::new(Some(pool.clone().into()), session_config);
    session_store.initiate().await.unwrap();
    session_store.cleanup().await.unwrap();

    (auth_config, session_store)
}