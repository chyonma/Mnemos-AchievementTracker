use crate::core::Installation;
use crate::AppState;
use tauri::State;
use crate::detection::GameDetector;
use sqlx;


async fn try_link_steam_installation(state: &State<'_, AppState>, installation: &crate::core::Installation) {
    let steamapps_dirs = [
        r"C:\Program Files (x86)\Steam\steamapps".to_string(),
        r"C:\Program Files\Steam\steamapps".to_string(),
    ];
    let mut app_id_map = std::collections::HashMap::new();
    for dir in &steamapps_dirs {
        app_id_map.extend(crate::providers::steam::resolve_app_ids(dir));
    }

    // Walk EVERY ancestor folder of the exe path
    let exe_path = std::path::Path::new(&installation.executable_path);
    let mut matched_app_id: Option<String> = None;

    for ancestor in exe_path.ancestors() {
        if let Some(name) = ancestor.file_name().and_then(|n| n.to_str()) {
            if let Some(app_id) = app_id_map.get(name) {
                matched_app_id = Some(app_id.clone());
                break;
            }
        }
    }

    let Some(app_id) = matched_app_id else { return }; // Not a Steam install, skip

    if let Ok(record_id) = crate::storage::get_or_create_provider_game_record(
        &state.db, "steam", &app_id, &installation.display_name
    ).await {
        let _ = crate::storage::link_installation_to_provider_game(&state.db, &installation.id, &record_id).await;
    }
}



#[tauri::command]
pub async fn get_installations(state: State<'_, AppState>) -> Result<Vec<Installation>, String> {
    crate::storage::get_all_installations(&state.db)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_watched_folder(state: State<'_, AppState>, path: String) -> Result<(), String> {
    crate::storage::insert_watched_folder(&state.db, &path)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_watched_folders(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    crate::storage::get_watched_folders(&state.db)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn scan_library(state: State<'_, AppState>) -> Result<usize, String> {
    let detector = crate::detection::windows::WindowsGameDetector;

    let mut roots: Vec<(String, Option<String>)> = vec![
        (r"C:\Program Files (x86)\Steam\steamapps\common".to_string(), Some("steam".to_string())),
        (r"C:\Program Files\Steam\steamapps\common".to_string(), Some("steam".to_string())),
    ];

    let watched = crate::storage::get_watched_folders(&state.db)
        .await
        .map_err(|e| e.to_string())?;
    roots.extend(watched.into_iter().map(|p| (p, None)));

    let discovered = detector.discover_installations(&roots);

    let known = crate::storage::get_all_installations(&state.db)
        .await
        .map_err(|e| e.to_string())?;

    let reconciliation = crate::core::reconcile_discovered_installations(&discovered, &known);

    for installation in &reconciliation.new {
        crate::storage::insert_installation(&state.db, installation)
            .await
            .map_err(|e| e.to_string())?;
    }

    for installation in &reconciliation.updated {
        if let Some(launcher) = &installation.known_launcher {
            crate::storage::update_installation_launcher(&state.db, &installation.id, launcher)
                .await
                .map_err(|e| e.to_string())?;
        }
    }

    for installation in &reconciliation.new {
        try_link_steam_installation(&state, installation).await;
    }

    Ok(reconciliation.new.len())
}


#[tauri::command]
pub async fn add_installation_manually(
    state: State<'_, AppState>,
    executable_path: String,
) -> Result<(), String> {
    let path = std::path::Path::new(&executable_path);
    let executable_name = path.file_name()
        .and_then(|n| n.to_str())
        .ok_or("Invalid executable path")?
        .to_string();
    let install_directory = path.parent()
        .and_then(|p| p.to_str())
        .ok_or("Invalid executable path")?
        .to_string();
    let display_name = std::path::Path::new(&install_directory)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(&executable_name)
        .to_string();

    let mut installation = Installation::new(
        executable_path.clone(),
        executable_name,
        install_directory.clone(),
        display_name,
    );

    
    if executable_path.to_lowercase().contains("steamapps") {
        installation.known_launcher = Some("steam".to_string());
    }

    installation.manually_linked = true;

    crate::storage::insert_installation(&state.db, &installation)
        .await
        .map_err(|e| e.to_string())?;

    try_link_steam_installation(&state, &installation).await;

    Ok(())
}

 

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct AchievementView {
    pub name: String,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub unlocked_at: Option<String>,
}

#[tauri::command]
pub async fn get_achievements_for_installation(
    state: State<'_, AppState>,
    installation_id: String,
) -> Result<Vec<AchievementView>, String> {
    let row: Option<(Option<String>,)> = sqlx::query_as(
        "SELECT provider_game_record_id FROM installations WHERE id = ?"
    )
    .bind(&installation_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| e.to_string())?;

    let Some((Some(record_id),)) = row else {
        return Ok(vec![]);
    };

    sqlx::query_as::<_, AchievementView>(
        "SELECT ad.name, ad.description, ad.icon_url, au.unlocked_at
         FROM achievement_definitions ad
         JOIN achievement_unlocks au ON au.achievement_definition_id = ad.id
         WHERE ad.provider_game_record_id = ?
         ORDER BY au.unlocked_at IS NULL, au.unlocked_at DESC"
    )
    .bind(&record_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| e.to_string())
}