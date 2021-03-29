#![no_std]
#![no_main]

#![feature(abi_efiapi)]
use uefi::prelude::*;

extern crate rlibc;
use core::fmt::Write;

use uefi::{
    prelude::*,
    proto::media::fs::SimpleFileSystem,
    proto::media::file::{File, FileAttribute, FileInfo, FileMode, FileType},
    table::boot::{AllocateType, MemoryType},
};

const KERNEL_BASE_ADDR: usize = 0x100000;

#[entry]
fn efi_main(_handle: Handle, st: SystemTable<Boot>) -> Status {
    uefi_services::init(&st).unwrap_success();
    st.stdout().reset(false).unwrap_success();
    writeln!(st.stdout(), "Hello, world!").unwrap();

    let boot_services = st.boot_services();

    let fs = boot_services.locate_protocol::<SimpleFileSystem>().unwrap_success();
    let fs = unsafe { &mut *fs.get() };
    let mut root_dir = fs.open_volume().unwrap_success();
 
    let fh = root_dir.open("kernel.efi", FileMode::Read, FileAttribute::READ_ONLY).unwrap_success();
    let file_type = fh.into_type().unwrap_success();
    if let FileType::Regular(mut f) = file_type {
        const BUF_SIZE: usize = 4000;
        let mut buf = [0u8; BUF_SIZE];
        let info: &mut FileInfo = f.get_info(&mut buf).unwrap_success();
        let kernel_file_size: u64 = info.file_size();
        panic!("OK");
    }
    loop {}
}
