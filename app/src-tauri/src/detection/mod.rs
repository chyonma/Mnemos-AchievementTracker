pub trait GameDetector {
    fn get_running_processes(&self) -> Vec<String>;
}

pub mod windows;