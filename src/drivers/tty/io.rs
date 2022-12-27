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

static mut BUFFER: *mut [[(u8, Color); 80]; 25] = 0xB8000 as *mut _;
pub static mut COLOR: Color = Color::LightGray;
static mut CURSOR: (usize, usize) = (0, 0);

pub fn cursor_xy(x: usize, y: usize){
    unsafe { 
        CURSOR = (x, y);
    }
}

pub fn cursor_move(offset: usize){
    unsafe { 
        CURSOR.0 += offset;
    }
}

pub fn write_byte(byte: u8){
    unsafe {
        if byte == 0 || CURSOR.1 > 25 {
            return;
        } else if byte == b'\n' || CURSOR.0 == 80 {
            CURSOR.0 = 0;
            CURSOR.1 += 1;
            return;
        }
        (&mut *BUFFER)[CURSOR.1][CURSOR.0] = (byte, COLOR.clone());
        cursor_move(1);
    }
}

pub fn write_string(string: &str){
    string.bytes()
        .for_each(|i| write_byte(i));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::drivers::tty::io::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

use core::fmt;
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    let str = args.as_str().unwrap();
    write_string(str);
}