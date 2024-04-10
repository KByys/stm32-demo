use core::cell::RefCell;
use cortex_m::interrupt::{free, Mutex};

use stm32f1xx_hal::{
    gpio::{ExtiPin, Output, Pin, PushPull, CRH, CRL},
    pac::{self, interrupt},
    prelude::*,
    timer::SysDelay,
};
static DELAY: Mutex<RefCell<Option<SysDelay>>> = Mutex::new(RefCell::new(None));

type GpioPin<const C: char, const N: u8, E> =
    Mutex<RefCell<Option<Pin<Output<PushPull>, E, C, N>>>>;

macro_rules! gen_gpio {
    ($(($ident:ident, $ch:expr, $n:expr, $e:ty)), +) => {
        $( static $ident: GpioPin<$ch, $n, $e> = Mutex::new(RefCell::new(None));)+
    };
}
macro_rules! free {
    ($pe:expr, $show:expr, $k:expr) => {
        free(|cs| {
            if let Some(val) = $pe.borrow(cs).borrow_mut().as_mut() {
                $crate::modify_p!(val, $show & $k);
            }
        });
    };
}
macro_rules! show {
    ($show:expr) => {{
        free!(PE8, $show, 1);
        free!(PE9, $show, 2);
        free!(PE10, $show, 4);
        free!(PE11, $show, 8);
        free!(PE12, $show, 16);
        free!(PE13, $show, 32);
        free!(PE14, $show, 64);
    }};
}
gen_gpio! {
    (PB0, 'B', 0, CRL),
    (PB1, 'B', 1, CRL),
    (PB2, 'B', 2, CRL),
    (PE8, 'E', 8, CRH),
    (PE9, 'E', 9, CRH),
    (PE10, 'E', 10, CRH),
    (PE11, 'E', 11, CRH),
    (PE12, 'E', 12, CRH),
    (PE13, 'E', 13, CRH),
    (PE14, 'E', 14, CRH)
}

macro_rules! gpio_init {
    ($(($w:expr, $v:expr)), +) => {
        $(
            free(|cs| *$w.borrow(cs).borrow_mut() = Some($v) );
        )+
    };
}

use crate::exp2::{APL_1, APL_2, APL_3, APL_4, APL_P, APL_Q, APL_Y};

macro_rules! __move {
    ($pos:expr) => {{
        free(|cs| {
            if let Some(val) = PB0.borrow(cs).borrow_mut().as_mut() {
                $crate::modify_p!(val, $pos & 1);
            }

            if let Some(val) = PB1.borrow(cs).borrow_mut().as_mut() {
                $crate::modify_p!(val, $pos & 2);
            }
            if let Some(val) = PB2.borrow(cs).borrow_mut().as_mut() {
                $crate::modify_p!(val, $pos & 4);
            }
        });
    }};
}

macro_rules! move_and_show {
    ($speed:expr, $time:expr, $arr:expr) => {{
        let mut direct = 0;
        let mut pos = 0;
        for _ in 0..$time {
            for _ in 0..$speed {
                __move!(pos);
                show!($arr);
                delay_ms(1);

                __move!(pos + 1);
                show!($arr);
                delay_ms(1);

                __move!(pos + 2);
                show!($arr);
                delay_ms(1);
            }
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
    }};
}

pub fn exp4() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    // unsafe { cortex_m::interrupt::enable() };
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let mut gpioc = dp.GPIOC.split();
    let mut afio = dp.AFIO.constrain();
    let mut k1 = gpioc.pc0.into_pull_up_input(&mut gpioc.crl);
    k1.make_interrupt_source(&mut afio);
    k1.trigger_on_edge(&dp.EXTI, stm32f1xx_hal::gpio::Edge::Rising);
    k1.enable_interrupt(&dp.EXTI);

    let mut k2 = gpioc.pc1.into_pull_up_input(&mut gpioc.crl);
    k2.make_interrupt_source(&mut afio);
    k2.trigger_on_edge(&dp.EXTI, stm32f1xx_hal::gpio::Edge::Rising);
    k2.enable_interrupt(&dp.EXTI);

    let mut k3 = gpioc.pc2.into_pull_up_input(&mut gpioc.crl);
    k3.make_interrupt_source(&mut afio);
    k3.trigger_on_edge(&dp.EXTI, stm32f1xx_hal::gpio::Edge::Rising);
    k3.enable_interrupt(&dp.EXTI);

    let mut k4 = gpioc.pc3.into_pull_up_input(&mut gpioc.crl);
    k4.make_interrupt_source(&mut afio);
    k4.trigger_on_edge(&dp.EXTI, stm32f1xx_hal::gpio::Edge::Rising);
    k4.enable_interrupt(&dp.EXTI);

    unsafe {
        pac::NVIC::unmask(interrupt::EXTI0);
        pac::NVIC::unmask(interrupt::EXTI1);
        pac::NVIC::unmask(interrupt::EXTI2);
        pac::NVIC::unmask(interrupt::EXTI3);

        cp.NVIC.set_priority(interrupt::EXTI0, 0);
        cp.NVIC.set_priority(interrupt::EXTI1, 32);
        cp.NVIC.set_priority(interrupt::EXTI2, 64);
        cp.NVIC.set_priority(interrupt::EXTI3, 128);
    }

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut pos = 0;
    let mut direct = 0;

    let mut gpiob = dp.GPIOB.split();
    let pb0 = gpiob.pb0.into_push_pull_output(&mut gpiob.crl);
    let pb1 = gpiob.pb1.into_push_pull_output(&mut gpiob.crl);
    let pb2 = gpiob.pb2.into_push_pull_output(&mut gpiob.crl);

    let mut gpioe = dp.GPIOE.split();
    let pe8 = gpioe.pe8.into_push_pull_output(&mut gpioe.crh);
    let pe9 = gpioe.pe9.into_push_pull_output(&mut gpioe.crh);
    let pe10 = gpioe.pe10.into_push_pull_output(&mut gpioe.crh);
    let pe11 = gpioe.pe11.into_push_pull_output(&mut gpioe.crh);
    let pe12 = gpioe.pe12.into_push_pull_output(&mut gpioe.crh);
    let pe13 = gpioe.pe13.into_push_pull_output(&mut gpioe.crh);
    let pe14 = gpioe.pe14.into_push_pull_output(&mut gpioe.crh);
    gpio_init!(
        (DELAY, cp.SYST.delay(&clocks)),
        (PE8, pe8),
        (PE9, pe9),
        (PE10, pe10),
        (PE11, pe11),
        (PE12, pe12),
        (PE13, pe13),
        (PE14, pe14),
        (PB0, pb0),
        (PB1, pb1),
        (PB2, pb2)
    );
    loop {
        for _ in 0..200 {
            __move!(pos);
            show!(APL_P);
            delay_ms(1);

            __move!(pos + 1);
            show!(APL_Q);
            delay_ms(1);

            __move!(pos + 2);
            show!(APL_Y);
            delay_ms(1);
        }
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

fn delay_ms(ms: u32) {
    free(|cs| {
        if let Some(delay) = DELAY.borrow(cs).borrow_mut().as_mut() {
            delay.delay_ms(ms);
        }
    });
}

#[allow(non_snake_case)]
#[interrupt]
fn EXTI0() {
    rtt_target::rprintln!("EXTI0 开始");
    move_and_show!(50, 5, APL_1);
    unsafe { (*pac::EXTI::ptr()).pr.write(|w| w.pr0().set_bit()) }
    rtt_target::rprintln!("EXTI0 结束");
    delay_ms(10);
}

#[allow(non_snake_case)]
#[interrupt]
fn EXTI1() {
    rtt_target::rprintln!("EXTI1 开始");
    move_and_show!(100, 10, APL_2);
    rtt_target::rprintln!("EXTI1 结束");
    unsafe { (*pac::EXTI::ptr()).pr.write(|w| w.pr1().set_bit()) }
    delay_ms(100)
}

#[allow(non_snake_case)]
#[interrupt]
fn EXTI2() {
    rtt_target::rprintln!("EXTI2 开始");
    move_and_show!(150, 15, APL_3);
    rtt_target::rprintln!("EXTI2 结束");
    unsafe { (*pac::EXTI::ptr()).pr.write(|w| w.pr2().set_bit()) }
    delay_ms(100)
}
#[allow(non_snake_case)]
#[interrupt]
fn EXTI3() {
    rtt_target::rprintln!("EXTI3 开始");
    move_and_show!(200, 20, APL_4);
    rtt_target::rprintln!("EXTI3 结束");
    unsafe { (*pac::EXTI::ptr()).pr.write(|w| w.pr3().set_bit()) }
    delay_ms(100)
}
