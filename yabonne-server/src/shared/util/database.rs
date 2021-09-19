use crate::shared::util::config::DatabaseConfig;
use sqlx::{postgres::PgPoolOptions, Error, Pool, Postgres};
use std::borrow::Borrow;

#[derive(Clone)]
pub struct DatabaseContext {
    pub database_pool: Pool<Postgres>,
}

impl DatabaseContext {
    pub async fn new(database_config: DatabaseConfig) -> Result<DatabaseContext, Error> {
        let database_url =
            generate_database_addr_based_on_config(database_config.clone());
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url.borrow())
            .await?;

        Ok(DatabaseContext {
            database_pool: pool,
        })
    }
}

fn generate_database_addr_based_on_config(database_config: DatabaseConfig) -> String {
    format!(
        "postgres://{}:{}@{}:{}/{}",
        database_config.username,
        database_config.password,
        database_config.hostname,
        database_config.port,
        database_config.database_name
    )
}
