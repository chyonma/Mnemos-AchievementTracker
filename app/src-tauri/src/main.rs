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

    println!("Known installations: {}", known_installations.len());
    println!("Currently running (matched): {}", matched.len());
    for installation in &matched {
        println!("{}", installation.display_name);
    }

    tauri::Builder::default()
        .manage(AppState { db: pool })
        .invoke_handler(tauri::generate_handler![bridge::get_installations])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}