#![no_std]
#![no_main]

#![feature(abi_efiapi)]

extern crate rlibc;
use core::fmt::Write;
use byteorder::{ByteOrder, LittleEndian};

use uefi::{
    prelude::*,
    proto::media::file::{File, FileAttribute, FileInfo, FileMode, FileType},
    proto::media::fs::SimpleFileSystem,
    proto::console::gop::GraphicsOutput,
    table::boot::{AllocateType, MemoryType},
};

#[entry]
fn efi_main(handle: Handle, st: SystemTable<Boot>) -> Status {
    uefi_services::init(&st).unwrap_success();
    st.stdout().reset(false).unwrap_success();
    writeln!(st.stdout(), "Hello, world!").unwrap();

    let boot_services = st.boot_services();

    // Get Framebuffer
    let go = boot_services.locate_protocol::<GraphicsOutput>().unwrap_success();
    let go = unsafe { &mut *go.get() };
    let mut fb = go.frame_buffer();
    let fb_addr = fb.as_mut_ptr() as u64;
    let fb_size = fb.size();
    writeln!(st.stdout(), "fb_addr={:x}, fb_size={}", fb_addr, fb_size).unwrap();

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

        const KERNEL_BASE_ADDR: usize = 0x100000;
        let n_pages = (kernel_file_size as usize + 0xfff) / 0x1000;
        let p = boot_services
            .allocate_pages(
                AllocateType::Address(KERNEL_BASE_ADDR),
                MemoryType::LOADER_DATA,
                n_pages,
            )
            .unwrap_success();

        // Read kernel file into the memory
        let buf = unsafe { core::slice::from_raw_parts_mut(p as *mut u8, kernel_file_size as usize) };
        f.read(buf).unwrap_success();
        f.close();
        writeln!(st.stdout(), "kernel is read into the memory").unwrap();

        // Entry address should be found at +24
        let buf = unsafe { core::slice::from_raw_parts((p + 24) as *mut u8, 8)};
        let kernel_main_addr = LittleEndian::read_u64(&buf);
        writeln!(st.stdout(), "kernel_main address = {:x}", kernel_main_addr).unwrap();

        st.exit_boot_services(handle, &mut tmp_buf).unwrap_success();

        let kernel_main = unsafe {
            let f: extern "C" fn(u64, u64) -> ! = core::mem::transmute(kernel_main_addr);
            f
        };
        kernel_main(fb_addr, fb_size as u64);
    }
    loop {}
}
