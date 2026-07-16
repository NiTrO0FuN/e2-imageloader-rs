use crate::game_path::get_steam_screenshots_dir;
use crate::image_processing::process_image;
use crate::options::get_options;
use notify::{EventKind, RecursiveMode, Watcher};
use std::{path::PathBuf, thread, time::Duration};
use tauri::{AppHandle, Emitter};

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
        let mut watcher = match notify::recommended_watcher(move |result: notify::Result<notify::Event>| {
            let options = get_options(&app);
            match result {
                Ok(event) if options.automode => {
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
            }
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
        match process_image(app.clone(), path.as_path()) {
            Ok(_) => {
                app.emit("automode", "success").unwrap();
                return;
            }
            Err(_) => thread::sleep(Duration::from_millis(250)),
        }
    }
    app.emit("automode", "error").unwrap();
}
