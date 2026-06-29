use std::{io::Error, path::Path};

pub fn get_repository_file_content(file_path: &Path) -> Result<Vec<u8>, Error> {
    std::fs::read(file_path)
}
