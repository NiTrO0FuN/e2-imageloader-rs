use crate::compression;
use crate::game_path;

use image::{self, imageops::FilterType, EncodableLayout};
use serde::Serialize;
use std::{fs::create_dir_all, fs::File, io::Write, path::Path};

#[derive(serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChunkSize {
    Small,
    Medium,
    Large,
}

impl Into<usize> for ChunkSize {
    fn into(self) -> usize {
        match self {
            ChunkSize::Small => 100_000,
            ChunkSize::Medium => 300_000,
            ChunkSize::Large => 500_000,
        }
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Quality {
    Low,
    Medium,
    High,
}

impl Into<u32> for Quality {
    fn into(self) -> u32 {
        match self {
            Quality::Low => 128,
            Quality::Medium => 256,
            Quality::High => 512,
        }
    }
}

#[derive(Debug, Serialize)]
pub enum Error {
    InvalidGamePath,
}

#[tauri::command]
pub fn process_image(app: tauri::AppHandle, path: &Path, quality: Quality, chunk_size: ChunkSize) -> Result<(), Error> {
    let game_data_path = match game_path::get_game_data_path(app) {
        Some(path) => path,
        _ => return Err(Error::InvalidGamePath),
    };

    create_dir_all(&game_data_path).map_err(|_| Error::InvalidGamePath)?;
    for entry in std::fs::read_dir(&game_data_path).map_err(|_| Error::InvalidGamePath)? {
        let entry = entry.map_err(|_| Error::InvalidGamePath)?;
        let path = entry.path();
        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                if file_name.starts_with("data") && file_name.ends_with(".txt") {
                    let _ = std::fs::remove_file(path);
                }
            }
        }
    }

    let size = quality.into();

    let img = image::open(path).unwrap();
    let img = img.resize_exact(size, size, FilterType::Nearest);
    let img = img.to_rgb8();

    // Compress
    let data = compression::compress(img.clone());
    let use_compress = data.len() < img.len();

    let to_save = if use_compress { data } else { img.as_bytes().to_vec() };

    let hex_string: String = to_save.iter().map(|b| format!("{:02X}", b)).collect();

    let chunks = hex_string.as_bytes().chunks(chunk_size.into());
    let n: usize = chunks.len();

    for (i, chunk) in chunks.enumerate() {
        let file_name = format!("data{i}.txt");
        let mut file = File::create(game_data_path.join(file_name)).unwrap();

        // Write compression header
        write!(file, "{}", if use_compress { "1" } else { "0" }).unwrap();

        // Write size header
        write!(file, "{:03X}", size).unwrap();

        // Write #files header
        write!(file, "{:02X}", n).unwrap();

        // Write pixel data
        file.write_all(chunk).unwrap();
    }
    println!("Wrote {size}x{size} image in {n} files, compressed: {use_compress}");

    Ok(())
}
