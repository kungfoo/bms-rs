# BMS Shared Memory


[![Crates.io][crates-badge]][crates-url]

[crates-badge]: https://img.shields.io/crates/v/bms-sm.svg
[crates-url]: https://crates.io/crates/bms-sm

Read BMS shared memory data

Quick example:
```rs
use bms_sm::*;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Wait until the memory-file exists
    let mut mem = IntellivibeData::new();
    loop {
        if mem.is_ok() {
            break;
        }
        sleep(Duration::from_millis(300)).await;
        mem = IntellivibeData::new();
    }
    let mem = mem?;

    // Threadable
    tokio::spawn(async move {
        loop {
            // Read the memory as it is right now
            dbg!(mem.read());
            sleep(Duration::from_secs(5)).await;
        }
    });

    tokio::signal::ctrl_c().await?;

    Ok(())
}
```
    
Currently implemented interfaces:
 - `IntellivibeData` ("FalconIntellivibeSharedMemoryArea")
 - `FlightData` ("FalconSharedMemoryArea")
 - `StringsData` ("FalconSharedMemoryAreaString")
 - `FlightData2` ("FalconSharedMemoryArea2")

