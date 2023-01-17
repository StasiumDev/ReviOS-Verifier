use anyhow::bail;
use log::{debug, info};
use sha2::Digest;
use std::io::Seek;

mod revi_version;
mod logger;


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

    // Get the path to the ISO
    let path = std::env::args().nth(1).unwrap();

    // Opening a file handle to the file
    let mut file = std::fs::File::open(path)?;

    // Creating a SHA256 and MD5 hasher
    let mut sha256_hasher = sha2::Sha256::new();
    let mut md5_hasher = md5::Md5::new();

    info!("Computing SHA256 hash, please wait...");
    // Copying the file contents to the hasher
    std::io::copy(&mut file, &mut sha256_hasher)?;
    let sha256_hash = format!("{:X}", sha256_hasher.finalize());
    info!("SHA-256: {}", sha256_hash);

    info!("Computing MD5 hash, please wait...");
    file.seek(std::io::SeekFrom::Start(0))?;
    // Copying the file contents to the hasher
    std::io::copy(&mut file, &mut md5_hasher)?;
    let md5_hash = format!("{:X}", md5_hasher.finalize());
    info!("MD5: {}", md5_hash);

    // Fetching hashes from Rest API
    info!("Retrieving official ReviOS hashes...");
    let official_hashes = revi_version::get_revi_hashes().await?;

    // Using the find method to find a matching MD5 / SHA256 hash
    info!("Comparing hashes...");
    let found_version = official_hashes
        .iter()
        .find(|entry| entry.sha256 == sha256_hash || entry.md5 == md5_hash);

    info!("========================= \x1b[38;5;113mRESULT\x1b[0m =========================");

    if found_version.is_none() {
        info!("\x1b[38;5;203mUnable to find a matching SHA256 / MD5 hash!\x1b[0m");
        info!("Either the ISO is corrupted or not an official ReviOS ISO!");
        info!("If you obtained the ISO from an unofficial source, please download it from our official website.");
        info!("However, If the error messages still occurs, please re-download the ISO");
        info!("because it got corrupted due to unstable connection, servers, etc.");
    } else {
        let version = found_version.unwrap();
        info!("\x1b[38;5;113mYour SHA256 / MD5 hash matches with the official ReviOS ISO:\x1b[0m");
        info!("Name:   {}", version.name);
        info!("SHA256: {}", version.sha256);
        info!("MD5:    {}", version.md5);
    }

    info!("==========================================================");

    #[cfg(target_os = "windows")]
    {
        info!("\x1b[38;5;113mConfirmation has ended. Press any key to quit the tool\x1b[0m");
        std::io::stdin().read_line(&mut String::new()).unwrap();
    }

    Ok(())
}
