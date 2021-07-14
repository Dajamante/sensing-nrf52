#![no_main]
#![no_std]

use sensing_nrf52 as _; // global logger + panicking-behavior + memory layout

use defmt::unwrap;
use nrf52840_hal::{
    self as hal,
    gpio::{p0::Parts as P0Parts, Level},
    prelude::*,
    Timer,
};

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Entering main and doing config");

    // comment on the unwrap! macro
    let board = unwrap!(hal::pac::Peripherals::take());
    let pins = P0Parts::new(board.P0);
    let mut trig = pins.p0_03.into_push_pull_output(Level::Low).degrade();
    let echo = pins.p0_04.into_pulldown_input().degrade();

    let mut timer = Timer::new(board.TIMER0);
    let threshold = 100_000;
    'outer: loop {
        trig.set_high().unwrap();
        timer.delay_us(10u32);
        trig.set_low().ok();
        timer.start(1_000_000_u32);
        while echo.is_low().unwrap() {
            if timer.read() > threshold {
                defmt::warn!("Threshold exceeded, restart");
                continue 'outer;
            }
        }
        timer.start(1_000_000u32);
        while echo.is_high().unwrap() {}
        let length = timer.read();
        defmt::info!("{=u32} : Estimated distance", length / 58);
    }
}
