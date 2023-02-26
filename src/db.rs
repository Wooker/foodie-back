use r2d2;
use std::env;
use lazy_static::lazy_static;
use diesel::{pg::PgConnection, r2d2::ConnectionManager};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::errors::CustomError;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

lazy_static! {
    static ref POOL: Pool = {
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}
pub fn init() -> Result<(), CustomError> {
    lazy_static::initialize(&POOL);
    let mut conn = connection()?;
    conn.run_pending_migrations(MIGRATIONS).expect("Could not run migrations");

    Ok(())
}

pub fn connection() -> Result<DbConnection, CustomError> {
    Ok(POOL.get()?)
}
