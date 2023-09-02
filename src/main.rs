use chrono::{DateTime, Local};
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::fs;
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

const DATE_FORMAT: &str = "%Y/%m/%d %H:%M:%S";

fn main() {
    let (tx, rx) = channel();

    let mut watcher = match watcher(tx, Duration::from_secs(1)) {
        Ok(watcher) => watcher,
        Err(e) => {
            eprintln!("{} Error creating watcher: {:?}", get_formatted_time(), e);
            return;
        }
    };

    if let Err(e) = watcher.watch("/Volumes/T7", RecursiveMode::Recursive) {
        eprintln!("{} Error watching directory: {:?}", get_formatted_time(), e);
        return;
    }

    println!("Watching for changes...");

    loop {
        match rx.recv() {
            Ok(event) => match event {
                DebouncedEvent::Create(path)
                | DebouncedEvent::Write(path)
                | DebouncedEvent::Rename(_, path) => remove_resource_forks(&path),
                _ => (),
            },
            Err(e) => eprintln!("{} Watcher error: {:?}", get_formatted_time(), e),
        }
    }
}

fn remove_resource_forks(folder_path: &Path) {
    if let Some(dir_path) = folder_path.parent() {
        match remove_files_with_prefix(dir_path, "._") {
            Ok(_) => println!(
                "{} Removed resource forks: {:?}",
                get_formatted_time(),
                dir_path
            ),
            Err(err) => eprintln!(
                "{} Failed to remove resource forks: {:?}",
                get_formatted_time(),
                err
            ),
        }
    }
}

fn remove_files_with_prefix(folder_path: &Path, prefix: &str) -> Result<(), std::io::Error> {
    for entry in fs::read_dir(folder_path)? {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
                if file_name.starts_with(prefix) {
                    println!("{} Removing: {:?}", get_formatted_time(), path);
                    fs::remove_file(&path)?;
                }
            }
        }
    }
    Ok(())
}

fn get_formatted_time() -> String {
    let now: DateTime<Local> = Local::now();
    now.format(DATE_FORMAT).to_string()
}
