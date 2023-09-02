use anyhow::Result;
use chrono::{DateTime, Local};
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::fs;
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

const DATE_FORMAT: &str = "%Y/%m/%d %H:%M:%S";

fn main() -> Result<()> {
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(1))?;
    watcher.watch("/Volumes/T7", RecursiveMode::Recursive)?;
    println!("Watching for changes...");

    for event in rx.iter() {
        if let Err(e) = handle_event(event) {
            eprintln!("{} Error handling event: {:?}", get_formatted_time(), e);
        }
    }

    Ok(())
}

fn handle_event(event: DebouncedEvent) -> Result<()> {
    if let DebouncedEvent::Create(path)
    | DebouncedEvent::Write(path)
    | DebouncedEvent::Rename(_, path) = event
    {
        if let Some(dir_path) = path.parent() {
            let prefix = "._";
            if fs::read_dir(dir_path)?.filter_map(|e| e.ok()).any(|e| {
                e.file_name()
                    .to_str()
                    .map(|s| s.starts_with(prefix))
                    .unwrap_or(false)
            }) {
                remove_files_with_prefix(dir_path, prefix)?;
            }
        }
    }
    Ok(())
}

fn remove_files_with_prefix(folder_path: &Path, prefix: &str) -> Result<()> {
    fs::read_dir(folder_path)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_name()
                .to_str()
                .map(|s| s.starts_with(prefix))
                .unwrap_or(false)
        })
        .map(|e| e.path())
        .try_for_each(|path| {
            println!("{} Removing: {:?}", get_formatted_time(), path);
            fs::remove_file(&path)
        })?;
    Ok(())
}

fn get_formatted_time() -> String {
    Local::now().format(DATE_FORMAT).to_string()
}
