use anyhow::bail;
use log::{debug, info};

mod revi_version;
mod logger;
mod hasher;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::args().len() < 2 {
        bail!("Please provide the path to the ISO file as an argument!");
    }

    // Initialize the logger
    logger::init();

    // Printing the ASCII art
    const ASCII: &str  = include_str!("../ascii.txt");
    println!("\n\x1b[38;5;203m{}\x1b[0m\n", ASCII);

    debug!("Running in DEBUG mode..");

    // Fetching hashes from Rest API
    info!("Retrieving official ReviOS hashes...");
    let official_hashes = revi_version::get_revi_hashes().await?;

    // Iterating over all provided files
    for path in std::env::args().skip(1) {

        // Opening the file in read-only mode
        let mut file = std::fs::File::open(&path)?;

        // Computing SHA-256 hash
        let sha256_hash = hasher::compute_sha256(&mut file).await?;
        info!("SHA-256: {}", sha256_hash);

        // Computing MD5 hash
        let md5_hash = hasher::compute_md5(&mut file).await?;
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

        match matching_hash {
            Some(version) => {
                info!("\x1b[38;5;113mSHA-256 / MD5 hash of \"{}\" matches with the official ReviOS ISO:\x1b[0m", file_name);
                info!("Name:   {}", version.name);
                info!("SHA256: {}", version.sha256);
                info!("MD5:    {}", version.md5);
            },
            None => {
                info!("\x1b[38;5;203mUnable to find a matching SHA256 / MD5 hash for \"{}\"!\x1b[0m", file_name);
                info!("Either the ISO is corrupted or not an official ReviOS ISO!");
                info!("If you obtained the ISO from an unofficial source, please download it from our official website.");
                info!("However, If the error messages still occurs, please re-download the ISO");
                info!("because it got corrupted due to unstable connection, servers, etc.");
            }
        }

        info!("==========================================================");
    }

    // if the binary was compiled for Windows, we need to wait for the user to press a key to not automatically close the terminal
    #[cfg(target_os = "windows")]
    {
        info!("\x1b[38;5;113mConfirmation has ended. Press any key to quit the tool\x1b[0m");
        std::io::stdin().read_line(&mut String::new()).unwrap();
    }

    Ok(())
}
