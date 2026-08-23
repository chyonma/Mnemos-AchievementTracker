use crate::detection::GameDetector;
use sysinfo::System;
use std::fs;
use std::path::Path;
use crate::core::Installation;

pub struct WindowsGameDetector;

impl GameDetector for WindowsGameDetector {

    fn get_running_processes(&self) -> Vec<String> {
        let mut sys = System::new_all();
        sys.refresh_all();

        sys.processes().values().filter_map(|process| {
            process.exe().map(|path| path.display().to_string())
        }).collect()
    }

    fn discover_installations(&self, roots: &[(String, Option<String>)]) -> Vec<Installation> {
        let mut discovered = Vec::new();

        for (root, launcher_hint) in roots {
            let path = Path::new(root);
            if !path.exists() {
                continue;
            }

            let Ok(entries) = fs::read_dir(path) else { continue };

            for entry in entries.flatten() {
                let game_dir = entry.path();
                if !game_dir.is_dir() {
                    continue;
                }

                if let Some(mut installation) = find_main_executable(&game_dir) {
                    installation.known_launcher = launcher_hint.clone();
                    discovered.push(installation);
                }
            }
        }

        discovered
    }
}

fn find_main_executable(game_dir: &Path) -> Option<Installation> {
    let Ok(entries) = fs::read_dir(game_dir) else { return None };

    let exes: Vec<std::path::PathBuf> = entries.flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("exe"))
        .collect();

    if exes.len() != 1 {
        return None; //  let user link manually.
    }

    let exe_path = &exes[0];
    let display_name = game_dir.file_name()?.to_str()?.to_string();
    let executable_name = exe_path.file_name()?.to_str()?.to_string();

    Some(Installation::new(
        exe_path.to_str()?.to_string(),
        executable_name,
        game_dir.to_str()?.to_string(),
        display_name,
    ))
}