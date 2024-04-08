use stm32f1xx_hal::{pac, prelude::*};

pub const LED1_HIGH: u32 = 0x00000100;
pub const LED2_HIGH: u32 = LED1_HIGH << 1;
pub const LED3_HIGH: u32 = LED1_HIGH << 2;
pub const LED4_HIGH: u32 = LED1_HIGH << 3;

pub const LED5_HIGH: u32 = LED1_HIGH << 4;
pub const LED6_HIGH: u32 = LED1_HIGH << 5;
pub const LED7_HIGH: u32 = LED1_HIGH << 6;
pub const LED8_HIGH: u32 = LED1_HIGH << 7;

#[macro_export]
macro_rules! modify {
    ($p:expr, = $flag:expr) => {
        $p.write(|w| unsafe { w.bits($flag) })
    };
    ($p:expr, & $flag:expr) => {
        $p.modify(|r, w| unsafe { w.bits(r.bits() & $flag) })
    };
    ($p:expr, | $flag:expr) => {
        $p.modify(|r, w| unsafe { w.bits(r.bits() | $flag) })
    };
    ($p:expr, $flag:expr, $status:expr) => {
        $p.modify(|r, w| unsafe {
            match $status {
                // =
                0 => w.bits($flag),
                // |
                1 => w.bits(r.bits() | $flag),
                // &
                2 => w.bits(r.bits() & $flag),
                _ => w,
            }
        })
    };
}
pub fn led_init(dp: &pac::Peripherals) {
    modify!(dp.RCC.apb2enr, | 1);
    modify!(dp.RCC.apb2enr, | 1 << 3);
    modify!(dp.RCC.apb2enr, | 1 << 6);
    modify!(dp.AFIO.mapr, | 0x02000000);
    modify!(dp.GPIOB.crl, &0xFFF0FFFF);
    modify!(dp.GPIOB.crl, | 0x00030000);
    modify!(dp.GPIOB.odr, | 0x00000010);
    modify!(dp.GPIOE.crh, = 0);
    modify!(dp.GPIOE.crh, | 0x33333333);
    modify!(dp.GPIOE.odr, | 0x0000FF00);
}

pub fn exp1() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    led_init(&dp);
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut gpioc = dp.GPIOC.split();
    let mut pc13 = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    pc13.set_high();
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC;
    let clocks = rcc.constrain().cfgr.freeze(&mut flash.acr);
    let mut delay = cp.SYST.delay(&clocks);
    let gpio_e = dp.GPIOE;
    let odr = &gpio_e.odr;
    loop {
        for i in 0..8 {
            modify!(odr, = LED1_HIGH << i );
            delay.delay_ms(500u32)
        }

        for i in (1..7).rev() {
            modify!(odr, = LED1_HIGH << i );
            delay.delay_ms(500u32)
        }
    }
}
