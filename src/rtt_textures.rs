use crate::{MemoryFile, RawMemoryFile};
use image::imageops::crop_imm;
use image::RgbImage;

use windows::Win32::Graphics::DirectDraw::DDSURFACEDESC2;

#[repr(C)]
#[derive(Default)]
pub struct RttTextureAreaHeader {
    pub dd_surface_desc: DDSURFACEDESC2,
}

/// If you manage to get an instance of this (i.e. BMS is running and exporting),
/// you can use it to read shared texture memory.
/// Either the entire area, or subimages thereof
/// by using `get_image()`.
#[derive(Debug)]
pub struct RttTextures {
    pub image: RgbImage,
}


impl RttTextures {
    pub fn get_image(&self, left: u16, top: u16, right: u16, bottom: u16) -> RgbImage {
        let width = (right - left) as u32;
        let height = (bottom - top) as u32;
        crop_imm(&self.image, left as u32, top as u32, width, height).to_image()
    }
}

impl RttTextures {
    pub fn read() -> Result<RttTextures, Box<dyn std::error::Error + Send + Sync>> {
        let header = unsafe {
            MemoryFile::<RttTextureAreaHeader>::new_with_offset(
                "FalconTexturesSharedMemoryArea",
                4,
            )?
        };
        let header = header.read();

        let height = header.dd_surface_desc.dwHeight as usize;
        let width = header.dd_surface_desc.dwWidth as usize;
        let pitch = unsafe { header.dd_surface_desc.Anonymous1.lPitch } as usize;
        let total_size = pitch * height;

        let offset = 4 + header.dd_surface_desc.dwSize as usize;

        let data = unsafe {
            RawMemoryFile::new_with_offset_and_size(
                "FalconTexturesSharedMemoryArea",
                offset,
                total_size,
            )?
        };

        let data = data.read();

        Ok(RttTextures {
            image: Self::extract_rgb_from_bgra_surface(data, width, height, pitch),
        })
    }

    fn extract_rgb_from_bgra_surface(
        raw: &[u8],
        width: usize,
        height: usize,
        pitch: usize,
    ) -> RgbImage {
        let mut rgb_data = Vec::with_capacity(width * height * 3);

        for y in 0..height {
            let row_start = y * pitch;
            // we simply assume 32bits BGRA as is normal in modern DirectDraw
            let row = &raw[row_start..row_start + width * 4]; // 4 bytes per pixel

            for px in row.chunks_exact(4) {
                // BGRA -> RGB
                rgb_data.extend_from_slice(&[px[2], px[1], px[0]]);
            }
        }

        RgbImage::from_raw(width as u32, height as u32, rgb_data).expect("Invalid image buffer")
    }
}
