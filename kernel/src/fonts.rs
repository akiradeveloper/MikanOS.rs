use font_tbl::font_tbl;

pub fn ascii_map(x: char) -> &'static [u8; 16] {
    let ascii_code = x as u8;
    &font_tbl[ascii_code as usize]
}