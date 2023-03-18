use sqlx::{Error, postgres::PgPool};

use crate::constants::ParsedConfig;

pub async fn get_conn_pool(config: &ParsedConfig) -> Result<PgPool, Error> {
    let pool = PgPool::connect(config.db).await?;
    sqlx::migrate!().run(&pool).await?;
    Ok(pool)
}