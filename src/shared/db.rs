use sea_orm::{ConnectOptions, DatabaseConnection};
use std::time::Duration;

pub struct DB;

impl DB {
    // Connect to sea-orm DatabaseConnection using SETTINGS.db_path as the database to connect too
    pub async fn connect() -> Result<DatabaseConnection, anyhow::Error> {
        let mut opt: ConnectOptions = ConnectOptions::new("postgres://shop:shop@localhost:5483/testrust".to_string());
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(false);

        let pool: DatabaseConnection = sea_orm::Database::connect(opt).await?;

        Ok(pool)
    }
}
