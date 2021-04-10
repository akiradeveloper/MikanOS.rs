use crate::graphics::PixelColor;
use crate::graphics;

const N_COLUMN: usize = 80;
const N_ROW: usize = 25;

pub struct Console {
    fg_color: PixelColor,
    bg_color: PixelColor,
    cur_row: usize,
    cur_column: usize,
    buf: [[char; N_COLUMN]; N_ROW],
}
impl Console {
    pub const fn new(fg_color: PixelColor, bg_color: PixelColor) -> Self {
        Self {
            fg_color,
            bg_color,
            cur_row: 0,
            cur_column: 0,
            buf: [[' '; N_COLUMN]; N_ROW],
        }
    }
    pub fn put_string(&mut self, s: &str) {
        for (i, c) in s.char_indices() {
            match c {
                '\n' => {
                    self.new_line();
                },
                c => {
                    if self.cur_column < N_COLUMN {
                        graphics::write_ascii(c, 8 * self.cur_column as u32, 16 * self.cur_row as u32, self.fg_color);
                        self.buf[self.cur_row][self.cur_column] = c;
                        self.cur_column += 1;
                    }
                }
            }
        }
    }
    fn new_line(&mut self) {
        if self.cur_row < N_ROW - 1 {
            self.cur_row += 1;
            self.cur_column = 0;
        } else {
            self.clear();
            for i in 1..N_ROW {
                for j in 0..N_COLUMN {
                    let c = self.buf[i][j];
                    graphics::write_ascii(c, 8 * j as u32, 16 * (i-1) as u32, self.fg_color);
                }
            }
            self.cur_column = 0;
        }
    }
    pub fn clear(&mut self) {
        for i in 0..N_ROW {
            for j in 0..N_COLUMN {
                graphics::write_pixel(8 * j as u32, 16 * i as u32, self.bg_color);
            }
        }
    }
}