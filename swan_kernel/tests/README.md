```rust
#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(swan_kernel::test_runner)]

use swan_kernel::*;
use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(tmain);

fn tmain(boot_info: &'static BootInfo) -> ! {
    // code test
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    swan_kernel::test_panic_handler(info)
}
```