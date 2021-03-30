#![feature(asm)]

#![no_std]
#![no_main]

extern crate panic_halt;

#[no_mangle]
extern "C" fn kernel_main(fb_addr: u64, fb_size: u64) -> ! {
    // tmp
    // let fb_addr: u64 = 0x80000000;
    // let fb_size: u64 = 1921024;
    let fb = unsafe { core::slice::from_raw_parts_mut(fb_addr as *mut u8, fb_size as usize) };
    for i in 0..fb_size as usize {
        fb[i] = 255;
    }
    loop {
        unsafe {
            asm!("hlt")
        }
    }
}