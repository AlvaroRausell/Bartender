use std::{fs::{self, read_dir}, io::Cursor, path::{Path, PathBuf}};

use log::{error, info};
use zip_extract::ZipExtractError;

pub fn extract(binary_data :Vec<u8> , target_dir: &PathBuf) -> Result<(),ZipExtractError>{
    zip_extract::extract(Cursor::new(binary_data), &target_dir, true)
}

/**
 * Extracts the given file and any further contained zips. If it is not a zip, it fails silently and continues
 */
pub fn extract_recourse(path: &Path) {
    let target_dir = path.with_extension("");
    let binary_data = fs::read(path);
    info!("Extracting {path:?}");
    if let Err(e) = binary_data {
        error!("Failed to read binary data of {path:?} with error: {e:?}");
        return;
    }
    if let Err(e) = extract(binary_data.unwrap(), &target_dir) {
        error!("Failed to extract {path:?} with error: {e:?}");
        return;
    }
    let contents = read_dir(target_dir).unwrap();
    for file in contents {
        extract_recourse(file.unwrap().path().as_ref());
    }
}