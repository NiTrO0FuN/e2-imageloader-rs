use crate::image_processing::{ChunkSize, Quality};
use serde::Deserialize;
use tauri_plugin_store::StoreExt;

#[derive(Deserialize)]
pub struct Options {
    pub quality: Quality,
    #[serde(rename(deserialize = "chunkSize"))]
    pub chunk_size: ChunkSize,
    pub automode: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            quality: Quality::High,
            chunk_size: ChunkSize::Medium,
            automode: false,
        }
    }
}

pub fn get_options(app: &tauri::AppHandle) -> Options {
    let store = match app.store("settings.json") {
        Ok(store) => store,
        Err(_) => return Default::default(),
    };

    store
        .get("options")
        .and_then(|options| serde_json::from_value::<Options>(options).ok())
        .unwrap_or_default()
}
