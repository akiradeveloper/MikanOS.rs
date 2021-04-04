use mikan::{FrameBufferConfig, PixelFormat};
#[derive(Clone, Copy)]
pub struct PixelColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

fn write_pixel_triple(fb_config: &FrameBufferConfig, x: u32, y: u32, abc: (u8,u8,u8)) {
    let pixel_pos = fb_config.pixels_per_scan_line * y + x;
    let (c0,c1,c2) = abc;
    let fb_addr = fb_config.frame_buffer;
    let fb_size = 4 * fb_config.pixels_per_scan_line * fb_config.vertical_resolution;
    let fb = unsafe { core::slice::from_raw_parts_mut(fb_addr, fb_size as usize) };
    let base = 4 * pixel_pos as usize;
    fb[base+0] = c0;
    fb[base+1] = c1;
    fb[base+2] = c2;
}

pub fn write_pixel(x: u32, y: u32, color: PixelColor) {
    let fb_config = unsafe { crate::G_CONTEXT.fb_config() };
    let (c0,c1,c2) = match fb_config.pixel_format {
        PixelFormat::RGBResv8BitPerColor => {
            (color.g, color.b, color.r)
        },
        PixelFormat::BGRResv8BitPerColor => {
            (color.b, color.g, color.r)
        },
    };
    write_pixel_triple(&fb_config, x, y, (c0, c1, c2));
}

pub fn write_ascii(c: char, x: u32, y: u32, color: PixelColor) {
    use crate::fonts::ascii_map;
    let ascii_map = ascii_map(c);
    for dy in 0..16u32 {
        let scan = ascii_map[dy as usize];
        for dx in 0..8 {
            if (scan << dx) & 0x80 > 0 {
                write_pixel(x+dx, y+dy, color);
            }
        }
    }
}