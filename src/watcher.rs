use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher, event::EventKind};
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, channel};
use std::thread;
use std::time::Duration;

use crate::{config::Config, sorter, utils};

pub fn start_watching(path: PathBuf, _cfg: Config, dry_run: bool) {
    let (tx, rx) = channel::<Event>();

    let path_clone = path.clone();
    thread::spawn(move || {
        // create watcher thread
        let mut watcher: RecommendedWatcher = Watcher::new(
            move |res| {
                if let Ok(event) = res {
                    let _ = tx.send(event);
                }
            },
            notify::Config::default(),
        )
        .expect("Failed to create watcher");

        watcher
            .watch(&path_clone, RecursiveMode::NonRecursive)
            .expect("Failed to watch downloads folder");

        loop {
            thread::sleep(Duration::from_secs(999999));
        }
    });

    listen_for_events(rx, path, dry_run);
}

fn listen_for_events(rx: Receiver<Event>, downloads_path: PathBuf, dry_run: bool) {
    utils::info("👀 Watcher running and waiting for new files...");

    for event in rx {
        // we only care about file creation
        if let EventKind::Create(_) = event.kind {
            for path in event.paths {
                // ignore directories or weird temp files
                if ignore_path(&path) {
                    continue;
                }

                utils::info(&format!(
                    "📥 New file detected: {}",
                    utils::highlight(&path.display().to_string())
                ));

                // wait a little so file finishes writing
                thread::sleep(Duration::from_millis(300));

                sorter::sort_file_path(&path, &downloads_path, dry_run);
            }
        }
    }
}

fn ignore_path(path: &PathBuf) -> bool {
    if path.is_dir() {
        return true;
    }
    if let Some(ext) = path.extension() {
        let ext = ext.to_string_lossy().to_lowercase();
        return ext == "crdownload" || ext == "part";
    }
    false
}
