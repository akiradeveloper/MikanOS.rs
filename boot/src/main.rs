#![no_std]
#![no_main]

#![feature(abi_efiapi)]

use mikan::{FrameBufferConfig};

extern crate rlibc;
use core::fmt::Write;
use byteorder::{ByteOrder, LittleEndian};

use uefi::{
    prelude::*,
    proto::media::file::{File, FileAttribute, FileInfo, FileMode, FileType},
    proto::media::fs::SimpleFileSystem,
    proto::console::gop::{GraphicsOutput, PixelFormat},
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
    let fb_addr = fb.as_mut_ptr();
    let fb_size = fb.size();
    let info = go.current_mode_info();
    let (hori, vert) = info.resolution();
    let pixels_per_scan_line = info.stride() as u32;
    let pixel_format = match info.pixel_format() {
        PixelFormat::Rgb => mikan::PixelFormat::RGBResv8BitPerColor,
        PixelFormat::Bgr => mikan::PixelFormat::BGRResv8BitPerColor,
        _ => panic!(),
    };
    let fb_config = FrameBufferConfig {
        frame_buffer: fb_addr,
        horizontal_resolution: hori as u32,
        vertical_resolution: vert as u32,
        pixels_per_scan_line,
        pixel_format,
    };
    writeln!(st.stdout(), "{:?}", &fb_config);

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
        let mut misc_buf = [0u8; TMP_BUF_SIZE];
        let info: &mut FileInfo = f.get_info(&mut misc_buf).unwrap_success();
        let kernel_file_size: u64 = info.file_size();

        // Read kernel file into temporary space
        let kernel_tmp_p = boot_services.allocate_pool(MemoryType::LOADER_DATA, kernel_file_size as usize).unwrap_success();
        let mut kernel_tmp_buf = unsafe { core::slice::from_raw_parts_mut(kernel_tmp_p as *mut u8, kernel_file_size as usize) };
        f.read(&mut kernel_tmp_buf).unwrap_success();
        f.close();

        use elf_rs::*;
        let elf = Elf::from_bytes(&kernel_tmp_buf).unwrap();
        let mut kernel_start = u64::max_value();
        let mut kernel_end = u64::min_value();
        if let Elf::Elf64(ref e) = elf {
            // writeln!(st.stdout(), "{:?} header: {:?}", e, e.header()).unwrap();
            for p in e.program_header_iter() {
                writeln!(st.stdout(), "{:x?}", p).unwrap();
                let header = p.ph;
                if matches!(header.ph_type(), ProgramType::LOAD) {
                    let s = header.vaddr();
                    let len = header.memsz();
                    kernel_start = core::cmp::min(kernel_start, s);
                    kernel_end = core::cmp::max(kernel_end, s + len);
                } 
            }
        }
        // writeln!(st.stdout(), "start: {:x}, end: {:x}", kernel_start, kernel_end).unwrap();

        let load_len = kernel_end - kernel_start;
        let n_pages = (load_len as usize + 0xfff) / 0x1000;
        let kernel_p = boot_services
            .allocate_pages(
                // NOTE: The starting address should be aligned.
                // But is the kernel_start guaranteed to be aligned?
                AllocateType::Address(kernel_start as usize),
                MemoryType::LOADER_DATA,
                n_pages,
            )
            .unwrap_success();

        let mut kernel_buf = unsafe { core::slice::from_raw_parts_mut(kernel_p as *mut u8, load_len as usize) };

        // Zeros the kernel region
        for i in 0..load_len as usize {
            kernel_buf[i] = 0;
        }

        // Read kernel file into the memory
        if let Elf::Elf64(ref e) = elf {
            for p in e.program_header_iter() {
                let header = p.ph;
                if matches!(header.ph_type(), ProgramType::LOAD) {
                    let src = p.segment();
                    let dst_addr = header.vaddr();
                    let src_len = header.filesz();
                    assert_eq!(src.len(), src_len as usize);
                    let mut dst = unsafe { core::slice::from_raw_parts_mut(dst_addr as *mut u8, src_len as usize) };
                    for i in 0..src_len as usize{
                        dst[i] = src[i];
                    }
                }
            }
        }

        // Entry address should be found at +24
        let buf = unsafe { core::slice::from_raw_parts((kernel_tmp_p as u64 + 24) as *mut u8, 8)};
        let kernel_main_addr = LittleEndian::read_u64(&buf);
        writeln!(st.stdout(), "kernel_main address = {:x}", kernel_main_addr).unwrap();

        boot_services.free_pool(kernel_tmp_p);
        st.exit_boot_services(handle, &mut misc_buf).unwrap_success();

        let kernel_main = unsafe {
            let f: extern "efiapi" fn(&FrameBufferConfig) -> ! = core::mem::transmute(kernel_main_addr);
            f
        };
        kernel_main(&fb_config);
    }
    loop {}
}
