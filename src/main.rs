// Main firmware code
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::fmt::{self, Write};

#[allow(unused_imports)]
use aux::{entry, iprint, iprintln, usart1};

macro_rules! uprint {
    ($serial:expr, $($arg:tt)*) => {
        $serial.write_fmt(format_args!($($arg)*)).ok()
    };
}

macro_rules! uprintln {
    ($serial:expr, $fmt:expr) => {
        uprint!($serial, concat!($fmt, "\n"))
    };
    ($serial:expr, $fmt:expr, $($arg:tt)*) => {
        uprint!($serial, concat!($fmt, "\n"), $($arg)*)
    };
}

struct SerialPort {
    usart1: &'static mut usart1::RegisterBlock,
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            // wait until it's safe to write to TDR
            while self.usart1.isr.read().txe().bit_is_clear() {}

            self.usart1.tdr.write(|w| w.tdr().bits(u16::from(byte)));
        }
        Ok(())
    }
}

#[entry]
fn main() -> ! {
    let (usart1, mono_timer, mut itm) = aux::init();

    let mut serial = SerialPort { usart1 };
    
    uprintln!(serial, "Startup firmware complete");

    loop {}
}
