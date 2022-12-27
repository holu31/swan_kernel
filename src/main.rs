#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod drivers;
mod arch;

use core::panic::PanicInfo;
use bootloader_api::*;
use drivers::*;
use arch::*;

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
    x86::idt::init_idt();
    println!("Hello!");
    x86_64::instructions::interrupts::int3();
    loop {
        x86_64::instructions::hlt();
    }
}
