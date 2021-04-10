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

const CTRL: u32 = 0x0cf8;
const DATA: u32 = 0x0cfc;

pub fn write_ctrl(v: u32) {

}
pub fn read_data() -> u32 {
    unimplemented!()
}