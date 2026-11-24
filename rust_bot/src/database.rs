use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

/// Optional Postgres-backed storage. If DATABASE_URL is unset, the bot may run
/// in stateless mode while features requiring persistence report a clear error.
#[derive(Clone)]
pub struct Database { pool: PgPool }

impl Database {
    pub async fn connect(url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(10))
            .connect(url)
            .await?;
        sqlx::migrate!("./migrations").run(&pool).await
            .map_err(|e| sqlx::Error::Migrate(Box::new(e)))?;
        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool { &self.pool }

    pub async fn ping(&self) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }

    pub async fn ensure_guild(&self, guild_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO guild_settings (guild_id) VALUES ($1) ON CONFLICT (guild_id) DO NOTHING")
            .bind(guild_id).execute(&self.pool).await?;
        Ok(())
    }
}
