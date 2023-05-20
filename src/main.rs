use anyhow::bail;
use log::{debug, info};

use crate::models::revi_version;
use crate::utils::{hasher, update_checker};

mod logger;
mod models;
mod utils;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the logger
    logger::init();

    match run_verifier().await {
        Ok(_) => {
            info!("\x1b[38;5;113mConfirmation has ended.\x1b[0m");
        }
        Err(err) => {
            log::error!("{}", err);
        }
    }

    // if the binary was compiled for Windows, we need to wait for the user to press enter to not automatically close the terminal
    if cfg!(target_os = "windows") {
        info!("Press enter to quit the tool!");
        std::io::stdin().read_line(&mut String::new()).unwrap();
    }

    Ok(())
}

async fn run_verifier() -> anyhow::Result<()> {
    // Printing the ASCII art
    const ASCII: &str = include_str!("../ascii.txt");
    println!("\n\x1b[38;5;203m{}\x1b[0m\n", ASCII);

    // Printing the version info
    info!("Version: v{}", VERSION);
    info!("Author:  Stasium#0001");
    info!("GitHub:  https://github.com/StasiumDev");
    debug!("Running in DEBUG mode..");

    // Checking for updates
    update_checker::check_for_update().await?;

    // Getting the ISO files from the command line arguments
    let mut iso_fies = std::env::args()
        .skip(1)
        .map(|arg| std::path::PathBuf::from(arg))
        .collect::<Vec<_>>();

    // If no ISO files were provided, open a file picker
    if iso_fies.is_empty() {
        info!("No iso files provided, opening file picker..");
        let files = rfd::FileDialog::new()
            .add_filter("ISO Files", &["iso"])
            .pick_files();

        if let Some(files) = files {
            info!("Selected {} file(s)!", files.len());
            files.into_iter().for_each(|file| {
                iso_fies.push(file);
            });
        } else {
            bail!("No iso files provided!");
        }
    }

    // Fetching hashes from Rest API
    info!("Retrieving official ReviOS hashes...");
    let official_hashes = revi_version::get_revi_hashes().await?;
    info!("Retrieved {} hashes!", official_hashes.len());

    // Iterating over all provided files
    for path in iso_fies {
        // Opening the file in read-only mode
        let mut file = std::fs::File::open(&path)?;
        info!("Computing SHA-256 hash, please wait...");
        let sha256_hash = hasher::compute_hash::<sha2::Sha256>(&mut file)?;
        info!("SHA-256: {}", sha256_hash);

        // Computing MD5 hash
        info!("Computing MD5 hash, please wait...");
        let md5_hash = hasher::compute_hash::<md5::Md5>(&mut file)?;
        info!("MD5: {}", md5_hash);

        // Looking for a matching hash using the find method
        info!("Comparing hashes...");
        let matching_hash = official_hashes
            .iter()
            .find(|entry| entry.sha256 == sha256_hash || entry.md5 == md5_hash);

        info!("========================= \x1b[38;5;113mRESULT\x1b[0m =========================");

        let file_name = std::path::Path::new(&path)
            .file_name()
            .expect("Failed to get file name")
            .to_str()
            .expect("Failed to convert file name to string");

        if let Some(version) = matching_hash {
            info!("\x1b[38;5;113mSHA-256 / MD5 hash of \"{}\" matches with the official ReviOS ISO:\x1b[0m", file_name);
            info!("Name:   {}", version.name);
            info!("SHA256: {}", version.sha256);
            info!("MD5:    {}", version.md5);
        } else {
            info!("\x1b[38;5;203mUnable to find a matching SHA256 / MD5 hash for \"{}\"!\x1b[0m", file_name);
            info!("Either the ISO is corrupted or not an official ReviOS ISO!");
            info!("If you obtained the ISO from an unofficial source, please download it from our official website.");
            info!("However, If the error messages still occurs, please re-download the ISO");
            info!("because it got corrupted due to unstable connection, servers, etc.");
        }

        info!("==========================================================");
    }

    Ok(())
}
