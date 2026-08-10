mod core;
mod storage;

use sqlx::SqlitePool;
pub struct AppState {

  pub db: SqlitePool,
}

#[tokio::main]
async fn main(){
  let pool = storage::connect("sqlite://mnemos.db?mode=rwc")
      .await
      .expect("failed to connect to database");

      tauri::Builder::default()
      .manage(AppState{ db: pool})
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}