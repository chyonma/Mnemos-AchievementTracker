use crate::core::Installation;
use crate::core::Session;
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
pub async fn insert_session(pool: &SqlitePool, session: &Session) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO sessions
        (id, installation_id, started_at, ended_at, duration_seconds, manually_edited)
        VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&session.id)
    .bind(&session.installation_id)
    .bind(&session.started_at)
    .bind(&session.ended_at)
    .bind(session.duration_seconds.map(|d| d as i64))
    .bind(session.manually_edited)
    .bind(&session.last_heartbeat)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_sessions_for_installation(
    pool: &SqlitePool,
    installation_id: &str,
) -> Result<Vec<Session>, sqlx::Error> {
    let rows = sqlx::query_as::<_, SessionRow>(
        "SELECT * FROM sessions WHERE installation_id = ?"
    )
    .bind(installation_id)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| r.into()).collect())
}

#[derive(sqlx::FromRow)]
struct SessionRow {
    id: String,
    installation_id: String,
    started_at: String,
    ended_at: Option<String>,
    duration_seconds: Option<i64>,
    manually_edited: bool,
    last_heartbeat: Option<String>,
}

impl From<SessionRow> for Session {
    fn from(row: SessionRow) -> Self {
        Session {
            id: row.id,
            installation_id: row.installation_id,
            started_at: row.started_at,
            ended_at: row.ended_at,
            duration_seconds: row.duration_seconds.map(|d| d as u64),
            manually_edited: row.manually_edited,
            last_heartbeat: row.last_heartbeat,
        }
    }
}
pub async fn update_heartbeat(pool: &SqlitePool, session_id: &str, at: &str) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE sessions SET last_heartbeat = ? WHERE id = ?")
        .bind(at)
        .bind(session_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn end_session(pool: &SqlitePool, session_id: &str, ended_at: &str, duration_seconds: i64) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE sessions SET ended_at = ?, duration_seconds = ? WHERE id = ?")
        .bind(ended_at)
        .bind(duration_seconds)
        .bind(session_id)
        .execute(pool)
        .await?;
    Ok(())
}
#[derive(sqlx::FromRow)]
struct ActiveSessionRow {
    id: String,
    installation_id: String,
    started_at: String,
    last_heartbeat: Option<String>,
}

pub async fn get_orphaned_sessions(pool: &SqlitePool) -> Result<Vec<(String, String, String, Option<String>)>, sqlx::Error> {
    let rows = sqlx::query_as::<_, ActiveSessionRow>(
        "SELECT id, installation_id, started_at, last_heartbeat FROM sessions WHERE ended_at IS NULL"
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| (r.id, r.installation_id, r.started_at, r.last_heartbeat)).collect())
}

pub async fn insert_watched_folder(pool: &SqlitePool, path: &str) -> Result<(), sqlx::Error> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query("INSERT INTO watched_folders (id, path, created_at) VALUES (?, ?, ?)")
        .bind(&id)
        .bind(path)
        .bind(&now)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_watched_folders(pool: &SqlitePool) -> Result<Vec<String>, sqlx::Error> {
    let rows: Vec<(String,)> = sqlx::query_as("SELECT path FROM watched_folders")
        .fetch_all(pool)
        .await?;
    Ok(rows.into_iter().map(|(p,)| p).collect())
}


pub async fn update_installation_launcher(pool: &SqlitePool, installation_id: &str, known_launcher: &str) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE installations SET known_launcher = ?, updated_at = ? WHERE id = ?")
        .bind(known_launcher)
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(installation_id)
        .execute(pool)
        .await?;
    Ok(())
}