use crate::providers::{AchievementProvider, ProviderAchievementData};

pub struct SteamProvider;

#[async_trait::async_trait]
impl AchievementProvider for SteamProvider {
    fn provider_key(&self) -> &'static str { "steam" }

    async fn fetch_achievements(&self, _source_id: &str) -> Result<Vec<ProviderAchievementData>, String> {
        Ok(vec![])
    }
}