use crate::modify;
use stm32f1xx_hal::{
    pac::{self, Peripherals},
    prelude::*,
};

pub static APL_0: u8 = 0b0000000;
pub static APL_2: u8 = 0b1011011;
pub static APL_4: u8 = 0b1100110;

pub static APL_P: u8 = 0b1110011;
pub static APL_Q: u8 = 0b1100111;
pub static APL_Y: u8 = 0b1101110;


fn init(dp: &Peripherals) {
    modify!(dp.RCC.apb2enr, | 1 << 3);
    modify!(dp.RCC.apb2enr, | 1 << 6);

    modify!(dp.GPIOB.crl, &0xFFFF0000);
    modify!(dp.GPIOB.crl, | 0x00003333);
    modify!(dp.GPIOB.odr, | 0x000000FF);
    modify!(dp.GPIOE.crh, = 0);
    modify!(dp.GPIOE.crh, | 0x33333333);
    modify!(dp.GPIOE.odr, | 0x0000FF00);
}

#[macro_export]
macro_rules! __move {
    ($pos:expr, $p0:expr, $p1:expr, $p2:expr) => {
        $crate::modify_p!($p0, $pos & 0b0001);
        $crate::modify_p!($p1, $pos & 0b0010);
        $crate::modify_p!($p2, $pos & 0b0100);
    };
}
#[macro_export]
macro_rules! show {
    ($show:expr, $pe8:expr, $pe9:expr, $pe10:expr, $pe11:expr, $pe12:expr, $pe13:expr, $pe14:expr) => {{
        $crate::modify_p!($pe8, $show & 1);
        $crate::modify_p!($pe9, $show & 2);
        $crate::modify_p!($pe10, $show & 4);
        $crate::modify_p!($pe11, $show & 8);
        $crate::modify_p!($pe12, $show & 16);
        $crate::modify_p!($pe13, $show & 32);
        $crate::modify_p!($pe14, $show & 64);
    }};
}

pub fn exp2() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    init(&dp);
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut delay = cp.SYST.delay(&clocks);
    let mut gpiob = dp.GPIOB.split();

    let mut gpioe = dp.GPIOE.split();
    let mut pe8 = gpioe.pe8.into_push_pull_output(&mut gpioe.crh);
    let mut pe9 = gpioe.pe9.into_push_pull_output(&mut gpioe.crh);
    let mut pe10 = gpioe.pe10.into_push_pull_output(&mut gpioe.crh);
    let mut pe11 = gpioe.pe11.into_push_pull_output(&mut gpioe.crh);
    let mut pe12 = gpioe.pe12.into_push_pull_output(&mut gpioe.crh);
    let mut pe13 = gpioe.pe13.into_push_pull_output(&mut gpioe.crh);
    let mut pe14 = gpioe.pe14.into_push_pull_output(&mut gpioe.crh);
    let mut pb1 = gpiob.pb1.into_push_pull_output(&mut gpiob.crl);
    let mut pb2 = gpiob.pb2.into_push_pull_output(&mut gpiob.crl);
    let mut pb0 = gpiob.pb0.into_push_pull_output(&mut gpiob.crl);
    let mut pos = 0;
    let mut direct = 0;
    loop {
        for _ in 0..100 {
            __move!(pos, pb0, pb1, pb2);
            show!(APL_P, pe8, pe9, pe10, pe11, pe12, pe13, pe14);
            delay.delay_ms(1u32);
            show!(APL_Q, pe8, pe9, pe10, pe11, pe12, pe13, pe14);
            __move!(pos + 1, pb0, pb1, pb2);
            delay.delay_ms(1u32);
            show!(APL_Y, pe8, pe9, pe10, pe11, pe12, pe13, pe14);
            __move!(pos + 2, pb0, pb1, pb2);
            delay.delay_ms(1u32);
        }
        // 右移动
        if direct == 0 {
            if pos == 5 {
                direct = 1;
                pos -= 1;
            } else {
                pos += 1;
            }
        } else if pos == 0 {
            direct = 0;
            pos += 1;
        } else {
            pos -= 1;
        }
    }
}
