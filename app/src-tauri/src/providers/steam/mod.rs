use crate::providers::{AchievementDefinitionSource, AchievementUnlockSource};
use crate::core::AchievementDefinition;
use serde::Deserialize;

pub struct SteamProvider {
    api_key: String,
    steam_id: String,
}

impl SteamProvider {
    pub fn new(api_key: String, steam_id: String) -> Self {
        Self { api_key, steam_id }
    }
}

#[derive(Deserialize)]
struct SchemaResponse { game: SchemaGame }

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SchemaGame {
    game_name: Option<String>,
    available_game_stats: Option<AvailableGameStats>,
}

#[derive(Deserialize)]
struct AvailableGameStats { achievements: Option<Vec<SchemaAchievement>> }

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SchemaAchievement {
    name: String,
    display_name: String,
    description: Option<String>,
    icon: Option<String>,
}

#[derive(Deserialize)]
struct PlayerAchievementsResponse { playerstats: PlayerStats }

#[derive(Deserialize)]
struct PlayerStats {
    success: bool,
    error: Option<String>,
    achievements: Option<Vec<PlayerAchievement>>,
}

#[derive(Deserialize)]
struct PlayerAchievement { apiname: String, achieved: i32, unlocktime: i64 }

#[async_trait::async_trait]
impl AchievementDefinitionSource for SteamProvider {
    fn provider_key(&self) -> &'static str { "steam" }

    async fn fetch_definitions(&self, source_id: &str) -> Result<Vec<AchievementDefinition>, String> {
        let client = reqwest::Client::new();
        let schema_url = format!(
            "https://api.steampowered.com/ISteamUserStats/GetSchemaForGame/v2/?key={}&appid={}&format=json",
            self.api_key, source_id
        );
        let schema: SchemaResponse = client.get(&schema_url).send().await
            .map_err(|e| e.to_string())?
            .json().await
            .map_err(|e| e.to_string())?;

        let raw = schema.game.available_game_stats
            .and_then(|s| s.achievements)
            .unwrap_or_default();

        Ok(raw.into_iter().map(|def| AchievementDefinition {
            id: uuid::Uuid::new_v4().to_string(),
            provider_game_record_id: String::new(),
            provider_achievement_key: def.name,
            name: def.display_name,
            description: def.description,
            icon_url: def.icon,
            fetched_at: chrono::Utc::now().to_rfc3339(),
        }).collect())
    }
}

#[async_trait::async_trait]
impl AchievementUnlockSource for SteamProvider {
    fn provider_key(&self) -> &'static str { "steam" }

    async fn fetch_unlocks(&self, source_id: &str) -> Result<Vec<crate::providers::RawUnlock>, String> {
        let client = reqwest::Client::new();
        let player_url = format!(
            "https://api.steampowered.com/ISteamUserStats/GetPlayerAchievements/v0001/?appid={}&key={}&steamid={}",
            source_id, self.api_key, self.steam_id
        );
        let player: PlayerAchievementsResponse = client.get(&player_url).send().await
            .map_err(|e| e.to_string())?
            .json().await
            .map_err(|e| e.to_string())?;

        if !player.playerstats.success {
            return Err(player.playerstats.error
                .unwrap_or_else(|| "Steam reported failure".to_string()));
        }

        let unlocks = player.playerstats.achievements.unwrap_or_default();
        Ok(unlocks.into_iter().map(|u| crate::providers::RawUnlock {
            provider_achievement_key: u.apiname,
            unlocked_at: if u.achieved == 1 {
                chrono::DateTime::from_timestamp(u.unlocktime, 0)
                    .map(|dt| dt.to_rfc3339())
            } else {
                None
            },
        }).collect())
    }
}