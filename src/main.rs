#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

fn print(s: &str){
    s.as_bytes()
        .iter()
        .flat_map(|bt| [*bt, 0x7 as u8])
        .enumerate()
        .for_each(|(i, byte)| unsafe {
            *VGA_BUFFER.offset(i as isize) = byte;
        });
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print("Hello world!");

    loop {}
}