mod core;
mod storage;
mod bridge;
mod detection;

use sqlx::SqlitePool;

pub struct AppState {
    pub db: SqlitePool,
}

#[tokio::main]
async fn main() {
    let pool = storage::connect("sqlite://mnemos.db?mode=rwc")
        .await
        .expect("failed to connect to database");

    use detection::GameDetector;
    let detector = detection::windows::WindowsGameDetector;
    let running = detector.get_running_processes();

    let known_installations = storage::get_all_installations(&pool)
        .await
        .expect("failed to fetch installations");

    let matched = core::match_running_installations(&running, &known_installations);

    let now = chrono::Utc::now().to_rfc3339();
    let new_sessions = core::start_sessions_for(&matched, now);

    for session in &new_sessions {
        storage::insert_session(&pool, session)
            .await
            .expect("failed to insert session");
    }

    println!("Created and saved {} session(s)", new_sessions.len());
    for session in &new_sessions {
        println!("Session {} for installation {} started at {}", session.id, session.installation_id, session.started_at);
    }


    tauri::Builder::default()
        .manage(AppState { db: pool })
        .invoke_handler(tauri::generate_handler![bridge::get_installations])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}