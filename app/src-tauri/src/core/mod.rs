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

pub struct Session{
    pub id: String,
    pub installation_id: String,
    pub started_at: String,
    pub ended_at: Option<String>,
    pub duration_seconds: Option<u64>,
    pub manually_edited: bool,
}