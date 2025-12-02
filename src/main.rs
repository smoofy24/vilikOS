#![no_std]
#![no_main]

mod uart;

use core::arch::global_asm;

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
    loop {
        unsafe {
            let mut buf: [u8; 64] = [0; 64];
            let len = uart::gets(&mut buf);
            uart::puts("You typed: ");
            uart::puts(core::str::from_utf8_unchecked(&buf[..len]));
        }
    }
}
