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
    // why no degrade on leds for example? And degrade on buttons?
    let mut trig = pins.p0_03.into_push_pull_output(Level::Low).degrade();
    let mut echo = pins.p0_04.into_pulldown_input().degrade();

    let mut millis: u32 = 0;
    // the bigger loop
    loop {
        'small: loop {
            // one second timer
            timer_1.start(1000u32);

            // send wave every 100 ms
            if millis % 10 == 0 {
                trig.set_high().unwrap();
                timer_0.delay_us(10u32);
                trig.set_low().unwrap();
            }

            // if 1 second passed without signal we waited too long
            while echo.is_low().unwrap() {
                if millis % 1000 == 0 {
                    break 'small;
                }
            }

            // // wait as long as the echo is high
            // this is blocking the program!
            while echo.is_high().unwrap() {}

            defmt::info!("{=u32} : timer value", timer_1.read());

            millis += 1;
            defmt::info!("{=u32} passed millis", millis);

            // blocking wait with timer_0 to not debounce the button
            timer_0.delay_ms(100u32);
        }
    }
    //sensing_nrf52::exit()
}
