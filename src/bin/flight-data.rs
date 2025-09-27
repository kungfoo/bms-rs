use bms_sm::{FlightData, FlightData2};

fn main() {
    let binding = FlightData::new().unwrap();
    let data = binding.read();
    dbg!(data);

    let binding = FlightData2::new().unwrap();
    let data = binding.read();
    dbg!(data);
}
