use std::pin::Pin;

use hbb_common::ResultType;
use sqlx::{Pool, Postgres, Transaction, migrate::MigrateError};
use tracing_subscriber::registry::Data;

#[derive(Clone)]
pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = Pool::<Postgres>::connect_lazy(database_url)?;

        Ok(Database { pool })
    }

    pub async fn migrate(&self) -> Result<(), MigrateError> {
        sqlx::migrate!().run(&self.pool).await
    }

    pub async fn transaction<F, T>(&self, f: F) -> Result<T, sqlx::Error>
    where
        F: for<'c> FnOnce(
            &'c mut Transaction<'_, Postgres>,
        ) -> Pin<Box<dyn Future<Output = Result<T, sqlx::Error>> + 'c>>,
    {
        let mut tx = self.pool.begin().await?;
        let result = f(&mut tx).await?;
        tx.commit().await?;
        Ok(result)
    }

    pub async fn close(self) {
        self.pool.close().await;
    }
}


