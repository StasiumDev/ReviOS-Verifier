use sha2::Digest;
use log::info;
use std::io::Seek;

pub async fn compute_md5(file: &mut std::fs::File) -> anyhow::Result<String> {
    // Creating MD5 hasher
    let mut md5_hasher = md5::Md5::new();

    info!("Computing MD5 hash, please wait...");
    file.seek(std::io::SeekFrom::Start(0))?;

    // Copying the file contents to the hasher
    std::io::copy(file, &mut md5_hasher)?;
    Ok(format!("{:X}", md5_hasher.finalize()))
}

pub async fn compute_sha256(file: &mut std::fs::File) -> anyhow::Result<String> {
    // Creating SHA256 hasher
    let mut sha256_hasher = sha2::Sha256::new();

    info!("Computing SHA-256 hash, please wait...");
    file.seek(std::io::SeekFrom::Start(0))?;

    // Copying the file contents to the hasher
    std::io::copy(file, &mut sha256_hasher)?;
    Ok(format!("{:X}", sha256_hasher.finalize()))
}