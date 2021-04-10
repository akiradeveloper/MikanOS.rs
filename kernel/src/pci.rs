use bit_field::BitField;

pub type Result<T> = core::result::Result<T, ()>;

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
#[derive(Default, Clone, Copy)]
pub struct PciConfig {
    pub vender_id: u16,
    pub device_id: u16,
    pub command: u16,
    pub status: u16,
    pub revision_id: u8,
    pub interface: u8,
    pub sub_class: u8,
    pub base_class: u8,
    pub cacheline_size: u8,
    pub latency_timer: u8,
    pub header_type: u8,
    pub bist: u8,
    pub bar: [u32; 6],
}
impl PciConfig {
    pub fn read(bus: u8, device: u8, function: u8) -> Self {
        let mut r = PciConfig::default();
        let row = read_data(ConfigAddress { bus, device, function, reg: 0 });
        r.vender_id = row.get_bits(0..16) as u16;
        r.device_id = row.get_bits(16..32) as u16;
        let row = read_data(ConfigAddress { bus, device, function, reg: 4 });
        r.command = row.get_bits(0..16) as u16;
        r.status = row.get_bits(16..32) as u16;
        let row = read_data(ConfigAddress { bus, device, function, reg: 8 });
        r.revision_id = row.get_bits(0..8) as u8;
        r.interface = row.get_bits(8..16) as u8;
        r.sub_class = row.get_bits(16..24) as u8;
        r.base_class = row.get_bits(24..32) as u8;
        let row = read_data(ConfigAddress { bus, device, function, reg: 12 });
        r.cacheline_size = row.get_bits(0..8) as u8;
        r.latency_timer = row.get_bits(8..16) as u8;
        r.header_type = row.get_bits(16..24) as u8;
        r.bist = row.get_bits(24..32) as u8;
        for i in 0..6 {
            let row = read_data(ConfigAddress { bus, device, function, reg: 16 + 4*i });
            r.bar[i as usize] = row;
        }
        r
    }
}
#[derive(Default, Clone, Copy)]
pub struct PciDevice {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub config: PciConfig,
}

pub struct ScanPciDevices {
    pub num_devices: usize,
    pub result: [PciDevice; 32],
}
impl ScanPciDevices {
    pub fn new() -> Self {
        Self {
            num_devices: 0,
            result: [PciDevice::default(); 32],
        }
    }
    pub fn scan_devices(&mut self) -> Result<()> {
        let config = PciConfig::read(0, 0, 0);
        if !config.header_type.get_bit(7) {
            return self.scan_bus(0);
        }
        for function in 1..8 {
            let config = PciConfig::read(0, 0, function);
            if config.vender_id == 0xffff {
                continue;
            }
            self.scan_bus(function)?;
        }
        Ok(())
    }
    fn scan_bus(&mut self, bus: u8) -> Result<()> {
        for device in 0..32 {
            let config = PciConfig::read(bus, device, 0);
            if config.vender_id == 0xffff {
                continue;
            }
            self.scan_device(bus, device)?;
        }
        Ok(())
    }
    fn scan_device(&mut self, bus: u8, device: u8) -> Result<()> {
        self.scan_function(bus, device, 0)?;
        let config = PciConfig::read(bus, device, 0);
        if !config.header_type.get_bit(7) {
            return Ok(())
        }
        for function in 1..8 {
            let config = PciConfig::read(bus, device, function);
            if config.vender_id == 0xffff {
                continue;
            }
            self.scan_function(bus, device, function)?;
        }
        Ok(())
    }
    fn scan_function(&mut self, bus: u8, device: u8, function: u8) -> Result<()> {
        let config = PciConfig::read(bus, device, function);
        let device = PciDevice { bus, device, function, config };
        self.result[self.num_devices] = device;
        self.num_devices += 1;

        // PCI-PCI bridge
        let base_class = config.base_class;
        let sub_class = config.sub_class;
        if base_class == 0x06 && sub_class == 0x04 {
            let bus_numbers = config.bar[2];
            let secondary_bus = (bus_numbers >> 8) & 0xff;
            return self.scan_bus(secondary_bus as u8);
        }

        Ok(())
    }
}