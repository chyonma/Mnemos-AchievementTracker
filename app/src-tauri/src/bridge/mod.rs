use crate::core::Installation;
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn get_installations(state: State<'_, AppState>) -> Result<Vec<Installation>, String> {
    crate::storage::get_all_installations(&state.db)
        .await
        .map_err(|e| e.to_string())
}