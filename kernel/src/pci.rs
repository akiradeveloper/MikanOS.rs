use bit_field::BitField;

pub struct ConfigAddress {
    pub reg: u8, // 0-255
    pub bus: u8, // 0-255
    pub device: u8, // 0-31
    pub function: u8, // 0-7
}
pub fn make_address(config: ConfigAddress) -> u32 {
    let mut res = 0;
    res.set_bits(0..=7, config.reg as u32);
    res.set_bits(8..=10, config.function as u32);
    res.set_bits(11..=15, config.device as u32);
    res.set_bits(16..=23, config.bus as u32);
    res.set_bits(24..=30, 0);
    res.set_bits(31..=31, 1);
    res
}

const CTRL: u16 = 0x0cf8;
const DATA: u16 = 0x0cfc;

use x86_64::instructions::port::Port;

fn read_data(config: ConfigAddress) -> u32 {
    let addr = make_address(config);
    let mut port_w = Port::new(CTRL);
    unsafe { port_w.write(addr) };
    let mut port_r = Port::new(DATA);
    unsafe { port_r.read() }
}
#[derive(Default)]
struct PciConfig {
    vender_id: u16,
    device_id: u16,
    command: u16,
    status: u16,
    revision_id: u8,
    interface: u8,
    sub_class: u8,
    base_class: u8,
    cacheline_size: u8,
    latency_timer: u8,
    header_type: u8,
    bist: u8,
    bar: [u32; 6],
}
impl PciConfig {
    pub fn new(bus: u8, device: u8, function: u8) -> Self {
        let r = PciConfig::default();
        let row = read_data(ConfigAddress { bus, device, function, reg: 0 });
        let row = read_data(ConfigAddress { bus, device, function, reg: 4 });
        let row = read_data(ConfigAddress { bus, device, function, reg: 8 });
        let row = read_data(ConfigAddress { bus, device, function, reg: 12 });
        for i in 0..6 {
            let row = read_data(ConfigAddress { bus, device, function, reg: 16 + 4*i });
            // TODO
        }
        r
    }
}