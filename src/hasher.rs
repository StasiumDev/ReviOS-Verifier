use sha2::Digest;
use log::info;
use std::io::Seek;

pub async fn compute_md5(file: &mut std::fs::File) -> anyhow::Result<String> {
    info!("Computing MD5 hash, please wait...");

    // Creating MD5 hasher
    let mut hasher = md5::Md5::new();
    file.seek(std::io::SeekFrom::Start(0))?;

    // Copying the file contents to the hasher
    std::io::copy(file, &mut hasher)?;
    Ok(format!("{:X}", hasher.finalize()))
}

pub async fn compute_sha256(file: &mut std::fs::File) -> anyhow::Result<String> {
    info!("Computing SHA-256 hash, please wait...");

    // Creating SHA-256 hasher
    let mut hasher = sha2::Sha256::new();
    file.seek(std::io::SeekFrom::Start(0))?;

    // Copying the file contents to the hasher
    std::io::copy(file, &mut hasher)?;
    Ok(format!("{:X}", hasher.finalize()))
}
