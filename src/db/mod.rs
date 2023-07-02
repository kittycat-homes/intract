use std::error::Error;

use crate::config::CONFIG;
use diesel::{Connection, PgConnection};
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tracing::info;

pub mod models;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

/// establishes as synchronous db connection
/// and runs migrations on it
pub fn run_migrations() -> Result<(), Box<dyn Error>> {
    let mut connection = PgConnection::establish(&CONFIG.database.db_url)?;
    info!("running migrations");
    connection
        .run_pending_migrations(MIGRATIONS)
        .unwrap_or_else(|_| panic!("Error connecting to: {}", CONFIG.database.db_url));
    Ok(())
}

pub type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

/// get pool of async db connections
pub async fn get_pool() -> Result<Pool, Box<dyn Error>> {
    let db_url = &CONFIG.database.db_url;
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
    let pool = bb8::Pool::builder().build(config).await?;
    Ok(pool)
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[tokio::test]
    async fn connect_to_db() {
        assert!(get_pool().await.unwrap().get().await.is_ok())
    }

    #[test]
    fn test_migrations() {
        assert!(run_migrations().is_ok())
    }
}