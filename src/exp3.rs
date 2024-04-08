use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::*;

use crate::__move;
use crate::exp2::APL_2;
use crate::exp2::APL_4;
use crate::exp2::APL_P;
use crate::exp2::APL_Q;
use crate::exp2::APL_Y;
use crate::modify;
use crate::show;

pub fn init(dp: &pac::Peripherals) {
    modify!(dp.RCC.apb2enr, | 1 << 4);
    modify!(dp.GPIOC.crl, &0xFFFF0000);
    modify!(dp.GPIOC.crl, | 0x00008888);
    modify!(dp.GPIOC.odr, | 0x0000000F);

    modify!(dp.GPIOB.crl, &0xFFFF0000);
    modify!(dp.GPIOB.crl, | 0x00003333);
    modify!(dp.GPIOB.odr, | 0x000000FF);
    modify!(dp.GPIOE.crh, = 0);
    modify!(dp.GPIOE.crh, | 0x33333333);
    modify!(dp.GPIOE.odr, | 0x0000FF00);
}

macro_rules! press {
    ($word:expr, $k:expr, $pc:expr, $down:stmt, $up:stmt) => {
        if !$k && $pc.is_low() {
            $k = true;
            rtt_target::rprintln!("{} 按下", $word);
            $down
        } else if $k && $pc.is_high() {
            $k = false;
            rtt_target::rprintln!("{} 释放", $word);
            $up
        }
    };
}
pub fn exp3() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    init(&dp);
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut delay = cp.SYST.delay(&clocks);
    let mut gpioc = dp.GPIOC.split();

    let k1 = gpioc.pc0.into_floating_input(&mut gpioc.crl);
    let mut k1_press = false;
    let k2 = gpioc.pc1.into_floating_input(&mut gpioc.crl);
    let mut k2_press = false;
    let k3 = gpioc.pc2.into_floating_input(&mut gpioc.crl);
    let mut k3_press = false;
    let k4 = gpioc.pc3.into_floating_input(&mut gpioc.crl);
    let mut k4_press = false;
    let mut gpiob = dp.GPIOB.split();
    let mut pb1 = gpiob.pb1.into_push_pull_output(&mut gpiob.crl);
    let mut pb2 = gpiob.pb2.into_push_pull_output(&mut gpiob.crl);
    let mut pb0 = gpiob.pb0.into_push_pull_output(&mut gpiob.crl);
    let mut gpioe = dp.GPIOE.split();
    let mut pe8 = gpioe.pe8.into_push_pull_output(&mut gpioe.crh);
    let mut pe9 = gpioe.pe9.into_push_pull_output(&mut gpioe.crh);
    let mut pe10 = gpioe.pe10.into_push_pull_output(&mut gpioe.crh);
    let mut pe11 = gpioe.pe11.into_push_pull_output(&mut gpioe.crh);
    let mut pe12 = gpioe.pe12.into_push_pull_output(&mut gpioe.crh);
    let mut pe13 = gpioe.pe13.into_push_pull_output(&mut gpioe.crh);
    let mut pe14 = gpioe.pe14.into_push_pull_output(&mut gpioe.crh);
    let mut key_down_up = 0;
    let mut speed = 200;
    let mut speed_index = 0;
    let mut pos = 0;
    let mut direct = 0;

    loop {
        while speed_index <= speed {
            __move!(pos, pb0, pb1, pb2);

            if key_down_up == 0 {
                show!(APL_P, pe8, pe9, pe10, pe11, pe12, pe13, pe14);
            } else if key_down_up == 2 {
                show!(APL_2, pe8, pe9, pe10, pe11, pe12, pe13, pe14);
            } else if key_down_up == 4 {
                show!(APL_4, pe8, pe9, pe10, pe11, pe12, pe13, pe14);
            }
            delay.delay_ms(1u32);
            if key_down_up == 0 {
                show!(APL_Q, pe8, pe9, pe10, pe11, pe12, pe13, pe14);
            } else if key_down_up == 2 {
                show!(APL_2, pe8, pe9, pe10, pe11, pe12, pe13, pe14);
            } else if key_down_up == 4 {
                show!(APL_4, pe8, pe9, pe10, pe11, pe12, pe13, pe14);
            }
            __move!(pos + 1, pb0, pb1, pb2);
            delay.delay_ms(1u32);

            if key_down_up == 0 {
                show!(APL_Y, pe8, pe9, pe10, pe11, pe12, pe13, pe14);
            } else if key_down_up == 2 {
                show!(APL_2, pe8, pe9, pe10, pe11, pe12, pe13, pe14);
            } else if key_down_up == 4 {
                show!(APL_4, pe8, pe9, pe10, pe11, pe12, pe13, pe14);
            }
            __move!(pos + 2, pb0, pb1, pb2);
            delay.delay_ms(1u32);
            press!("k2", k2_press, k2, key_down_up = 2, key_down_up = 0);
            press!("k4", k4_press, k4, key_down_up = 4, key_down_up = 0);
            press!("k1", k1_press, k1, speed = 50, speed = 200);
            press!("k3", k3_press, k3, speed = 400, speed = 200);
            speed_index += 1;
        }
        speed_index = 0;
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
