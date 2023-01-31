use log::{debug, info};
use version_compare::{Cmp, Version};

use crate::models::github::GithubResponse;
use crate::VERSION;

const API_URL: &str = "https://api.github.com/repos/StasiumDev/ReviOS-Verifier/releases/latest";
const USER_AGENT: &str = "ReviOS-Verifier";

pub async fn check_for_update() -> anyhow::Result<()> {
    // Fetching the latest release from GitHub
    let response = reqwest::Client::new()
        .get(API_URL)
        .header("User-Agent", USER_AGENT)
        .send()
        .await?;

    if response.status() != reqwest::StatusCode::OK {
        debug!("Failed to fetch latest release from GitHub");
        debug!("Status code: {}", response.status());
        debug!("Response body: {}", response.text().await?);
        anyhow::bail!("Failed to fetch latest release info from GitHub!");
    }

    // Parsing the response as JSON
    let latest_release = response.text().await?;
    let github_response: GithubResponse = serde_json::from_str(&latest_release)?;

    // Getting the tag name
    let current_version = Version::from(VERSION).expect("Failed to parse current version!");
    let github_version =
        Version::from(&github_response.tag_name).expect("Failed to parse GitHub version!");

    // Comparing the versions
    match current_version.compare(&github_version) {
        Cmp::Lt => {
            debug!(
                "Newer version available! Current: {}, Latest: {}",
                current_version, github_version
            );
            info!(
                "========================= \x1b[38;5;113mUPDATE\x1b[0m ========================="
            );
            info!("\x1b[38;5;229mA new version is available!\x1b[0m");
            info!("Download: https://github.com/StasiumDev/ReviOS-Verifier/releases");
            info!("==========================================================");
        }
        _ => debug!("No update available!"),
    };

    Ok(())
}
