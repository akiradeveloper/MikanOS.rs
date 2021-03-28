#![no_std]
#![no_main]

#![feature(abi_efiapi)]
use uefi::prelude::*;

use rlibc::*;
use core::fmt::Write;

#[entry]
fn efi_main(_handle: Handle, st: SystemTable<Boot>) -> Status {
    uefi_services::init(&st).expect("Failed to initialize UEFI services");
    st.stdout()
        .reset(false)
        .expect_success("Failed to reset UEFI stdout");
    writeln!(st.stdout(), "Hello world").unwrap();
    loop {}
}
