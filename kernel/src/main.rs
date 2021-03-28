#![no_std]
#![no_main]

extern crate panic_halt;

#[no_mangle]
extern "C" fn kernel_main() -> ! {
    loop {}
}