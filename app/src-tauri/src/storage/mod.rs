pub async fn insert_watched_folder(pool: &SqlitePool, path: &str) -> Result<(), sqlx::Error> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query("INSERT INTO watched_folders (id, path, created_at) VALUES (?, ?, ?)")
        .bind(&id)
        .bind(path)
        .bind(&now)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_watched_folders(pool: &SqlitePool) -> Result<Vec<String>, sqlx::Error> {
    let rows: Vec<(String,)> = sqlx::query_as("SELECT path FROM watched_folders")
        .fetch_all(pool)
        .await?;
    Ok(rows.into_iter().map(|(p,)| p).collect())
}


pub async fn update_installation_launcher(pool: &SqlitePool, installation_id: &str, known_launcher: &str) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE installations SET known_launcher = ?, updated_at = ? WHERE id = ?")
        .bind(known_launcher)
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(installation_id)
        .execute(pool)
        .await?;
    Ok(())
}