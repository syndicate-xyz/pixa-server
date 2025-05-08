use sea_orm::{Database, DatabaseConnection, DbErr};
use sqlx::{database, migrate::Migrator};
use std::sync::OnceLock;
use tokio::sync::OnceCell;
use utils::ENV_CONFIG;

pub mod tg_user;

static DB_CONN: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn get_db() -> &'static DatabaseConnection {
    DB_CONN
        .get_or_init(|| async {
            Database::connect(ENV_CONFIG.database_url.clone())
                .await
                .expect("Failed to connect to database")
        })
        .await
}
