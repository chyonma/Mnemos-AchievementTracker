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
    println!("Found {} running processes:", running.len());
    for path in running.iter().take(10) {
        println!("{}", path);
    }

    tauri::Builder::default()
        .manage(AppState { db: pool })
        .invoke_handler(tauri::generate_handler![bridge::get_installations])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}