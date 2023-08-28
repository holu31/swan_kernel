#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(swan_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};
use swan_kernel::task::{Task, executor::Executor};
use swan_kernel::task::keyboard;

use swan_kernel::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    swan_kernel::test_panic_handler(info)
}

entry_point!(kernel_main);

extern crate alloc;

#[no_mangle]
fn kernel_main(_boot_info: &'static BootInfo)-> !{

    swan_kernel::init(_boot_info);

    #[cfg(test)]
    test_main();

    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}