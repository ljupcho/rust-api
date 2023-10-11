use sea_orm::{ConnectOptions, DatabaseConnection};
use std::time::Duration;
use std::env;

pub struct DB;

impl DB {
    // Connect to sea-orm DatabaseConnection using SETTINGS.db_path as the database to connect too
    pub async fn connect() -> Result<DatabaseConnection, anyhow::Error> {
        let host = env::var("DB_HOST").expect("missing db host");
        let user = env::var("DB_USER").expect("missing db user");
        let password = env::var("DB_PASSWORD").expect("missing db password");
        let db_name = env::var("DB_NAME").expect("missing db name");
        let port = env::var("DB_PORT").expect("missing db port");

        let connection_string = format!("postgres://{0}:{1}@{2}:{3}/{4}", user, password, host, port, db_name);

        let mut opt: ConnectOptions = ConnectOptions::new(connection_string.to_string());
        opt.max_connections(25)
            .min_connections(25)
            .connect_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(false);

        let pool: DatabaseConnection = sea_orm::Database::connect(opt).await?;

        Ok(pool)
    }
}
