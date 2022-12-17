#![no_std]
#![no_main]

mod tty;

use core::panic::PanicInfo;
use crate::tty::print;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print("Hello world!");
    loop {}
}