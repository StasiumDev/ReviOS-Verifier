use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReviVersion {
    pub name: String,
    pub md5: String,
    pub sha256: String
}

pub async fn get_revi_hashes() -> anyhow::Result<Vec<ReviVersion>> {
    let response = reqwest::Client::new()
        .get("https://api.stasium.dev/v1/revios/hashes")
        .send()
        .await?;

    if response.status() != reqwest::StatusCode::OK {
        anyhow::bail!("Failed to fetch hashes from the API!");
    }

    let json_raw = response.text().await?;
    Ok(serde_json::from_str(&json_raw)?)
}
