use std::env;
use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
pub struct SQLDatabase;

impl SQLDatabase {
    pub async fn new() -> DatabaseConnection {
        dotenvy::dotenv().ok();
        let database_url = env::var("PSQL_DATABASE_URL")
            .expect("PSQL_DATABASE_URL must be set");
        let mut opts = ConnectOptions::new(database_url);
        opts.max_connections(5)
            .min_connections(2)
            .connect_timeout(Duration::from_secs(5))
            .acquire_timeout(Duration::from_secs(10))
            .idle_timeout(Duration::from_secs(10))
            .max_lifetime(Duration::from_secs(10))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info)
            .set_schema_search_path("public");
        Database::connect(opts).await.expect("Database connection failed")
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn check() {
    let db = SQLDatabase::new().await;
    assert!(db.ping().await.is_ok());
    db.clone().close().await.expect("Database close failed");
    assert!(
        matches!(db.ping().await, Err(sea_orm::DbErr::ConnectionAcquire(_))),
        "Expected ConnectionAcquire error, got something else"
    );
}