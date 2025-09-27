mod flight_data;
mod flight_data2;
mod intellivibe_data;
mod rtt_export_event;
mod rtt_textures;
mod string_data;

pub use flight_data::FlightData;
pub use flight_data2::FlightData2;
pub use intellivibe_data::IntellivibeData;
pub use rtt_export_event::RttExportDone;
pub use rtt_textures::RttTextures;
pub use string_data::*;

use std::mem::size_of;

use windows::{
    core::HSTRING,
    Win32::{
        Foundation::{CloseHandle, HANDLE},
        System::Memory::{
            MapViewOfFile, OpenFileMappingW, UnmapViewOfFile, FILE_MAP_READ,
            MEMORY_MAPPED_VIEW_ADDRESS,
        },
    },
};

pub struct MemoryFile<'a, T> {
    data: &'a T,
    map: HANDLE,
    p_buf: MEMORY_MAPPED_VIEW_ADDRESS,
}

// I need to know how much of a bad idea this is...
unsafe impl<'a, T> Send for MemoryFile<'a, T> {}
unsafe impl<'a, T> Sync for MemoryFile<'a, T> {}

impl<'a, T> MemoryFile<'a, T>
where
    T: Default,
{
    /// # Safety
    /// If the generic struct given does not match the memory layout,
    /// this is probably going to make shit hit the fan
    /// Do not use directly unless you know what you're doing.
    pub unsafe fn new(name: &'a str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        Self::new_with_size(name, size_of::<T>())
    }

    /// # Safety
    /// If the generic struct given does not match the memory layout,
    /// this is probably going to make shit hit the fan
    /// Do not use directly unless you know what you're doing.
    pub unsafe fn new_with_offset(
        name: &'a str,
        offset: usize,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        Self::new_with_offset_and_size(name, offset, size_of::<T>())
    }

    /// # Safety
    /// If the generic struct given does not match the memory layout,
    /// this is probably going to make shit hit the fan
    /// Do not use directly unless you know what you're doing.
    pub unsafe fn new_with_size(
        name: &'a str,
        size: usize,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        Self::new_with_offset_and_size(name, 0, size)
    }

    /// # Safety
    /// If the generic struct given does not match the memory layout,
    /// this is probably going to make shit hit the fan
    /// Do not use directly unless you know what you're doing.
    pub unsafe fn new_with_offset_and_size(
        name: &'a str,
        offset: usize,
        size: usize,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let hname: HSTRING = HSTRING::from(name);
        let map = OpenFileMappingW(FILE_MAP_READ.0, false, &hname)?;

        let p_buf = MapViewOfFile(map, FILE_MAP_READ, 0, 0, 0);

        let p = p_buf.Value.add(offset);
        let ptr: *const u8 = p as *const u8;

        let slice = std::slice::from_raw_parts(ptr, size);

        let data = &slice.align_to::<T>().1;
        if data.is_empty() {
            let unaligned = std::ptr::addr_of!(slice);
            let data: &T = std::ptr::read_unaligned(unaligned as *const _);
            return Ok(Self { data, map, p_buf });
        }
        Ok(Self {
            data: &data[0],
            map,
            p_buf,
        })
    }

    pub fn read(&self) -> &T {
        self.data
    }
}

impl<'a, T> Drop for MemoryFile<'a, T> {
    fn drop(&mut self) {
        let _ = unsafe { CloseHandle(self.map) };
        let _ = unsafe { UnmapViewOfFile(self.p_buf) };
    }
}

pub struct RawMemoryFile<'a> {
    data: &'a [u8],
    map: HANDLE,
    p_buf: MEMORY_MAPPED_VIEW_ADDRESS,
}

// I need to know how much of a bad idea this is...
unsafe impl<'a> Send for RawMemoryFile<'a> {}
unsafe impl<'a> Sync for RawMemoryFile<'a> {}

impl<'a> RawMemoryFile<'a> {
    /// # Safety
    /// If the generic struct given does not match the memory layout,
    /// this is probably going to make shit hit the fan
    /// Do not use directly unless you know what you're doing.
    pub unsafe fn new_with_size(
        name: &'a str,
        size: usize,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        Self::new_with_offset_and_size(name, 0, size)
    }

    /// # Safety
    /// If the generic struct given does not match the memory layout,
    /// this is probably going to make shit hit the fan
    /// Do not use directly unless you know what you're doing.
    pub unsafe fn new_with_offset_and_size(
        name: &'a str,
        offset: usize,
        size: usize,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let hname: HSTRING = HSTRING::from(name);
        let map = OpenFileMappingW(FILE_MAP_READ.0, false, &hname)?;

        let p_buf = MapViewOfFile(map, FILE_MAP_READ, 0, 0, 0);

        let p = p_buf.Value.add(offset);
        let ptr: *const u8 = p as *const u8;

        let slice = std::slice::from_raw_parts(ptr, size);

        Ok(Self {
            data: slice,
            map,
            p_buf,
        })
    }

    pub fn read(&self) -> &'a [u8] {
        self.data
    }
}

impl<'a> Drop for RawMemoryFile<'a> {
    fn drop(&mut self) {
        let _ = unsafe { CloseHandle(self.map) };
        let _ = unsafe { UnmapViewOfFile(self.p_buf) };
    }
}
