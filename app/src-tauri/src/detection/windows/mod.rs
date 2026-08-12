use crate::detection::GameDetector;
use sysinfo::System;
pub struct WindowsGameDetector;

impl GameDetector for WindowsGameDetector {
    fn get_running_processes(&self) -> Vec<String> {
        let mut sys = System::new_all();
        sys.refresh_all();

        sys.processes().values().filter_map(|process| {
            process.exe().map(|path.display().to_string())
        }).collect()
    }
}