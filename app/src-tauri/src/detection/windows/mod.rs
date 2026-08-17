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

    let mut best: Option<(std::path::PathBuf, u64)> = None;

    for entry in entries.flatten() {
        let file_path = entry.path();
        if file_path.extension().and_then(|e| e.to_str()) != Some("exe") {
            continue;
        }

        let Ok(metadata) = entry.metadata() else { continue };
        let size = metadata.len();

        if best.as_ref().map_or(true, |(_, best_size)| size > *best_size) {
            best = Some((file_path, size));
        }
    }

    let (exe_path, _) = best?;
    let display_name = game_dir.file_name()?.to_str()?.to_string();
    let executable_name = exe_path.file_name()?.to_str()?.to_string();

    let mut installation = Installation::new(
        exe_path.to_str()?.to_string(),
        executable_name,
        game_dir.to_str()?.to_string(),
        display_name,
    );

    Some(installation)
}