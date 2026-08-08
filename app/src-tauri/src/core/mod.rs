pub struct Installation{
    pub id: String,
    pub executable_path: String,
    pub executable_name: String,
    pub install_directory: String,
    pub display_name: String,
    pub known_launcher: Option<String>,
    pub steam_app_id: Option<String>,
    pub manually_linked: bool,
}