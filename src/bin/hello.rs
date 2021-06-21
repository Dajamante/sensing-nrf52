#![no_main]
#![no_std]

use sensing_nrf52 as _; // global logger + panicking-behavior + memory layout

// this is for nop if I need it

use groundhog::RollingTimer;
use groundhog_nrf52::GlobalRollingTimer;
use nrf52840_hal::{
    self as hal,
    gpio::{p0::Parts as P0Parts, Level},
    prelude::*,
};

#[cortex_m_rt::entry]
fn main() -> ! {
    let board = hal::pac::Peripherals::take().unwrap();
    let pins = P0Parts::new(board.P0);
    GlobalRollingTimer::init(board.TIMER0);
    let timer = GlobalRollingTimer::new();
    let mut trig = pins.p0_03.into_push_pull_output(Level::Low).degrade();
    let echo = pins.p0_04.into_pullup_input().degrade();

    let threshold = 100;
    // the bigger loop
    'outer: loop {
        let start = timer.get_ticks();
        trig.set_high().unwrap();
        while timer.micros_since(start) < 10 {}
        trig.set_low().ok();

        while echo.is_low().unwrap() {
            if timer.millis_since(start) > threshold {
                defmt::warn!("We froze, restart");
                trig.set_high().unwrap();
                continue 'outer;
            }
        }
        let start_pulse: u32 = timer.get_ticks();

        while echo.is_high().unwrap() {}
        let length = timer.micros_since(start_pulse);

        defmt::info!("{=u32} : Estimated distance", length / 58);

        // Blocking wait to allow human reading
        while timer.millis_since(start) <= 100 {}
    }
    //sensing_nrf52::exit()
}
