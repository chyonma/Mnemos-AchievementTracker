mod core;
mod storage;
mod bridge;
mod detection;
mod providers;

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

    dotenvy::dotenv().ok();
    let steam_api_key = std::env::var("STEAM_API_KEY").expect("STEAM_API_KEY not set in .env");
    let steam_id = std::env::var("STEAM_ID").expect("STEAM_ID not set in .env");
    let steam = providers::steam::SteamProvider::new(steam_api_key, steam_id);

    use providers::{AchievementDefinitionSource, AchievementUnlockSource};
    let test_app_id = "1245620"; // Elden Ring

    let definitions = steam.fetch_definitions(test_app_id).await;
    let unlocks = steam.fetch_unlocks(test_app_id).await;

    match (definitions, unlocks) {
        (Ok(defs), Ok(raw_unlocks)) => {
            let merged = core::merge_achievements(defs, &raw_unlocks);
            let record_id = storage::get_or_create_provider_game_record(&pool, "steam", test_app_id, "Elden Ring")
                .await.expect("failed to get/create provider game record");
            storage::save_provider_achievements(&pool, &record_id, &merged)
                .await.expect("failed to save achievements");

            let unlocked_count = merged.iter().filter(|(_, u)| u.is_some()).count();
            println!("Synced {} total, {} unlocked ({:.0}%)", merged.len(), unlocked_count,
                (unlocked_count as f64 / merged.len() as f64) * 100.0);
        }
        (Err(e), _) | (_, Err(e)) => println!("Steam fetch failed: {}", e),
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
            bridge::add_installation_manually
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}