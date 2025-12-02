const UART_BASE: usize = 0x0900_0000; // Example base address for UART
const UART_DR: usize = UART_BASE + 0x00; // Data Register
const UART_FR: usize = UART_BASE + 0x18; // Flag Register

const UART_FR_RXFE: u32 = 1 << 4;
const UART_FR_TXFF: u32 = 1 << 5;

pub fn init() {
    // UART initialization code would go here
}

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

pub unsafe fn getc() -> u8 {
    let fr: *const u32 = UART_FR as *const u32;
    let dr: *mut u32 = UART_DR as *mut u32;

    while core::ptr::read_volatile(fr) & UART_FR_RXFE != 0 {}

    core::ptr::read_volatile(dr) as u8
}

pub unsafe fn gets(buf: &mut [u8]) -> usize {
    let mut i = 0;
    loop {
        let c = getc();

        if c == b'\r' || c == b'\n' {
            putc(b'\n');
            break;
        }

        if c == 0x7F || c == 0x08 {
            if i > 0 {
                i -= 1;
                puts("\x08 \x08");
            }
            continue;
        }

        if i < buf.len() - 1 {
            buf[i] = c;
            i += 1;
            putc(c);
        } else {
            putc(0x07);
        }
    }
    buf[i] = 0;
    i
}
