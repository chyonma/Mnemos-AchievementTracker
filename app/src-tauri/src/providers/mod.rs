use crate::core::AchievementDefinition;
pub struct RawUnlock {
    pub provider_achievement_key: String,
    pub unlocked_at: Option<String>,
}

#[async_trait::async_trait]
pub trait AchievementDefinitionSource {
    fn provider_key(&self) -> &'static str;
    async fn fetch_definitions(&self, source_id: &str) -> Result<Vec<AchievementDefinition>, String>;
}

pub mod steam;