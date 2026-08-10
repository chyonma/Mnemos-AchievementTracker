use crate::core::Installation;
use chrono::Utc;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

pub async fn connect(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .connect(database_url)
        .await?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    Ok(pool)
}
pub async fn insert_installation(
    pool: &SqlitePool,
    installation: &Installation,
) -> Result<(), sqlx::Error> {
    let now = Utc::now().to_rfc3339();

    sqlx:: query(
        "INSERT INTO installations 
         (id, executable_path, executable_name, install_directory, display_name, known_launcher, steam_app_id, manually_linked, created_at, updated_at) 
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
         "
    )

    .bind(&installation.id)
    .bind(&installation.executable_path)
    .bind(&installation.executable_name)
    .bind(&installation.install_directory)
    .bind(&installation.display_name)
    .bind(&installation.known_launcher)
    .bind(&installation.steam_app_id)
    .bind(installation.manually_linked)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_all_installations(
    pool: &SqlitePool,
) -> Result<Vec<Installation>,sqlx::Error>{
    let rows = sqlx::query_as::<_, InstallationRow>("SELECT * FROM installations")
        .fetch_all(pool)
         .await?;

    Ok(rows.into_iter().map(|r| r.into()).collect())


}

#[derive(sqlx::FromRow)]
struct InstallationRow {
    id: String,
    executable_path: String,
    executable_name: String,
    install_directory: String,
    display_name: String,
    known_launcher: Option<String>,
    steam_app_id: Option<String>,
    manually_linked: bool,
}

impl From<InstallationRow> for Installation {
    fn from(row: InstallationRow) -> Self {
        Installation {
            id: row.id,
            executable_path: row.executable_path,
            executable_name: row.executable_name,
            install_directory: row.install_directory,
            display_name: row.display_name,
            known_launcher: row.known_launcher,
            steam_app_id: row.steam_app_id,
            manually_linked: row.manually_linked,
        }
    }
}