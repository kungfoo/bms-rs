use bms_sm::RttTextures;

use image::{ImageBuffer, RgbImage};
use std::fs::File;
use std::io::BufWriter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let textures = RttTextures::read().unwrap();

    let output = File::create("output.jpeg")?;
    let writer = BufWriter::new(output);

    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(writer, 90); // 0–100 quality
    encoder.encode_image(&textures.image)?;

    Ok(())
}
