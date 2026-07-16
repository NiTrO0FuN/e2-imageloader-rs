use std::fs;
use std::io::Error;
use std::path::{Path, PathBuf};

use tauri_plugin_store::StoreExt;

fn has_gmod(path: &Path) -> Result<bool, Error> {
    for entry in fs::read_dir(path)? {
        let name = entry?.file_name();
        if name == "gmod" || name == "gmod.exe" {
            return Ok(true);
        }
    }

    Ok(false)
}

const POSSIBLE_GAME_PATHS: &[&str] = if cfg!(target_os = "windows") {
    &[
        "C:\\Program Files\\Steam\\steamapps\\common\\GarrysMod",
        "C:\\Program Files (x86)\\Steam\\steamapps\\common\\GarrysMod",
    ]
} else if cfg!(target_os = "linux") {
    &[
        "~/.steam/steam/steamapps/common/GarrysMod",
        "~/.local/share/Steam/steamapps/common/GarrysMod",
    ]
} else if cfg!(target_os = "macos") {
    &["~/Library/Application Support/Steam/steamapps/common/GarrysMod"]
} else {
    &[]
};

pub fn guess_game_path(app: &tauri::AppHandle) -> tauri_plugin_store::Result<()> {
    let store = app.store("settings.json")?;

    if let Some(existing_path) = store.get("gamePath") {
        if let Some(path_str) = existing_path.as_str() {
            if is_valid_game_path(Path::new(path_str)) {
                return Ok(());
            }
        }
    }

    for &path_str in POSSIBLE_GAME_PATHS {
        if is_valid_game_path(Path::new(path_str)) {
            store.set("gamePath", path_str);
            break;
        }
    }

    Ok(())
}

const DATA_PATH: &str = "garrysmod/data/e2files/e2imageloader";

pub fn get_game_data_path(app: &tauri::AppHandle) -> Option<PathBuf> {
    let store = app.store("settings.json").ok()?;

    store
        .get("gamePath")
        .and_then(|path| path.as_str().map(PathBuf::from))
        .and_then(|path| is_valid_game_path(&path).then_some(path))
        .map(|path| path.join(DATA_PATH))
}

fn find_steam_root_from_game_path(path: &Path) -> Option<PathBuf> {
    path.ancestors().find_map(|ancestor| {
        ancestor
            .file_name()
            .and_then(|name| name.to_str())
            .filter(|name| name.eq_ignore_ascii_case("steam"))
            .map(|_| ancestor.to_path_buf())
    })
}

pub fn get_steam_screenshots_dir(app: &tauri::AppHandle) -> Option<PathBuf> {
    let store = app.store("settings.json").ok()?;
    let game_path_value = store.get("gamePath")?;
    let game_path = game_path_value.as_str()?;
    let steam_root = find_steam_root_from_game_path(Path::new(game_path))?;
    let userdata_dir = steam_root.join("userdata");

    // Can be multiple steamd ids
    let mut entries = fs::read_dir(&userdata_dir).ok()?;
    let first = entries.next()?.ok()?.path();

    let screenshots_path = first.join("760").join("remote").join("4000").join("screenshots");
    screenshots_path.is_dir().then_some(screenshots_path)
}

#[tauri::command]
pub fn is_valid_game_path(path: &Path) -> bool {
    has_gmod(path).unwrap_or(false)
}
