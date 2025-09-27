use bms_sm::RttExportDone;
use bms_sm::RttTextures;
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

    let mut update_count = 0;
    let mut last_report = Instant::now();

    loop {
        rtt_export_done.wait_for_event();

        let _textures = RttTextures::read().unwrap();

        update_count += 1;

        let now = Instant::now();
        if now.duration_since(last_report) >= Duration::from_secs(1) {
            println!("Updates per second: {}", update_count);
            update_count = 0;
            last_report = now;
        }
    }
}
