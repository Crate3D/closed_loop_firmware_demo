// Main firmware code
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

use stm32f4xx_hal::{stm32, prelude::*, serial::{Serial, config}};
use serialio::{SerialIO, sprintln};
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let p = stm32::Peripherals::take().unwrap();

    let rcc = p.RCC.constrain();
    let gpioa = p.GPIOA.split();

    let clocks = rcc.cfgr.freeze();

    let tx = gpioa.pa9.into_alternate_af7();
    let rx = gpioa.pa10.into_alternate_af7();

    let conf = config::Config::default();
    let serial = Serial::usart1(p.USART1, (tx, rx), conf.baudrate(115_200.bps()), clocks);

    let (tx, rx) = serial.unwrap().split();

    let mut in_out = SerialIO::new(tx, rx);

    sprintln!(in_out, "Startup firmware complete");

    loop {}
}
