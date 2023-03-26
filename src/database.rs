use sqlx::{Error, postgres::{PgPool, PgConnectOptions}};

use crate::constants;

pub async fn create_conn_pool() -> Result<PgPool, Error> {
    // https://www.postgresql.org/docs/current/libpq-envars.html
    // https://docs.rs/sqlx/latest/sqlx/postgres/struct.PgConnectOptions.html
    let options = PgConnectOptions::new();
    let pool = match PgPool::connect_with(options).await {
        Ok(pool) => pool,
        Err(_) => PgPool::connect(constants::DB_URL).await?
    };
    sqlx::migrate!().run(&pool).await?;
    Ok(pool)
}
