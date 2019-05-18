//! Initialization code

#![no_std]

extern crate panic_itm;

pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
pub use cortex_m_rt::entry;
pub use f3::hal::{prelude, serial::Serial, stm32f30x::usart1, time::MonoTimer};

use f3::hal::{
    prelude::*,
    stm32f30x::{self, USART1},
};

#[inline(never)]
pub fn init() -> (&'static mut usart1::RegisterBlock, MonoTimer, ITM) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioc = dp.GPIOC.split(&mut rcc.ahb);

    let tx = gpioc.pc4.into_af7(&mut gpioc.moder, &mut gpioc.afrl);
    let rx = gpioc.pc5.into_af7(&mut gpioc.moder, &mut gpioc.afrl);

    Serial::usart1(dp.USART1, (tx, rx), 115_200.bps(), clocks, &mut rcc.apb2);

    unsafe {
        (
            &mut *(USART1::ptr() as *mut _),
            MonoTimer::new(cp.DWT, clocks),
            cp.ITM,
        )
    }
}
