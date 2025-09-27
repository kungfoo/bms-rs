use bms_sm::FlightData2;
use bms_sm::RttArea;
use bms_sm::RttExportDone;
use bms_sm::RttTextures;
use image::RgbImage;
use std::time::Duration;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Waiting for event to be available...");
    let mut waiting;
    loop {
        waiting = RttExportDone::new();
        if waiting.is_ok() {
            break;
        }
    }

    // now we can just unwrap on it.
    let rtt_export_done = RttExportDone::new().unwrap();

    let binding = FlightData2::new().unwrap();
    let flight_data2 = binding.read();

    let mut update_count = 0;
    let mut last_report = Instant::now();

    loop {
        rtt_export_done.wait_for_event();

        let textures = RttTextures::read().unwrap();

        let coords = flight_data2.get_rtt_area(RttArea::MfdLeft);
        let _l_mfd = textures.get_image(coords.left, coords.top, coords.right, coords.bottom);

        update_count += 1;

        let now = Instant::now();
        if now.duration_since(last_report) >= Duration::from_secs(1) {
            println!("Updates per second: {}", update_count);
            update_count = 0;
            last_report = now;
        }
    }
}

#[allow(dead_code)]
fn save_as_jpeg(image: &RgbImage, path: &str, quality: u8) -> image::ImageResult<()> {
    use image::codecs::jpeg::JpegEncoder;
    use std::fs::File;
    use std::io::BufWriter;

    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    let mut encoder = JpegEncoder::new_with_quality(writer, quality);
    encoder.encode_image(image)
}
