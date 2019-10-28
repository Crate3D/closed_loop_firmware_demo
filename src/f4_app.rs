//! STM32F4 specific application

use panic_semihosting as _;

use serialio::{sprintln, SerialIO};
use stm32f4xx_hal::{
    prelude::*,
    serial::{config, Serial},
    stm32,
};

use cortex_m::asm::wfi;
use rtfm::app;

#[app(device = stm32f4xx_hal::stm32)]
const APP: () = {
    #[init]
    fn init() {
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
    }

    #[idle]
    fn idle() -> ! {
        loop {
            // Waits for interrupt
            wfi();
        }
    }
};
