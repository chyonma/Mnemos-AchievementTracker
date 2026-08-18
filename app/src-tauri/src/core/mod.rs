#[derive(Debug, Clone, serde::Serialize)]
pub struct Installation {
    pub id: String,
    pub executable_path: String,
    pub executable_name: String,
    pub install_directory: String,
    pub display_name: String,
    pub known_launcher: Option<String>,
    pub steam_app_id: Option<String>,
    pub provider_game_record_id: Option<String>,
    pub manually_linked: bool,
}

impl Installation {
    pub fn new(
        executable_path: String,
        executable_name: String,
        install_directory: String,
        display_name: String,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            executable_path,
            executable_name,
            install_directory,
            display_name,
            known_launcher: None,
            steam_app_id: None,
            provider_game_record_id: None,
            manually_linked: false,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Game {
    pub id: String,
    pub name: String,
    pub created_at: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ProviderGameRecord {
    pub id: String,
    pub game_id: String,
    pub provider: String,
    pub source_id: String,
    pub created_at: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct AchievementDefinition {
    pub id: String,
    pub provider_game_record_id: String,
    pub provider_achievement_key: String,
    pub name: String,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub fetched_at: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct AchievementUnlock {
    pub id: String,
    pub achievement_definition_id: String,
    pub unlocked_at: Option<String>,
    pub notified: bool,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Session {
    pub id: String,
    pub installation_id: String,
    pub started_at: String,
    pub ended_at: Option<String>,
    pub duration_seconds: Option<u64>,
    pub manually_edited: bool,
    pub last_heartbeat: Option<String>,
}

impl Session {
    pub fn new(installation_id: String, started_at: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            installation_id,
            started_at,
            ended_at: None,
            duration_seconds: None,
            manually_edited: false,
            last_heartbeat: None,
        }
    }

    pub fn end(&mut self, ended_at: String, duration_seconds: u64) {
        self.ended_at = Some(ended_at);
        self.duration_seconds = Some(duration_seconds);
    }

    pub fn last_heartbeat(&mut self, at: String) {
        self.last_heartbeat = Some(at);
    }
}

pub fn match_running_installations(
    running_paths: &[String],
    installations: &[Installation],
) -> Vec<Installation> {
    installations
        .iter()
        .filter(|installation| running_paths.contains(&installation.executable_path))
        .cloned()
        .collect()
}

pub struct DiscoveryReconciliation {
    pub new: Vec<Installation>,
    pub updated: Vec<Installation>,
}

pub fn reconcile_discovered_installations(discovered: &[Installation], known: &[Installation]) -> DiscoveryReconciliation {
    let mut new = Vec::new();
    let mut updated = Vec::new();
    let mut seen_paths: Vec<&str> = Vec::new();

    for d in discovered {
        if seen_paths.contains(&d.executable_path.as_str()) {
            continue;
        }
        seen_paths.push(&d.executable_path);

        match known.iter().find(|k| k.executable_path == d.executable_path) {
            None => new.push(d.clone()),
            Some(existing) => {
                if existing.known_launcher.is_none() && d.known_launcher.is_some() {
                    let mut merged = existing.clone();
                    merged.known_launcher = d.known_launcher.clone();
                    updated.push(merged);
                }
            }
        }
    }

    DiscoveryReconciliation { new, updated }
}

pub fn start_sessions_for(installations: &[Installation], started_at: String) -> Vec<Session> {
    installations
        .iter()
        .map(|installation| Session::new(installation.id.clone(), started_at.clone()))
        .collect()
}

pub fn recover_orphaned_session(started_at: &str, last_heartbeat: &Option<String>) -> (String, i64) {
    let ended_at = last_heartbeat.clone().unwrap_or_else(|| started_at.to_string());

    let start = chrono::DateTime::parse_from_rfc3339(started_at)
        .expect("invalid started_at timestamp");
    let end = chrono::DateTime::parse_from_rfc3339(&ended_at)
        .expect("invalid ended_at timestamp");

    let duration = (end - start).num_seconds().max(0);

    (ended_at, duration)
}