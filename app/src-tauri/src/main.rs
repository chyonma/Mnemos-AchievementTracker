mod core;
mod storage;
mod bridge;
mod detection;

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



    tauri::Builder::default()
        .manage(AppState { db: pool })
        .invoke_handler(tauri::generate_handler![bridge::get_installations])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}