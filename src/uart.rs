const UART_BASE: usize = 0x0900_0000; // Example base address for UART
const UART_DR: usize = UART_BASE + 0x00; // Data Register
const UART_FR: usize = UART_BASE + 0x18; // Flag Register
const UART_FR_TXFF: u32 = 1 << 5;

pub unsafe fn putc(c: u8) {
    let fr = UART_FR as *const u32;
    let dr = UART_DR as *mut u32;

    while core::ptr::read_volatile(fr) & UART_FR_TXFF != 0 {}
    core::ptr::write_volatile(dr, c as u32);
}

pub unsafe fn puts(s: &str) {
    for c in s.bytes() {
        if c == b'\n' {
            putc(b'\r');
        }
        putc(c);
    }
}
