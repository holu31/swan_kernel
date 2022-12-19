#![no_std]
#![no_main]

mod tty;

use core::panic::PanicInfo;
use bootloader_api::*;
use crate::tty::*;

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
fn kernel_main(bootinfo: &'static mut bootloader_api::BootInfo)-> !{
    let mut writer = tty::Writer::init();
    writer.write_string("Hello world!");
    loop {
        x86_64::instructions::hlt();
    }
}
