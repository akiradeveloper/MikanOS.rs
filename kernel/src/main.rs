#![no_std]
#![no_main]

#![feature(asm)]
#![feature(abi_efiapi)]

extern crate panic_halt;
mod fonts;
mod graphics;

use mikan::{FrameBufferConfig, PixelFormat};
use graphics::{write_pixel, write_string, PixelColor};

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
    write_string("akiradeveloper", 300, 300, PixelColor { r: 255, g: 0, b: 0 });
    loop {
        unsafe {
            asm!("hlt")
        }
    }
}