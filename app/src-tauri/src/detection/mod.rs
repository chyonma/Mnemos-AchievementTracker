pub trait GameDetector {
    fn get_running_processes(&self) -> Vec<String>;

    fn discover_installations(&self, roots: &[(String, Option<String>)]) -> Vec<crate::core::Installation>;
}

pub mod windows;