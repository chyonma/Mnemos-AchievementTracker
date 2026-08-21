pub async fn save_provider_achievements(
    pool: &SqlitePool,
    provider_game_record_id: &str,
    merged: &[(crate::core::AchievementDefinition, Option<String>)],
) -> Result<(), sqlx::Error> {
    let now = chrono::Utc::now().to_rfc3339();

    for (definition, unlocked_at) in merged {
        let existing_def: Option<(String,)> = sqlx::query_as(
            "SELECT id FROM achievement_definitions WHERE provider_game_record_id = ? AND provider_achievement_key = ?"
        )
        .bind(provider_game_record_id)
        .bind(&definition.provider_achievement_key)
        .fetch_optional(pool)
        .await?;

        let definition_id = if let Some((id,)) = existing_def {
            sqlx::query(
                "UPDATE achievement_definitions SET name = ?, description = ?, icon_url = ?, fetched_at = ? WHERE id = ?"
            )
            .bind(&definition.name)
            .bind(&definition.description)
            .bind(&definition.icon_url)
            .bind(&now)
            .bind(&id)
            .execute(pool)
            .await?;
            id
        } else {
            let new_id = uuid::Uuid::new_v4().to_string();
            sqlx::query(
                "INSERT INTO achievement_definitions (id, provider_game_record_id, provider_achievement_key, name, description, icon_url, fetched_at)
                VALUES (?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(&new_id)
            .bind(provider_game_record_id)
            .bind(&definition.provider_achievement_key)
            .bind(&definition.name)
            .bind(&definition.description)
            .bind(&definition.icon_url)
            .bind(&now)
            .execute(pool)
            .await?;
            new_id
        };

        let existing_unlock: Option<(String,)> = sqlx::query_as(
            "SELECT id FROM achievement_unlocks WHERE achievement_definition_id = ?"
        )
        .bind(&definition_id)
        .fetch_optional(pool)
        .await?;

        if let Some((id,)) = existing_unlock {
            sqlx::query("UPDATE achievement_unlocks SET unlocked_at = ? WHERE id = ?")
                .bind(unlocked_at)
                .bind(&id)
                .execute(pool)
                .await?;
        } else {
            sqlx::query(
                "INSERT INTO achievement_unlocks (id, achievement_definition_id, unlocked_at, notified) VALUES (?, ?, ?, ?)"
            )
            .bind(uuid::Uuid::new_v4().to_string())
            .bind(&definition_id)
            .bind(unlocked_at)
            .bind(false)
            .execute(pool)
            .await?;
        }
    }
    Ok(())
}