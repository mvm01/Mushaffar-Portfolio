use chrono::{Duration, Utc};
use rand::seq::SliceRandom;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct UserRecord {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub status: String,
    pub date: String,
}

#[tauri::command]
pub fn generate_mock_data() -> Vec<UserRecord> {
    let first_names = [
        "Amina", "Lucas", "Jordan", "Maya", "Elias", "Nadia", "Sofia", "Noah", "Avery", "Mateo",
    ];
    let last_names = [
        "Khan", "Reyes", "Bennett", "Cole", "Patel", "Turner", "Brooks", "Morales", "Young", "Diaz",
    ];
    let statuses = ["Active", "Pending", "Archived"];

    let mut rng = rand::thread_rng();

    (1..=50)
        .map(|id| {
            let first = first_names.choose(&mut rng).unwrap_or(&"Alex");
            let last = last_names.choose(&mut rng).unwrap_or(&"Taylor");
            let status = statuses.choose(&mut rng).unwrap_or(&"Active");
            let created_at = (Utc::now() - Duration::days(id.into()))
                .format("%b %d, %Y")
                .to_string();
            let slug = format!("{}.{}", first.to_lowercase(), last.to_lowercase());

            UserRecord {
                id,
                name: format!("{first} {last}"),
                email: format!("{slug}{id}@local-manager.dev"),
                status: status.to_string(),
                date: created_at,
            }
        })
        .collect()
}
