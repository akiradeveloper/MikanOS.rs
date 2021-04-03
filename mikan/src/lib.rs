#![no_std]

#[derive(Debug)]
pub enum PixelFormat {
    RGBResv8BitPerColor,
    BGRResv8BitPerColor,
}

#[derive(Debug)]
pub struct FrameBufferConfig {
    pub frame_buffer: *mut u8,
    pub pixels_per_scan_line: u32,
    pub horizontal_resolution: u32,
    pub vertical_resolution: u32,
    pub pixel_format: PixelFormat,
}