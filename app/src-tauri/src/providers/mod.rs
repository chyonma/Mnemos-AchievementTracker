use crate::core::{AchievementDefinition, AchievementUnlock};

pub struct ProviderAchievementData {
    pub definition: AchievementDefinition,
    pub unlock: AchievementUnlock,
}

#[async_trait::async_trait]
pub trait AchievementProvider {
    fn provider_key(&self) -> &'static str;

    async fn fetch_achievements(&self, source_id: &str) -> Result<Vec<ProviderAchievementData>, String>;
}

pub mod steam;