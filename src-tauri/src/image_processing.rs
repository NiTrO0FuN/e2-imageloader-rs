use crate::compression;
use crate::game_path;
use crate::options::get_options;
use image::{imageops::FilterType, open, EncodableLayout, GenericImageView, ImageError};
use serde::{Deserialize, Serialize};
use std::io;
use std::{fs::create_dir_all, fs::File, io::Write, path::Path};

#[derive(Deserialize)]
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

#[derive(Deserialize)]
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
    ImageNotFound,
    ImageError,
}

#[tauri::command]
pub fn process_image(app: tauri::AppHandle, path: &Path) -> Result<(), Error> {
    let game_data_path = match game_path::get_game_data_path(&app) {
        Some(path) => path,
        _ => return Err(Error::InvalidGamePath),
    };

    let options = get_options(&app);

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

    let size = options.quality.into();

    let img = match open(path) {
        Ok(img) => img,
        Err(ImageError::IoError(e)) if e.kind() == io::ErrorKind::NotFound => return Err(Error::ImageNotFound),
        Err(_) => return Err(Error::ImageError),
    };

    // Crop to square
    let (w, h) = img.dimensions();
    let side = std::cmp::min(w, h);
    let img = img.crop_imm(w / 2 - side / 2, h / 2 - side / 2, side, side);

    // Resize
    let img = img.resize_exact(size, size, FilterType::Nearest);

    let img = img.to_rgb8();

    // Compress
    let data = compression::compress(img.clone());
    let use_compress = data.len() < img.len();

    let to_save = if use_compress { data } else { img.as_bytes().to_vec() };

    let hex_string: String = to_save.iter().map(|b| format!("{:02X}", b)).collect();

    let chunks = hex_string.as_bytes().chunks(options.chunk_size.into());
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
