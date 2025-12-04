#![no_std]
#![no_main]
use heapless::String;

mod uart;
mod printk;

use core::arch::global_asm;
use crate::printk::test_printk;

// Include boot.S
global_asm!(include_str!("../boot/boot.S"));

// Panic handler
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Entry point from boot.S
#[no_mangle]
pub extern "C" fn kernel_main() {
    test_printk();

    let mut buf = String::<64>::new();
    loop {
        unsafe {
            uart::gets(&mut buf);
            printk!("You typed: {}, length is: {}", buf, buf.len());
        }
    }
}
