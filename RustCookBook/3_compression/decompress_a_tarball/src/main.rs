use std::fs::File;
use flate2::Compression;
use flate2::write::{GzDecoder, GzEncoder};
use tar::Archive;

fn main() -> Result<(), std::io::Error> {
    let tar_gz = File::create("test.tar.gz")?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("backups", "test")?;

    let path = "test.tar.gz";
    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;

    Ok(())
}