use crate::game_path::get_steam_screenshots_dir;
use crate::image_processing::{process_image, ChunkSize, Quality};
use notify::{EventKind, RecursiveMode, Watcher};
use std::{path::PathBuf, thread, time::Duration};

use tauri::{AppHandle, Emitter};
use tauri_plugin_store::StoreExt;

pub fn start_screenshots_watcher(app: &tauri::AppHandle) {
    let Some(screenshots_path) = get_steam_screenshots_dir(&app) else {
        println!("No Steam screenshots directory found for watcher");
        return;
    };

    if !screenshots_path.exists() {
        println!("Steam screenshots directory does not exist: {:?}", screenshots_path);
        return;
    }

    let app = app.clone();
    thread::spawn(move || {
        let store = app.store("settings.json").unwrap();
        let mut watcher = match notify::recommended_watcher(move |result: notify::Result<notify::Event>| match result {
            Ok(event) if store.get("automode").and_then(|value| value.as_bool()).unwrap_or(false) => {
                if !matches!(event.kind, EventKind::Create(_)) {
                    return;
                }

                for path in event.paths {
                    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or_default();
                    if path.is_file()
                        && (extension.eq_ignore_ascii_case("jpg") || extension.eq_ignore_ascii_case("jpeg"))
                    {
                        safe_process(&app, path);
                    }
                }
            }
            Ok(_) => {}
            Err(err) => eprintln!("Failed to watch screenshots directory: {:?}", err),
        }) {
            Ok(watcher) => watcher,
            Err(err) => {
                eprintln!("Failed to initialize screenshot watcher: {:?}", err);
                return;
            }
        };

        if let Err(err) = watcher.watch(&screenshots_path, RecursiveMode::NonRecursive) {
            eprintln!("Failed to watch {:?}: {:?}", screenshots_path, err);
            return;
        }

        loop {
            thread::park();
        }
    });
}

fn safe_process(app: &AppHandle, path: PathBuf) {
    thread::sleep(Duration::from_millis(250));
    for _ in 1..=3 {
        match process_image(app.clone(), path.as_path(), Quality::Medium, ChunkSize::Medium) {
            Ok(_) => {
                app.emit("automode", "success").unwrap();
                return;
            }
            Err(_) => thread::sleep(Duration::from_millis(250)),
        }
    }
    app.emit("automode", "error").unwrap();
}
