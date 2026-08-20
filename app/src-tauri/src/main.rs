mod core;
mod storage;
mod bridge;
mod detection;
mod providers; // <-- ADD THIS

use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::Duration;

use crate::detection::GameDetector;

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

    dotenvy::dotenv().ok();
    let steam_api_key = std::env::var("STEAM_API_KEY").expect("STEAM_API_KEY not set in .env");
    let steam_id = std::env::var("STEAM_ID").expect("STEAM_ID not set in .env");
    let steam = providers::steam::SteamProvider::new(steam_api_key, steam_id);

    use providers::AchievementProvider;
    let test_app_id = "1245620"; // Elden Ring

    match steam.fetch_achievements(test_app_id).await {
        Ok(data) => {
            let record_id = storage::get_or_create_provider_game_record(&pool, "steam", test_app_id, "Elden Ring")
                .await.expect("failed to get/create provider game record");
            storage::save_provider_achievements(&pool, &record_id, &data)
                .await.expect("failed to save achievements");

            let unlocked_count = data.iter().filter(|d| d.unlock.unlocked_at.is_some()).count();
            println!("Synced {} total, {} unlocked ({:.0}%)", data.len(), unlocked_count,
                (unlocked_count as f64 / data.len() as f64) * 100.0);
        }
        Err(e) => println!("Steam fetch failed: {}", e),
    }

    let active_sessions: Arc<Mutex<HashMap<String, core::Session>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let polling_pool = pool.clone();
    let polling_sessions = active_sessions.clone();
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
            bridge::get_achievements_for_installation
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}