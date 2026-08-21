use crate::core::AchievementDefinition;
pub struct RawUnlock {
    pub provider_achievement_key: String,
    pub unlocked_at: Option<String>,
}

#[async_trait::async_trait]
pub trait AchievementProvider {
    fn provider_key(&self) -> &'static str;

    async fn fetch_achievements(&self, source_id: &str) -> Result<Vec<ProviderAchievementData>, String>;
}

pub mod steam;