#![no_main]
#![no_std]

use sensing_nrf52 as _; // global logger + panicking-behavior + memory layout

// this is for nop if I need it

use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use nrf52840_hal::{
    self as hal,
    gpio::{p0::Parts as P0Parts, Level},
    prelude::*,
    Timer,
};

#[cortex_m_rt::entry]
fn main() -> ! {
    let board = hal::pac::Peripherals::take().unwrap();
    let pins = P0Parts::new(board.P0);
    let mut timer_0 = Timer::new(board.TIMER0);
    let mut timer_1 = Timer::new(board.TIMER1).into_periodic();
    let mut trig = pins.p0_03.into_push_pull_output(Level::Low).degrade();
    let mut echo = pins.p0_04.into_pulldown_input().degrade();

    // the bigger loop
    loop {
        'small: loop {
            timer_1.start(1000000u32);
            // send wave every 100 ms
            if timer_1.read() % 1000 == 0 {
                trig.set_high().unwrap();
                timer_0.delay_us(10u32);
                trig.set_low().unwrap();
                defmt::info!("Sent wave");
            }

            // if 1 second passed without signal we waited too long
            while echo.is_low().unwrap() {
                defmt::info!("Echo is low");
                if timer_1.read() % 1000 == 0 {
                    defmt::info!("No signal received, starting over from echo low.");
                    break 'small;
                }
            }

            let mut start: u32 = 0;
            // Wait while the echo is high,
            // same here, if 1 second has passed the wait is too long
            while echo.is_high().unwrap() {
                if timer_1.read() % 1000 == 0 {
                    defmt::info!("No signal received, starting over from echo high.");
                    break 'small;
                }
                start = timer_1.read();
                defmt::info!("Echo is high. {=u32} : timer value", timer_1.read());
            }
            let mut length = timer_1.read() - start;
            defmt::info!("{=u32} : value counter", length);

            defmt::info!("{=u32} : Estimated distance", length / 58);

            // Blocking wait to allow human reading
            timer_0.delay_ms(1000u32);
        }
    }
    //sensing_nrf52::exit()
}
