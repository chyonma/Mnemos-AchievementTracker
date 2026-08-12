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

    let brave_installation = core::Installation::new(
        "C:\\Program Files\\BraveSoftware\\Brave-Browser\\Application\\brave.exe".to_string(),
        "brave.exe".to_string(),
        "C:\\Program Files\\BraveSoftware\\Brave-Browser\\Application".to_string(),
        "Brave Browser (test)".to_string(),
    );
    storage::insert_installation(&pool, &brave_installation)
        .await
        .expect("failed to insert test installation");

    use detection::GameDetector;
    let detector = detection::windows::WindowsDetector;
    let running = detector.get_running_processes();

    let known_installations = storage::get_all_installations(&pool)
        .await
        .expect("failed to fetch installations");

    let matched = core::match_running_installations(&running, &known_installations);

    tauri::Builder::default()
        .manage(AppState { db: pool })
        .invoke_handler(tauri::generate_handler![bridge::get_installations])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}