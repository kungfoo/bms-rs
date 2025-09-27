use std::{ffi::OsStr, os::windows::ffi::OsStrExt};
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{CloseHandle, HANDLE, WAIT_OBJECT_0, WAIT_TIMEOUT},
        System::Threading::{
            OpenEventW, WaitForSingleObject, EVENT_MODIFY_STATE, SYNCHRONIZATION_SYNCHRONIZE,
        },
    },
};

/// This is the 4.38 way of checking whether shared texture memory has been written.
/// Once you get an instance of this (may fail of BMS is not running), you can use
/// it to block until new data has been written using `wait_for_event()`.
pub struct RttExportDone {
    handle: HANDLE,
}

impl RttExportDone {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let event_name = to_wide("BMS_RTTExport_Done");

        let handle = unsafe {
            OpenEventW(
                SYNCHRONIZATION_SYNCHRONIZE | EVENT_MODIFY_STATE, // desired access
                false,                                            // inherit handle
                PCWSTR::from_raw(event_name.as_ptr()),
            )
        };

        match handle {
            Ok(handle) => Ok(RttExportDone { handle }),
            Err(e) => Err(Box::new(e)),
        }
    }

    /// Will wait (blocking) for the event to be flagged by the BMS process.
    pub fn wait_for_event(&self) {
        let wait_result = unsafe { WaitForSingleObject(self.handle, u32::MAX) };

        match wait_result {
            WAIT_OBJECT_0 => {
                // intentionally left blank
            }
            WAIT_TIMEOUT => {
                eprintln!("Timed out while waiting for the RTT export done event.");
            }
            _ => eprintln!("Unexpected wait result: {:?}", wait_result),
        }
    }
}

impl Drop for RttExportDone {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.handle).expect("Failed to close handle") };
    }
}

fn to_wide(s: &str) -> Vec<u16> {
    OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}
