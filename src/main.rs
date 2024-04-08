#![no_main]
#![no_std]

#[allow(unused)]
use stm32::{exp1::exp1, exp2::exp2, exp3::exp3};
use core::panic::PanicInfo;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();
    rtt_target::rprintln!("start");
    exp3()
    // app::empty_loop()
}

#[panic_handler]
fn panic_(_info: &PanicInfo) -> ! {
    loop {}
}
