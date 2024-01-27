use std::path::Path;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};

use crate::Error;

pub mod create_thing;
pub mod delete_thing;
pub mod list_things;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(filename: impl AsRef<Path>) -> Result<Self, Error> {
        let options = SqliteConnectOptions::new()
            .filename(filename)
            .create_if_missing(true);

        let pool = SqlitePool::connect_with(options).await?;

        Ok(Self { pool })
    }

    pub async fn migrate(&self) -> Result<(), Error> {
        sqlx::migrate!().run(&self.pool).await?;

        Ok(())
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct Thing {
    pub id: i64,
}