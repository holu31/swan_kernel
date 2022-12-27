#![no_std]
#![no_main]
#![feature(once_cell)]

mod drivers;

use core::panic::PanicInfo;
use bootloader_api::*;
use drivers::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

const CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};
entry_point!(kernel_main, config = &CONFIG);

#[no_mangle]
fn kernel_main(_bootinfo: &'static mut bootloader_api::BootInfo)-> !{
    tty::io::write_string("hello");
    loop {
        x86_64::instructions::hlt();
    }
}
