use sha2::Digest;
use std::io::{Read, Seek, Write};

const SIZE: usize = 0xFFFF;

pub fn compute_hash<D: Digest + Write>(file: &mut std::fs::File) -> anyhow::Result<String> {
    // Seeking to the beginning of the file
    file.seek(std::io::SeekFrom::Start(0))?;

    // Creating Hasher
    let mut hasher = D::new();
    let mut file_data = vec![0; SIZE];

    // Reading the file in chunks of 64KB
    #[allow(unused_assignments)]
    let mut bytes_read = 0;
    loop {
        bytes_read = file.read(&mut file_data)?;
        if bytes_read == SIZE {
            hasher.update(&file_data);
        } else {
            hasher.update(&file_data[0..bytes_read]);
            break;
        }
    }

    let hash = hasher.finalize();
    let sz = <D as Digest>::output_size();
    let mut ret = vec![0; sz];
    ret.copy_from_slice(&hash);

    Ok(ret.iter().map(|byte| format!("{:02X}", byte)).collect())
}
