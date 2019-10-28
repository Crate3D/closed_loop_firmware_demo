use panic_semihosting as _;

use stm32f1xx_hal as _;

use cortex_m::asm::wfi;
use rtfm::app;

#[app(device = stm32f1xx_hal::pac)]
const APP: () = {
    #[init]
    fn init() {}

    #[idle]
    fn idle() -> ! {
        loop {
            // Waits for interrupt
            wfi();
        }
    }
};
