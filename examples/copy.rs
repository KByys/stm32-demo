#![no_std]
#![no_main]

use core::{panic::PanicInfo, time::Duration};
use cortex_m_rt::entry;
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::{
    prelude::{
        _stm32_hal_flash_FlashExt, _stm32_hal_gpio_GpioExt, _stm32_hal_rcc_RccExt,
        _stm32f4xx_hal_timer_SysCounterExt,
    },
    stm32 as device,
    time::MicroSeconds,
};
#[entry]
fn entry() -> ! {
    let dp = device::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut gpio_c = dp.GPIOC.split();

    let mut pc13 = gpio_c.pc13.into_push_pull_output(&mut gpio_c.crh);
    pc13.set_high(); // Ensure LED is off initially
    let mut pc1 = gpio_c.pc1.into_push_pull_output(&mut gpio_c.crl);
    pc1.set_low();
    let mut delay = cp.SYST.delay(&clocks);
    loop {
        pc13.set_low();
        pc1.toggle();
        delay.delay_ms(4000u32);
        pc13.set_high();
        delay.delay_ms(1000u32);
    }
}
#[panic_handler]
fn panic_(_info: &PanicInfo) -> ! {
    let mut gpio_c = device::Peripherals::take().unwrap().GPIOC.split();
    let mut pc13 = gpio_c.pc13.into_push_pull_output(&mut gpio_c.crh);
    loop {
        pc13.toggle(); // Toggle LED on panic
        cortex_m::asm::delay(500_000); // Delay for visibility
    }
}
