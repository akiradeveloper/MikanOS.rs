#![no_std]
#![no_main]
#![feature(abi_efiapi)]
use uefi::prelude::*;

extern crate rlibc;
use core::fmt::Write;

use uefi::{
    prelude::*,
    proto::media::file::{File, FileAttribute, FileInfo, FileMode, FileType},
    proto::media::fs::SimpleFileSystem,
    table::boot::{AllocateType, MemoryType},
};


#[entry]
fn efi_main(handle: Handle, st: SystemTable<Boot>) -> Status {
    uefi_services::init(&st).unwrap_success();
    st.stdout().reset(false).unwrap_success();
    writeln!(st.stdout(), "Hello, world!").unwrap();

    let boot_services = st.boot_services();

    let fs = boot_services
        .locate_protocol::<SimpleFileSystem>()
        .unwrap_success();
    let fs = unsafe { &mut *fs.get() };
    let mut root_dir = fs.open_volume().unwrap_success();

    let fh = root_dir
        .open("kernel.elf", FileMode::Read, FileAttribute::READ_ONLY)
        .unwrap_success();
    let file_type = fh.into_type().unwrap_success();
    if let FileType::Regular(mut f) = file_type {
        const TMP_BUF_SIZE: usize = 4000;
        let mut tmp_buf = [0u8; TMP_BUF_SIZE];
        let info: &mut FileInfo = f.get_info(&mut tmp_buf).unwrap_success();
        let kernel_file_size: u64 = info.file_size();
        writeln!(st.stdout(), "kernel_file_size={}", kernel_file_size).unwrap();

        const KERNEL_BASE_ADDR: usize = 0x100000;
        let n_pages = (kernel_file_size as usize + 0xfff) / 0x1000;
        let p = boot_services
            .allocate_pages(
                AllocateType::Address(KERNEL_BASE_ADDR),
                MemoryType::LOADER_DATA,
                n_pages,
            )
            .unwrap_success();
        let buf = unsafe { core::slice::from_raw_parts_mut(p as *mut u8, kernel_file_size as usize) };
        // Read kernel file into the memory
        f.read(buf).unwrap_success();
        writeln!(st.stdout(), "kernel is read into the memory").unwrap();

        st.exit_boot_services(handle, &mut tmp_buf).unwrap_success();
    }
    loop {}
}
