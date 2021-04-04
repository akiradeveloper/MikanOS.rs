#![no_std]
#![no_main]

#![feature(asm)]
#![feature(abi_efiapi)]

extern crate panic_halt;
mod fonts;

use mikan::{FrameBufferConfig, PixelFormat};

struct Context {
    fb_config: Option<FrameBufferConfig>,
}
impl Context {
    fn fb_config(&self) -> &FrameBufferConfig {
        self.fb_config.as_ref().unwrap()
    }
}
static mut G_CONTEXT: Context = Context {
    fb_config: None,
};

#[derive(Clone, Copy)]
struct PixelColor {
    r: u8,
    g: u8,
    b: u8,
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

fn write_pixel(x: u32, y: u32, color: PixelColor) {
    let fb_config = unsafe { G_CONTEXT.fb_config() };
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

#[no_mangle]
extern "efiapi" fn kernel_main(fb_config: FrameBufferConfig) -> ! {
    unsafe { G_CONTEXT.fb_config = Some(fb_config); }

    for x in 0..fb_config.horizontal_resolution {
        for y in 0..fb_config.vertical_resolution {
            write_pixel( x, y, PixelColor { r: 255, g: 255, b: 255 });
        }
    }
    for x in 0..200 {
        for y in 0..100 {
            write_pixel( 100+x, 100+y, PixelColor { r: 0, g: 0, b: 255 });
        }
    }
    loop {
        unsafe {
            asm!("hlt")
        }
    }
}