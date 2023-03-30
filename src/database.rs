use sqlx::{Error, postgres::{PgPool, PgConnectOptions}};

use crate::constants;

pub async fn create_conn_pool() -> Result<PgPool, Error> {
    // https://www.postgresql.org/docs/current/libpq-envars.html
    // https://docs.rs/sqlx/latest/sqlx/postgres/struct.PgConnectOptions.html
    // options is filled using PGUSER, PGPASSWORD, PGDATABASE and PGHOST environment variables
    // you can set these easily by modifying .env
    let options = PgConnectOptions::new();
    let pool = match PgPool::connect_with(options).await {
        Ok(pool) => pool,
        Err(_) => PgPool::connect(constants::DB_URL).await?
    };
    sqlx::migrate!().run(&pool).await?;
    Ok(pool)
}
