use crate::core::Installation;
use crate::AppState;
use tauri::State;
use crate::detection::GameDetector;

#[tauri::command]
pub async fn get_installations(state: State<'_, AppState>) -> Result<Vec<Installation>, String> {
    crate::storage::get_all_installations(&state.db)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn scan_library(state: State<'_, AppState>) -> Result<usize, String> {
    let detector = crate::detection::windows::WindowsGameDetector;
    let discovered = detector.discover_installations();

    let known = crate::storage::get_all_installations(&state.db)
        .await
        .map_err(|e| e.to_string())?;

    let new_installations = crate::core::filter_new_installations(&discovered, &known);

    for installation in &new_installations {
        crate::storage::insert_installation(&state.db, installation)
            .await
            .map_err(|e| e.to_string())?;
    }

    Ok(new_installations.len())
}