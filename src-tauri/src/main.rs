#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod compression;
mod game_path;
mod image_processing;
mod options;
mod watcher;

use game_path::{guess_game_path, is_valid_game_path};
use image_processing::process_image;

use watcher::start_screenshots_watcher;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![is_valid_game_path, process_image])
        .setup(|app| {
            let _ = guess_game_path(app.handle()).map_err(|_| println!("Error while trying to guess Garry's Mod path"));
            start_screenshots_watcher(app.handle());
            Ok(())
        })
        .run(tauri::generate_context!())
        .unwrap();
}
