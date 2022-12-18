#![no_std]
#![no_main]

mod tty;

use core::panic::PanicInfo;
use crate::tty::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

const CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};
bootloader_api::entry_point!(kernel_main, config = &CONFIG);

#[no_mangle]
fn kernel_main(bootinfo: &'static mut bootloader_api::BootInfo)-> !{
    print("Hello world!");
    loop {}
}