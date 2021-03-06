#![no_std]
#![no_main]

#![feature(asm)]
#![feature(abi_efiapi)]


extern crate rlibc;
extern crate panic_halt;
mod fonts;
mod console;
mod graphics;
mod pci;
mod mouse;

use core::fmt::Write;
use mikan::{FrameBufferConfig, PixelFormat};
use graphics::{write_pixel, write_string, PixelColor};
use console::{Console};

struct Context {
    fb_config: Option<FrameBufferConfig>,
    console: Console,
}
impl Context {
    fn fb_config(&self) -> &FrameBufferConfig {
        self.fb_config.as_ref().unwrap()
    }
}
static mut G_CONTEXT: Context = Context {
    fb_config: None,
    console: Console::new(PixelColor { r: 0, b: 0, g: 0 }, PixelColor { r: 255, b: 255, g: 255}),
};

#[no_mangle]
extern "efiapi" fn kernel_main(fb_config: FrameBufferConfig) -> ! {
    unsafe { G_CONTEXT.fb_config = Some(fb_config); }
    unsafe { G_CONTEXT.console.clear(); }

    let mut bus_scan = pci::ScanPciDevices::new();
    bus_scan.scan_devices().unwrap();
    for i in 0..bus_scan.num_devices {
        let dev = bus_scan.result[i];
        let config = dev.config;
        printk!("{}.{}.{} vend:0x{:x} devid:0x{:x} base:0x{:x} sub:0x{:x} interface:0x{:x}",
        dev.bus, dev.device, dev.function, config.vendor_id, config.device_id, config.base_class, config.sub_class, config.interface);
    }

    // for x in 0..fb_config.horizontal_resolution {
    //     for y in 0..fb_config.vertical_resolution {
    //         write_pixel( x, y, PixelColor { r: 255, g: 255, b: 255 });
    //     }
    // }
    // for x in 0..200 {
    //     for y in 0..100 {
    //         write_pixel( 100+x, 100+y, PixelColor { r: 0, g: 0, b: 255 });
    //     }
    // }
    // write_string("akira developer", 300, 300, PixelColor { r: 255, g: 0, b: 0 });
    loop {
        unsafe {
            asm!("hlt")
        }
    }
}