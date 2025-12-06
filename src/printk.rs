use crate::uart;

use core::fmt::Write;
use heapless::String;

pub fn _print(args: core::fmt::Arguments) {
    let mut buffer = String::<64>::new();

    if write!(&mut buffer, "{}", args).is_ok() {
        unsafe {
            uart::puts(&buffer);
        }
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::printk::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! printk {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*))
    };
}

