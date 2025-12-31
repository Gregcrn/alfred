use dirs;
use std::path::PathBuf;

pub fn get_downloads_path() -> Option<PathBuf> {
    dirs::download_dir()
}
