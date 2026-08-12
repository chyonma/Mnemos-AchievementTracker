#[derive (Debug, Clone, serde::Serialize)]
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
impl Installation{
    pub fn new(
        executable_path: String,
        executable_name: String,
        install_directory: String,
        display_name: String,
    ) -> Self {
        Self{ 
            id: uuid::Uuid::new_v4().to_string(),
            executable_path,
            executable_name,
            install_directory,
            display_name,
            known_launcher: None,
            steam_app_id: None,
            manually_linked: false,

      
        }
    }

}

#[derive (Debug, Clone, serde::Serialize)]
pub struct Session{
    pub id: String,
    pub installation_id: String,
    pub started_at: String,
    pub ended_at: Option<String>,
    pub duration_seconds: Option<u64>,
    pub manually_edited: bool,
}
impl Session{
    pub fn new (installation_id: String, started_at: String) -> Self{
        Self{
            id: uuid::Uuid::new_v4().to_string(),
            installation_id,
            started_at,
            ended_at: None,
            duration_seconds: None,
            manually_edited: false,

        }
    }
    pub fn end(&mut self, ended_at: String, duration_seconds: u64){
        self.ended_at = Some(ended_at);
        self.duration_seconds = Some(duration_seconds);
    }

}
