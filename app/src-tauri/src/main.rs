mod core;
mod storage;
mod bridge;
mod detection;
mod providers;
mod credentials;

use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::Duration;


pub struct AppState {
    pub db: SqlitePool,
}


#[tokio::main]
async fn main() {
    let pool = storage::connect("sqlite://mnemos.db?mode=rwc")
        .await
        .expect("failed to connect to database");

    let orphaned = storage::get_orphaned_sessions(&pool)
        .await
        .expect("failed to fetch orphaned sessions");

    for (session_id, _installation_id, started_at, last_heartbeat) in &orphaned {
        let (ended_at, duration) = core::recover_orphaned_session(started_at, last_heartbeat);
        storage::end_session(&pool, session_id, &ended_at, duration)
            .await
            .expect("failed to close orphaned session");
    }

    if !orphaned.is_empty() {
        println!("Recovered {} orphaned session(s) from a previous run", orphaned.len());
    }

    let active_sessions: Arc<Mutex<HashMap<String, core::Session>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let _polling_pool = pool.clone();
    let _polling_sessions = active_sessions.clone();
    tokio::spawn(async move {
        
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState { db: pool })
        .invoke_handler(tauri::generate_handler![
            bridge::get_installations,
            bridge::scan_library,
            bridge::add_watched_folder,
            bridge::get_watched_folders,
            bridge::get_achievements_for_installation,
            bridge::add_installation_manually,
            bridge::sync_achievements_for_installation, 
            bridge::save_steam_credentials,             
            bridge::get_steam_credentials_status       
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}