const BUF: *mut [[(u8, Color); 80]; 25] = 0xB8000 as *mut _;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Color {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,
    DarkGray
}

pub use Color::*;

static mut CURSOR: (usize, usize) = (0, 0);

static mut COLOR: Color = LightGray;

pub fn set_color(color: Color) {
    unsafe {
        COLOR = color;
    }
}

pub fn putc(c: u8) {
    unsafe {
        if c == 0 || CURSOR.1 > 25 {
            return;
        } else if c == b'\n' || CURSOR.0 == 80 {
            CURSOR.0 = 0;
            CURSOR.1 += 1;
            return;
        }
        (&mut *BUF)[CURSOR.1][CURSOR.0] = (c, COLOR.clone());
        CURSOR.0 += 1;
    }
}

pub fn puts(s: &str) {
    s.bytes()
    .for_each(|i| putc(i));
}