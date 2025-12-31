use crate::utils;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

pub fn sort_file_path(file: &Path, downloads: &PathBuf, dry_run: bool) {
    let extension = match file.extension() {
        Some(ext) => ext.to_string_lossy().to_lowercase(),
        None => return, // Ignore files with no extension
    };

    let target_folder = match extension.as_str() {
        "pdf" => "PDFs",
        "jpg" | "jpeg" | "png" | "gif" | "webp" => "Images",
        "zip" | "tar" | "gz" | "rar" | "7z" => "Archives",
        "mp4" | "mov" | "avi" | "mkv" => "Videos",
        "docx" | "doc" | "xlsx" | "pptx" | "txt" => "Documents",
        "mp3" | "wav" | "flac" => "Music",
        _ => "Others",
    };

    let target_dir = downloads.join(target_folder);

    // Create target directory if it doesn't exist
    if let Err(e) = fs::create_dir_all(&target_dir) {
        utils::error(&format!(
            "Creating directory {}: {}",
            target_dir.display(),
            e
        ));
        return;
    }

    let file_name = match file.file_name() {
        Some(name) => name,
        None => return,
    };

    let dest_path = target_dir.join(file_name);

    utils::info(&format!(
        "Moving {} to {}",
        utils::highlight(&file.display().to_string()),
        utils::highlight(target_folder)
    ));

    if dry_run {
        return;
    }

    if let Err(e) = fs::rename(file, &dest_path) {
        utils::error(&format!("Moving file: {}", e));
    }
}

pub fn scan_and_sort(downloads: &PathBuf, dry_run: bool) {
    utils::info("Starting initial scan of existing files...");
    let start = Instant::now();
    let mut count = 0;

    if let Ok(entries) = fs::read_dir(downloads) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                sort_file_path(&path, downloads, dry_run);
                count += 1;
            }
        }
    }

    let duration = start.elapsed();
    utils::success(&format!(
        "Scan complete! {} files organized in {}.",
        utils::highlight(&count.to_string()),
        utils::highlight(&format!("{:?}", duration))
    ));
}
