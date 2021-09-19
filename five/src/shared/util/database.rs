use crate::shared::util::config::DatabaseConfig;
use sqlx::{postgres::PgPoolOptions, Error, Pool, Postgres};
use std::borrow::Borrow;

#[derive(Clone)]
pub struct PostgresDatabaseContext {
    pub database_pool: Pool<Postgres>,
}

impl PostgresDatabaseContext {
    pub async fn new(database_config: DatabaseConfig) -> Result<PostgresDatabaseContext, Error> {
        let database_url =
            generate_postgres_database_addr_based_on_config(database_config.clone());
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url.borrow())
            .await?;

        Ok(PostgresDatabaseContext {
            database_pool: pool,
        })
    }
}

fn generate_postgres_database_addr_based_on_config(postgres_database_config: DatabaseConfig) -> String {
    format!(
        "postgres://{}:{}@{}:{}/{}",
        postgres_database_config.username,
        postgres_database_config.password,
        postgres_database_config.hostname,
        postgres_database_config.port,
        postgres_database_config.database_name
    )
}

pub trait PostgresRepository{}

pub struct StoredProcedureInfo{}
