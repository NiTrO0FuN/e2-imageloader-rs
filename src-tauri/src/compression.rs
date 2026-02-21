use image::{ImageBuffer, Rgb};

pub fn compress(img: ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<u8> {
    let mut result = Vec::new();

    let mut current_color: Option<&Rgb<u8>> = None;
    let mut count = 1;
    for pixel in img.enumerate_pixels() {
        let pixel_color = pixel.2;
        match current_color {
            Some(color) if color == pixel_color && count < u8::MAX => count += 1,
            Some(color) => {
                result.push(count);
                result.extend_from_slice(&color.0);
                current_color = Some(pixel_color);
                count = 1
            }
            _ => current_color = Some(pixel_color),
        }
    }
    result.push(count);
    result.extend_from_slice(&current_color.unwrap().0);
    result
}

#[test]
fn test_no_compression() {
    let data = [
        0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    ];

    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_vec(5, 1, data.to_vec()).unwrap();

    assert_eq!(
        compress(img),
        vec![
            0x01, 0xFF, 0x00, 0x00, 0x01, 0x00, 0xFF, 0x00, 0x01, 0x00, 0x00, 0xFF, 0x01, 0x00, 0xFF, 0xFF, 0x01, 0xFF,
            0xFF, 0xFF,
        ]
    )
}

#[test]
fn test_compression() {
    let data = [
        0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00, 0xFF,
    ];

    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_vec(5, 1, data.to_vec()).unwrap();

    assert_eq!(compress(img), vec![0x01, 0xFF, 0x00, 0x00, 0x04, 0x00, 0x00, 0xFF])
}

#[test]
fn test_chunk_compression() {
    let data = [0; 300 * 3];

    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_vec(300, 1, data.to_vec()).unwrap();

    assert_eq!(compress(img), vec![0xFF, 0x00, 0x00, 0x00, 0x2D, 0x00, 0x00, 0x00]);
}

#[test]
fn test_roundtrip() {
    fn decompress(data: &[u8], width: u32, height: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut pixels = Vec::new();

        for i in (0..data.len()).step_by(4) {
            let count = data[i];
            for _ in 0..count {
                pixels.extend_from_slice(&data[i + 1..i + 4]);
            }
        }

        ImageBuffer::from_vec(width, height, pixels).unwrap()
    }

    let img = image::open("tests/test.png").expect("Failed to load test.png");
    let img = img.to_rgb8();
    let (width, height) = img.dimensions();

    let compressed = compress(img.clone());
    let decompressed = decompress(&compressed, width, height);

    assert_eq!(img, decompressed, "Decompressed image should match original");
}
