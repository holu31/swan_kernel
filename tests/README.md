```rust
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(swan_kernel::test_runner)]

use swan_kernel::*;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[test_case]
fn test_func(){
    ...
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[OK]");
    exit_qemu(QemuExitCode::Success);

    loop {
        x86_64::instructions::hlt();
    }
}
```