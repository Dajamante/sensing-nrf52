#![no_std]
#![cfg_attr(test, no_main)]

use sensing_nrf52 as _; // memory layout + panic handler

#[defmt_test::tests]
mod tests {}
