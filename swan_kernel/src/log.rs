use core::fmt::Arguments;
use crate::arch::x86_64::video::{WRITER, Color};
use crate::{println, serial_println};

#[macro_export]
macro_rules! ok {
    ($($arg:tt)*) => (
        $crate::log::log("[OK]", $crate::arch::x86_64::video::Color::Green, format_args!($($arg)*))
    );
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => (
        $crate::log::log("[INFO]", $crate::arch::x86_64::video::Color::Yellow, format_args!($($arg)*))
    );
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => (
        $crate::log::log("[ERROR]", $crate::arch::x86_64::video::Color::Red, format_args!($($arg)*))
    );
}

pub fn log(prefix: &str, prefix_color: Color, s: Arguments<'_>) {
    for character in prefix.as_bytes() {
        WRITER.lock().write_custom_byte(*character,
            prefix_color);
    }
    println!(" {}", s);
    serial_println!("{} {}", prefix, s);
}