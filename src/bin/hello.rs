#![no_main]
#![no_std]

use sensing_nrf52 as _; // global logger + panicking-behavior + memory layout

use nrf52840_hal::{
    self as hal,
    gpio::{p0::Parts as P0Parts, Input, Level, Output, Pin, PullUp, PushPull},
    prelude::{InputPin, OutputPin},
    Timer,
};

use embedded_hal::{blocking::delay::DelayMs, blocking::delay::DelayUs};

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");
    let board = hal::pac::Peripherals::take().unwrap();
    let pins = P0Parts::new(board.P0);
    let mut timer = Timer::new(board.TIMER0);
    // why no degrade on leds for example? And degrade on buttons?
    let mut trig = pins.p0_03.into_push_pull_output(Level::High).degrade();
    let mut echo = pins.p0_04.into_pullup_input().degrade();
    let mut btn = pins.p0_11.into_pullup_input().degrade();

    // here should be if btn.is_low() { // set trigger, check what is sent from the echo}
    // if not unwrap :
    // mismatched types
    // expected `bool`, found enum `core::result::Result`
    // note: expected type `bool`
    // found enum `core::result::Result<bool, void::Void>`rustc(E0308)
    loop {
        if btn.is_low().unwrap() {
            timer.delay_ms(1000u32);
            trig.set_high().unwrap();
            timer.delay_ms(10u32);
            trig.set_low().unwrap();
            defmt::info!("button pushed");
            let mut time_a = timer.read();
            defmt::info!("{=u32}", time_a);
            //while echo.is_high().unwrap() {}
            let mut time_b = timer.read();
            defmt::info!("{=u32}", time_b - time_a);
        }
    }
    sensing_nrf52::exit()
}
