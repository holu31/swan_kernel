use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::drivers::*;

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn init_idt() {
    unsafe {
        IDT.breakpoint.set_handler_fn(breakpoint_handler);
        IDT.load();
    }
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame)
{
    tty::io::write_string("BREAKPOINT");
}