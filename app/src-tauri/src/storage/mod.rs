use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

pub async fn connect(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .connect(database_url)
        .await?;

    Ok(pool)
}