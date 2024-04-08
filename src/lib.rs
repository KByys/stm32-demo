#![no_std]
pub mod exp1;
pub mod exp2;
pub mod exp3;
use stm32f1xx_hal::{pac, prelude::*};

#[macro_export]
macro_rules! modify_p {
    ($p:expr, $bit:expr) => {
        if $bit == 0 {
            $p.set_low();
        } else {
            $p.set_high();
        }
    };
}

pub fn empty_loop() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut gpioc = dp.GPIOC.split();
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut delay = cp.SYST.delay(&clocks);
    let mut pc13 = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    loop {
        pc13.toggle();
        delay.delay_ms(1000u32);
    }
}
