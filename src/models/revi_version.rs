use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReviVersion {
    pub name: String,
    pub md5: String,
    pub sha256: String,
}

pub async fn get_revi_hashes() -> anyhow::Result<Vec<ReviVersion>> {
    let hashes = include_str!("../.././hashes.json");
    Ok(serde_json::from_str(&hashes)?)
}
