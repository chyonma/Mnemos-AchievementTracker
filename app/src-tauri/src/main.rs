mod core;
mod storage;
mod bridge;
mod detection;

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
        let active_sessions: Arc<Mutex<HashMap<String, core::Session>>> = Arc::new(Mutex::new(HashMap::new()));
        let polling_pool = pool.clone();
        let polling_sessions = active_sessions.clone();
        tokio::spawn(async move {
            let detector = detection::windows::WindowsGameDetector;
        
            loop {
                tokio::time::sleep(Duration::from_secs(5)).await;
        
                let running = detector.get_running_processes();
                let known_installations = match storage::get_all_installations(&polling_pool).await {
                    Ok(list) => list,
                    Err(e) => { eprintln!("Polling: failed to fetch installations: {}", e); continue; }
                };
                let matched = core::match_running_installations(&running, &known_installations);
                let now = chrono::Utc::now().to_rfc3339();
                let mut sessions = polling_sessions.lock().await;
                for installation in &matched {
                    if let Some(session) = sessions.get_mut(&installation.id) {
                        session.last_heartbeat(now.clone());
                        let _ = storage::update_heartbeat(&polling_pool, &session.id, &now).await;
                    } else {
                        let mut new_session = core::Session::new(installation.id.clone(), now.clone());
                        new_session.last_heartbeat(now.clone());
                        if storage::insert_session(&polling_pool, &new_session).await.is_ok() {
                            sessions.insert(installation.id.clone(), new_session);
                        }
                    }
                }
                let matched_ids: Vec<String> = matched.iter().map(|i| i.id.clone()).collect();
        let stopped_ids: Vec<String> = sessions.keys().filter(|id| !matched_ids.contains(id)).cloned().collect();

        for id in stopped_ids {
            if let Some(session) = sessions.remove(&id) {
                let start = chrono::DateTime::parse_from_rfc3339(&session.started_at).expect("bad timestamp");
                let end = chrono::DateTime::parse_from_rfc3339(&now).expect("bad timestamp");
                let duration = (end - start).num_seconds().max(0);
                let _ = storage::end_session(&polling_pool, &session.id, &now, duration).await;
            }
        }
    }
});

    tauri::Builder::default()
        .manage(AppState { db: pool })
        .invoke_handler(tauri::generate_handler![bridge::get_installations])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}